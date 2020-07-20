// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

mod common;

use actor::{
    init::{ExecParams, Method as INIT_METHOD},
    miner::Method as MinerMethod,
    power::{
        Claim, CreateMinerParams, CreateMinerReturn, CurrentTotalPowerReturn,
        EnrollCronEventParams, Method, State, UpdateClaimedPowerParams, CONSENSUS_MINER_MIN_POWER,
    },
    reward::Method as RewardMethod,
    //multisig::{},
    ACCOUNT_ACTOR_CODE_ID,
    CRON_ACTOR_ADDR,
    CRON_ACTOR_CODE_ID,
    INIT_ACTOR_ADDR,
    MINER_ACTOR_CODE_ID,
    MULTISIG_ACTOR_CODE_ID,
    PAYCH_ACTOR_CODE_ID,
    POWER_ACTOR_CODE_ID,
    REWARD_ACTOR_ADDR,
    STORAGE_POWER_ACTOR_ADDR,
    SYSTEM_ACTOR_ADDR,
    SYSTEM_ACTOR_CODE_ID,
};
use address::Address;
use cid::Cid;
use common::*;
use db::MemoryDB;
use fil_types::{RegisteredSealProof, StoragePower};
//use filecoin_proofs_api::{ RegisteredSealProof};
use ipld_blockstore::BlockStore;
use ipld_hamt::BytesKey;
use ipld_hamt::Hamt;
use message::UnsignedMessage;
use num_bigint::{bigint_ser::BigIntSer, biguint_ser::BigUintSer, BigInt, BigUint};
// use serde::Serialize;
use vm::{ActorError, ExitCode, Serialized, TokenAmount, METHOD_CONSTRUCTOR, METHOD_SEND};

// Need to be super careful with this func. Change the type you are trafersing. Probably inlcude a type paramteter so you cna easily chekc what you are ocunting
fn verify_map_size<BS: BlockStore>(rt: &mut MockRuntime<'_, BS>, cid: Cid, expected: u64) {
    let map: Hamt<BytesKey, _> = Hamt::load(&cid, rt.store).unwrap();
    let mut count = 0;
    map.for_each(|_, _: Claim| {
        count += 1;
        Ok(())
    })
    .unwrap();
    assert_eq!(count, expected);
}

fn verify_empty_map<BS: BlockStore>(rt: &mut MockRuntime<'_, BS>, cid: Cid) {
    verify_map_size(rt, cid, 0)
}

fn construct_and_verify<BS: BlockStore>(rt: &mut MockRuntime<'_, BS>) {
    rt.expect_validate_caller_addr(&[SYSTEM_ACTOR_ADDR.clone()]);
    assert!(rt
        .call(
            &*POWER_ACTOR_CODE_ID,
            METHOD_CONSTRUCTOR,
            &Serialized::default()
        )
        .is_ok());
    rt.verify();
    let st: State = rt.get_state().unwrap();
    assert_eq!(StoragePower::from(0u8), st.total_raw_byte_power);
    assert_eq!(StoragePower::from(0u8), st.total_quality_adj_power);
    assert_eq!(0, st.miner_count);
    assert_eq!(0, st.num_miners_meeting_min_power);
    verify_empty_map(rt, st.claims);
    verify_empty_map(rt, st.cron_event_queue);
}

fn init_create_miner_bytes(
    owner: Address,
    worker: Address,
    peer: Vec<u8>,
    multi_addrs: Vec<u8>,
    seal_proof_type: RegisteredSealProof,
) -> Serialized {
    let params = CreateMinerParams {
        owner_addr: owner.clone(),
        worker_addr: worker,
        seal_proof_type: seal_proof_type,
        peer_id: peer,
        multi_addrs: multi_addrs,
    };

    Serialized::serialize(params).unwrap()
}

fn create_miner<BS: BlockStore>(
    rt: &mut MockRuntime<'_, BS>,
    owner: Address,
    worker: Address,
    miner: Address,
    robust: Address,
    peer: Vec<u8>,
    multi_addrs: Vec<u8>,
    seal_proof_type: RegisteredSealProof,
    value: TokenAmount,
) {
    rt.set_caller(ACCOUNT_ACTOR_CODE_ID.clone(), owner.clone());
    rt.balance = value.clone();
    rt.set_recieved(value.clone());

    rt.expect_validate_caller_type(&[
        ACCOUNT_ACTOR_CODE_ID.clone(),
        MULTISIG_ACTOR_CODE_ID.clone(),
    ]);

    let msg_params = ExecParams {
        code_cid: MINER_ACTOR_CODE_ID.clone(),
        constructor_params: init_create_miner_bytes(
            owner.clone(),
            worker.clone(),
            peer.clone(),
            multi_addrs.clone(),
            seal_proof_type,
        ),
    };

    let create_miner_ret = CreateMinerReturn {
        id_address: miner.clone(),
        robust_address: robust.clone(),
    };

    rt.expect_send(
        INIT_ACTOR_ADDR.clone(),
        INIT_METHOD::Exec as u64,
        Serialized::serialize(msg_params).unwrap(),
        value.clone(),
        Serialized::serialize(create_miner_ret).unwrap(),
        ExitCode::Ok,
    );

    let create_miner_params = CreateMinerParams {
        owner_addr: owner.clone(),
        worker_addr: worker.clone(),
        seal_proof_type: seal_proof_type,
        peer_id: peer.clone(),
        multi_addrs: multi_addrs.clone(),
    };

    assert!(rt
        .call(
            &*POWER_ACTOR_CODE_ID,
            Method::CreateMiner as u64,
            &Serialized::serialize(create_miner_params).unwrap()
        )
        .is_ok());
    rt.verify();
}

fn create_miner_basic<BS: BlockStore>(
    rt: &mut MockRuntime<'_, BS>,
    miner_seq: &mut u64,
    owner: Address,
    worker: Address,
    miner: Address,
) {
    let label = miner_seq.to_string();
    let actor_addr = Address::new_actor(label.as_bytes());
    *miner_seq = *miner_seq + 1;
    create_miner(
        rt,
        owner.clone(),
        worker.clone(),
        miner.clone(),
        actor_addr,
        label.as_bytes().to_vec(),
        vec![],
        RegisteredSealProof::StackedDRG2KiBV1,
        TokenAmount::from(0u8),
    );
}

#[allow(dead_code)]
fn update_claimed_power<BS: BlockStore>(
    rt: &mut MockRuntime<'_, BS>,
    miner: Address,
    raw_delta: StoragePower,
    qa_delta: StoragePower,
) {
    let params = UpdateClaimedPowerParams {
        raw_byte_power: raw_delta,
        quality_adj_delta: qa_delta,
    };

    rt.set_caller(MINER_ACTOR_CODE_ID.clone(), miner.clone());
    rt.expect_validate_caller_type(&[MINER_ACTOR_CODE_ID.clone()]);
    assert!(rt
        .call(
            &*POWER_ACTOR_CODE_ID,
            Method::UpdateClaimedPower as u64,
            &Serialized::serialize(params).unwrap()
        )
        .is_ok());
    rt.verify()
}

#[allow(dead_code)]
fn current_power_total<BS: BlockStore>(rt: &mut MockRuntime<'_, BS>) -> CurrentTotalPowerReturn {
    rt.expect_validate_caller_any();
    let ret = rt
        .call(
            &*POWER_ACTOR_CODE_ID,
            Method::CurrentTotalPower as u64,
            &Serialized::default(),
        )
        .unwrap();
    rt.verify();
    ret.deserialize().unwrap()
}

#[allow(dead_code)]
fn update_pledge_total<BS: BlockStore>(
    rt: &mut MockRuntime<'_, BS>,
    miner: Address,
    delta: TokenAmount,
) {
    rt.set_caller(MINER_ACTOR_CODE_ID.clone(), miner.clone());
    rt.expect_validate_caller_type(&[MINER_ACTOR_CODE_ID.clone()]);

    // Not sure why i cant serialize when it already exists. need to double check
    let params = &Serialized::serialize(BigIntSer(&delta)).unwrap();
    assert!(rt
        .call(
            &*POWER_ACTOR_CODE_ID,
            Method::UpdatePledgeTotal as u64,
            params
        )
        .is_ok());
    rt.verify();
}

#[allow(dead_code)]
fn enroll_cron_event<BS: BlockStore>(
    rt: &mut MockRuntime<'_, BS>,
    miner: Address,
    epoch: i64,
    payload: &[u8],
) {
    rt.expect_validate_caller_type(&[MINER_ACTOR_CODE_ID.clone()]);
    rt.set_caller(MINER_ACTOR_CODE_ID.clone(), miner.clone());

    let params = EnrollCronEventParams {
        event_epoch: epoch,
        payload: Serialized::serialize(payload).unwrap(),
    };

    assert!(rt
        .call(
            &*POWER_ACTOR_CODE_ID,
            Method::EnrollCronEvent as u64,
            &Serialized::serialize(params).unwrap()
        )
        .is_ok());

    rt.verify();
}

fn construct_runtime<BS: BlockStore>(bs: &BS) -> MockRuntime<'_, BS> {
    let message = UnsignedMessage::builder()
        .to(STORAGE_POWER_ACTOR_ADDR.clone())
        .from(SYSTEM_ACTOR_ADDR.clone())
        .build()
        .unwrap();
    let mut rt = MockRuntime::new(bs, message);
    rt.set_caller(SYSTEM_ACTOR_CODE_ID.clone(), SYSTEM_ACTOR_ADDR.clone());
    construct_and_verify(&mut rt);
    return rt;
}

fn expect_total_power_eager<BS: BlockStore>(
    rt: &MockRuntime<BS>,
    expected_raw: StoragePower,
    expected_qa: StoragePower,
) {
    let state: State = rt.get_state().unwrap();
    let (raw_power, qa_power) = state.current_total_power();
    assert_eq!(expected_raw, raw_power);
    assert_eq!(expected_qa, qa_power);
}
fn expect_total_pledge_eager<BS: BlockStore>(rt: &MockRuntime<BS>, expected_pledge: TokenAmount) {
    let state: State = rt.get_state().unwrap();
    assert_eq!(expected_pledge, state.total_pledge_collateral);
}

fn on_consensus_fault<BS: BlockStore>(
    rt: &mut MockRuntime<BS>,
    miner : Address,
    pledge_amt : TokenAmount){
        rt.expect_validate_caller_type(&[MINER_ACTOR_CODE_ID.clone()]);
        rt.set_caller(MINER_ACTOR_CODE_ID.clone(), miner.clone());
        assert!(rt.call(&*POWER_ACTOR_CODE_ID,  Method::OnConsensusFault as u64  , &Serialized::serialize(&BigIntSer(&pledge_amt)).unwrap()).is_ok() ) ;
        rt.verify();
        let state : State = rt.get_state().unwrap();

        assert!(state.get_claim(rt.store, &miner).unwrap().is_none());
    }

mod test_construction {
    use super::*;
    const OWNER: u64 = 101;
    const MINER: u64 = 103;
    //const ACTR : = Fro"actor";

    #[test]
    fn simple_construction() {
        let bs = MemoryDB::default();
        let _ = construct_runtime(&bs);
    }

    #[test]
    fn create_miner_test() {
        let bs = MemoryDB::default();
        let mut rt = construct_runtime(&bs);

        let owner = Address::new_id(OWNER);
        let miner = Address::new_id(MINER);

        //peer : Vec<u8>, multi_addrs : Vec<u8> , seal_proof_type : RegisteredProof, value : TokenAmount ){

        create_miner(
            &mut rt,
            owner.clone(),
            owner.clone(),
            miner.clone(),
            Address::new_actor("actor".as_bytes()),
            "miner".as_bytes().to_vec(),
            vec![],
            RegisteredSealProof::StackedDRG2KiBV1,
            TokenAmount::from(10u8),
        );

        let state: State = rt.get_state().unwrap();
        assert_eq!(1, state.miner_count);
        assert_eq!(StoragePower::from(0u8), state.total_quality_adj_power);
        assert_eq!(StoragePower::from(0u8), state.total_raw_byte_power);
        assert_eq!(0, state.num_miners_meeting_min_power);

        //let claim_map: Hamt<Claim, MemoryDB> = Hamt::load(&state.claims, rt.store).unwrap();
        // let keys = claim_map.collect_keys().unwrap();
        // assert_eq!(1, keys.len());

        verify_map_size(&mut rt, state.claims.clone(), 1);

        // let map: Hamt<BytesKey, _> = Hamt::load(&state.claims, rt.store).unwrap();
        // let claim_map = Set::from_root(rt.store, &state.claims).unwrap();
        // let keys = claim_map.collect_keys().unwrap();
        // assert!(map.get::<_,Claim>(&keys[0]).unwrap().is_some());

        verify_empty_map(&mut rt, state.cron_event_queue);
    }
}

mod power_and_pledge {
    use super::*;
    const OWNER: u64 = 101;
    const MINER_1: u64 = 111;
    const MINER_2: u64 = 112;
    const MINER_3: u64 = 113;
    const MINER_4: u64 = 114;
    const MINER_5: u64 = 115;

    #[test]
    fn power_and_pledge_accounted() {
        let bs = MemoryDB::default();
        let mut rt = construct_runtime(&bs);
        let owner = Address::new_id(OWNER);
        let miner1 = Address::new_id(MINER_1);
        let miner2 = Address::new_id(MINER_2);
        let small_power_unit = StoragePower::from(1_000_000u64);

        let mut miner_seq = 0;
        create_miner_basic(
            &mut rt,
            &mut miner_seq,
            owner.clone(),
            owner.clone(),
            miner1.clone(),
        );
        create_miner_basic(
            &mut rt,
            &mut miner_seq,
            owner.clone(),
            owner.clone(),
            miner2.clone(),
        );

        // Current power total method requires method update
        let cp = current_power_total(&mut rt);
        assert_eq!(BigInt::from(0u8), cp.raw_byte_power);
        assert_eq!(BigInt::from(0u8), cp.quality_adj_power);
        assert_eq!(BigInt::from(0u8), cp.pledge_collateral);

        update_claimed_power(
            &mut rt,
            miner1.clone(),
            small_power_unit.clone(),
            small_power_unit.clone() * 2 as u64,
        );
        expect_total_power_eager(&rt, small_power_unit.clone(), &small_power_unit * 2);
        assert_eq!(BigInt::from(0u8), cp.pledge_collateral);

        update_claimed_power(
            &mut rt,
            miner2.clone(),
            small_power_unit.clone(),
            small_power_unit.clone(),
        );
        update_pledge_total(&mut rt, miner1.clone(), TokenAmount::from(1_000_000u64));
        expect_total_power_eager(&rt, &small_power_unit * 2, &small_power_unit * 3);
        expect_total_pledge_eager(&rt, TokenAmount::from(1_000_000u64));

        rt.verify();

        let state: State = rt.get_state().unwrap();
        let claim1 = state.get_claim(rt.store, &miner1).unwrap().unwrap();
        assert_eq!(small_power_unit, claim1.raw_byte_power);
        assert_eq!(&small_power_unit * 2 as u64, claim1.quality_adj_power);

        let claim2 = state.get_claim(rt.store, &miner2).unwrap().unwrap();
        assert_eq!(small_power_unit.clone(), claim2.raw_byte_power);
        assert_eq!(small_power_unit, claim2.quality_adj_power);

        // Subtract power and some pledge for miner2
        // has to be negative power_unit instea dof positive. need to chnage the type
        update_claimed_power(
            &mut rt,
            miner2.clone(),
            small_power_unit.clone() * -1,
            small_power_unit.clone() * -1,
        );

        update_pledge_total(&mut rt, miner2.clone(), TokenAmount::from(100_000u64) * -1);

        expect_total_power_eager(&rt, &small_power_unit * 1, &small_power_unit * 2);
        expect_total_pledge_eager(&rt, TokenAmount::from(900_000u64));

        let state: State = rt.get_state().unwrap();
        let claim2 = state.get_claim(rt.store, &miner2).unwrap().unwrap();
        assert_eq!(StoragePower::from(0u8), claim2.raw_byte_power);
        assert_eq!(StoragePower::from(0u8), claim2.quality_adj_power);
    }

    #[test]
    fn power_accounting_crossing_thresh() {
        let bs = MemoryDB::default();
        let mut rt = construct_runtime(&bs);
        let owner = Address::new_id(OWNER);
        let mut miner_seq = 0;
        let power_unit = CONSENSUS_MINER_MIN_POWER.clone();
        let small_power_unit = StoragePower::from(1_000_000u64);

        for miner in &[MINER_1, MINER_2, MINER_3, MINER_4, MINER_5] {
            let miner_addr = Address::new_id(*miner);
            create_miner_basic(
                &mut rt,
                &mut miner_seq,
                owner.clone(),
                owner.clone(),
                miner_addr.clone(),
            );
        }
        for miner in &[MINER_1, MINER_2, MINER_3] {
            let miner_addr = Address::new_id(*miner);
            update_claimed_power(
                &mut rt,
                miner_addr,
                small_power_unit.clone() / 2,
                small_power_unit.clone(),
            );
        }

        for miner in &[MINER_4, MINER_5] {
            let miner_addr = Address::new_id(*miner);
            update_claimed_power(
                &mut rt,
                miner_addr,
                power_unit.clone() / 2,
                power_unit.clone(),
            );
        }

        let expected_total_below: TokenAmount = &small_power_unit * 3 + &power_unit * 2;
        expect_total_power_eager(&rt, &expected_total_below / 2, expected_total_below.clone());

        let delta = &power_unit - &small_power_unit;

        update_claimed_power(&mut rt, Address::new_id(MINER_3), &delta / 2, delta.clone());

        let expected_total_above = power_unit * 3;
        expect_total_power_eager(&rt, &expected_total_above / 2, expected_total_above);

        let state: State = rt.get_state().unwrap();
        assert_eq!(3, state.num_miners_meeting_min_power);

        update_claimed_power(&mut rt, Address::new_id(MINER_3), &delta / -2, delta * -1);

        expect_total_power_eager(&rt, &expected_total_below / 2, expected_total_below);
    }

    #[test]
    fn all_power_dissapears(){
        let bs = MemoryDB::default();
        let mut rt = construct_runtime(&bs);
        let owner = Address::new_id(OWNER);
        let mut miner_seq = 0;
        let power_unit = CONSENSUS_MINER_MIN_POWER.clone();
        let small_power_unit = StoragePower::from(1_000_000u64);

        for miner in &[MINER_1, MINER_2, MINER_3, MINER_4] {
            let miner_addr = Address::new_id(*miner);
            create_miner_basic(
                &mut rt,
                &mut miner_seq,
                owner.clone(),
                owner.clone(),
                miner_addr.clone(),
            );
            update_claimed_power(
                &mut rt,
                miner_addr,
                power_unit.clone() ,
                power_unit.clone(),
            );
        }

        let exp_total = StoragePower::from( power_unit.clone() * 4);
        expect_total_power_eager(&rt, exp_total.clone(), exp_total.clone());

        update_claimed_power(
            &mut rt,
            Address::new_id(MINER_4),
            power_unit.clone() * -1 ,
            power_unit.clone() * -1,
        );
        let exp_total = StoragePower::from( power_unit.clone() * 3);
        expect_total_power_eager(&rt, exp_total.clone(), exp_total.clone());
    }

    #[test]
    pub fn thresh_qa_not_raw(){
        let bs = MemoryDB::default();
        let mut rt = construct_runtime(&bs);
        let owner = Address::new_id(OWNER);
        let mut miner_seq = 0;
        let power_unit = CONSENSUS_MINER_MIN_POWER.clone();
        
        for miner in &[MINER_1, MINER_2, MINER_3] {
            let miner_addr = Address::new_id(*miner);
            create_miner_basic(
                &mut rt,
                &mut miner_seq,
                owner.clone(),
                owner.clone(),
                miner_addr.clone(),
            );
            update_claimed_power(
                &mut rt,
                miner_addr,
                power_unit.clone() ,
                StoragePower::default(),
            );
        }

        let state : State = rt.get_state().unwrap();
        assert_eq!(0, state.num_miners_meeting_min_power);

        for miner in &[MINER_1, MINER_2, MINER_3] {
            let miner_addr = Address::new_id(*miner);
            update_claimed_power(
                &mut rt,
                miner_addr,
                StoragePower::default(),
                power_unit.clone() ,
            );
        }
        let state : State = rt.get_state().unwrap();
        assert_eq!(3, state.num_miners_meeting_min_power);
    }

    #[test]
    pub fn slash_miner_below_min(){
        let bs = MemoryDB::default();
        let mut rt = construct_runtime(&bs);
        let owner = Address::new_id(OWNER);
        let mut miner_seq = 0;
        let power_unit = CONSENSUS_MINER_MIN_POWER.clone();
        let small_power_unit = StoragePower::from(1_000_000u64);

        
        for miner in &[MINER_1, MINER_2, MINER_3] {
            let miner_addr = Address::new_id(*miner);
            create_miner_basic(
                &mut rt,
                &mut miner_seq,
                owner.clone(),
                owner.clone(),
                miner_addr.clone(),
            );
            update_claimed_power(
                &mut rt,
                miner_addr,
                power_unit.clone() ,
                power_unit.clone(),
            );
        }

        create_miner_basic(
            &mut rt,
            &mut miner_seq,
            owner.clone(),
            owner.clone(),
            Address::new_id(MINER_4),
        );
        update_claimed_power(
            &mut rt,
            Address::new_id(MINER_4),
            small_power_unit.clone() ,
            small_power_unit.clone(),
        );

        expect_total_power_eager(&rt, &power_unit * 3, &power_unit * 3);

        on_consensus_fault(&mut rt, Address::new_id(MINER_4), TokenAmount::default());
        
        expect_total_power_eager(&rt, &power_unit * 3, &power_unit * 3);
    }
}

mod test_cron {
    use super::*;

    const MINER_1: u64 = 101;
    const MINER_2: u64 = 102;
    const OWNER: u64 = 103;

    //#[test]
    fn calls_reward_actor() {
        let bs = MemoryDB::default();
        let mut rt = construct_runtime(&bs);
        rt.epoch = 1;
        rt.expect_validate_caller_addr(&[CRON_ACTOR_ADDR.clone()]);

        let expected_power = Serialized::serialize(BigIntSer(&StoragePower::from(0u8))).unwrap();
        rt.expect_send(
            REWARD_ACTOR_ADDR.clone(),
            RewardMethod::UpdateNetworkKPI as u64,
            expected_power,
            TokenAmount::from(0u8),
            Serialized::default(),
            ExitCode::Ok,
        );
        rt.set_caller(CRON_ACTOR_CODE_ID.clone(), CRON_ACTOR_ADDR.clone());

        assert!(rt
            .call(
                &*POWER_ACTOR_CODE_ID,
                Method::OnEpochTickEnd as u64,
                &Serialized::default()
            )
            .is_ok());

        rt.verify();
    }

    //#[test]
    fn event_called_next() {
        let bs = MemoryDB::default();
        let mut rt = construct_runtime(&bs);

        //  0 - genesis
        //  1 - block - registers events
        //  2 - null  - has event
        //  3 - null
        //  4 - block - has event

        rt.epoch = 1;
        let miner1 = Address::new_id(MINER_1);
        let miner2 = Address::new_id(MINER_2);
        enroll_cron_event(&mut rt, miner1, 2, &[1u8, 3u8]);
        enroll_cron_event(&mut rt, miner2, 4, &[2u8, 3u8]);

        let params_1 = Serialized::serialize([1u8, 3u8]).unwrap();
        let params_2 = Serialized::serialize([2u8, 3u8]).unwrap();
        let params_3 = BigInt::default();
        //Serialized::serialize(BigInt::from(0u8)).unwrap();

        rt.epoch = 4;
        rt.expect_validate_caller_addr(&[CRON_ACTOR_ADDR.clone()]);
        rt.expect_send(
            miner1.clone(),
            MinerMethod::OnDeferredCronEvent as u64,
            params_1,
            TokenAmount::from(0u8),
            Serialized::default(),
            ExitCode::Ok,
        );
        rt.expect_send(
            miner2.clone(),
            MinerMethod::OnDeferredCronEvent as u64,
            params_2,
            TokenAmount::from(0u8),
            Serialized::default(),
            ExitCode::Ok,
        );
        //rt.expect_send(REWARD_ACTOR_ADDR.clone(), RewardMethod::UpdateNetworkKPI as u64, params_3, TokenAmount::from(0u8), Serialized::default(), ExitCode::Ok);
        rt.set_caller(CRON_ACTOR_CODE_ID.clone(), CRON_ACTOR_ADDR.clone());

        assert!(rt
            .call(
                &*POWER_ACTOR_CODE_ID,
                Method::OnEpochTickEnd as u64,
                &Serialized::default()
            )
            .is_ok());
        rt.verify();
    }

    //#[test]
    fn handles_failed_call() {
        let bs = MemoryDB::default();
        let mut rt = construct_runtime(&bs);

        rt.epoch = 1;
        let miner1 = Address::new_id(MINER_1);
        let miner2 = Address::new_id(MINER_2);
        enroll_cron_event(&mut rt, miner1, 2, &[]);
        enroll_cron_event(&mut rt, miner2, 4, &[]);

        let owner = Address::new_id(OWNER);
        let mut miner_seq = 1;
        create_miner_basic(
            &mut rt,
            &mut miner_seq,
            owner.clone(),
            owner.clone(),
            miner1.clone(),
        );
        create_miner_basic(
            &mut rt,
            &mut miner_seq,
            owner.clone(),
            owner.clone(),
            miner2.clone(),
        );

        let raw_pow = CONSENSUS_MINER_MIN_POWER.clone();
        let qa_pow = CONSENSUS_MINER_MIN_POWER.clone();
        update_claimed_power(&mut rt, miner1.clone(), raw_pow.clone(), qa_pow.clone());
        let start_pow = current_power_total(&mut rt);
        assert_eq!(raw_pow, start_pow.raw_byte_power);
        assert_eq!(qa_pow, start_pow.quality_adj_power);

        rt.epoch = 2;
        rt.expect_validate_caller_addr(&[CRON_ACTOR_ADDR.clone()]);
        rt.expect_send(
            miner1.clone(),
            MinerMethod::OnDeferredCronEvent as u64,
            Serialized::default(),
            TokenAmount::from(0u8),
            Serialized::default(),
            ExitCode::ErrIllegalState,
        );
        rt.expect_send(
            miner2.clone(),
            MinerMethod::OnDeferredCronEvent as u64,
            Serialized::default(),
            TokenAmount::from(0u8),
            Serialized::default(),
            ExitCode::Ok,
        );
        //rt.expect_send(REWARD_ACTOR_ADDR.clone(), RewardMethod::UpdateNetworkKPI as u64 , &Serialized::default(TokenAmount::from(0u8)), TokenAmount::from(0u8), &Serialized::default(), ExitCode::ErrIllegalState);
        rt.set_caller(CRON_ACTOR_CODE_ID.clone(), CRON_ACTOR_ADDR.clone());
        assert!(rt
            .call(
                &*POWER_ACTOR_CODE_ID,
                Method::OnEpochTickEnd as u64,
                &Serialized::default()
            )
            .is_ok());
        rt.verify();

        let new_pow = current_power_total(&mut rt);
        assert_eq!(StoragePower::from(0u8), new_pow.raw_byte_power);
        assert_eq!(StoragePower::from(0u8), new_pow.quality_adj_power);

        rt.epoch = 3;
        rt.expect_validate_caller_addr(&[CRON_ACTOR_ADDR.clone()]);
        //rt.expect_send(REWARD_ACTOR_ADDR.clone(), RewardMethod::UpdateNetworkKPI as u64 , &Serialized::default(TokenAmount::from(0u8)), TokenAmount::from(0u8), &Serialized::default(), ExitCode::OK);
        rt.set_caller(CRON_ACTOR_CODE_ID.clone(), CRON_ACTOR_ADDR.clone());
        assert!(rt
            .call(
                &*POWER_ACTOR_CODE_ID,
                Method::OnEpochTickEnd as u64,
                &Serialized::default()
            )
            .is_ok());
        rt.verify();
    }
}
