// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

mod common;

use actor::{
    init::{ConstructorParams, ExecParams, ExecReturn, Method as INIT_METHOD},
    power::{CreateMinerParams, CreateMinerReturn, Method,  State},
    multisig::{Transaction},
    Multimap, Set, ACCOUNT_ACTOR_CODE_ID, FIRST_NON_SINGLETON_ADDR, INIT_ACTOR_ADDR,
    INIT_ACTOR_CODE_ID, MINER_ACTOR_CODE_ID, MULTISIG_ACTOR_CODE_ID, PAYCH_ACTOR_CODE_ID,
    POWER_ACTOR_CODE_ID, STORAGE_POWER_ACTOR_ADDR, SYSTEM_ACTOR_ADDR, SYSTEM_ACTOR_CODE_ID,
};
use address::Address;
use cid::Cid;
use common::*;
use db::MemoryDB;
use fil_types::{RegisteredSealProof, SectorSize, StoragePower};
//use filecoin_proofs_api::{ RegisteredSealProof};
use ipld_blockstore::BlockStore;
use ipld_hamt::BytesKey;
use ipld_hamt::{Error as HamtError, Hamt};
use message::{Message, UnsignedMessage};
use num_bigint::BigUint;
use serde::Serialize;
use vm::{ActorError, ExitCode, Serialized, TokenAmount, METHOD_CONSTRUCTOR, METHOD_SEND};

fn verify_map_size<BS: BlockStore>(rt: &mut MockRuntime<'_, BS>, cid: Cid,expected : u64) {
    
    let map: Hamt<BytesKey, _> = Hamt::load(&cid, rt.store).unwrap();
    let mut count = 0;
    map
        .for_each(|_, _: Transaction| {
            count += 1;
            Ok(())
        }).unwrap();
    assert_eq!(count, expected);
}

fn verify_empty_map<BS: BlockStore>(rt: &mut MockRuntime<'_, BS>, cid: Cid) {
    verify_map_size(rt, cid,0) 
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
    rt.received = value.clone();

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

mod test_construction {
    use super::*;
    const OWNER: u64 = 101;
    const MINER: u64 = 103;
    //const ACTR : = Fro"actor";

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

    #[test]
    fn simple_construction() {
        let bs = MemoryDB::default();
        let rt = construct_runtime(&bs);
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

        let claim_map = Set::from_root(rt.store, &state.claims).unwrap();
        let keys = claim_map.collect_keys().unwrap();
        assert_eq!(1, keys.len());

        let claim_map: Hamt<BytesKey, _> = Hamt::load(&state.claims, rt.store).unwrap();

        verify_map_size(&mut rt, state.claims, 1);


        //assert!(claim_map.get(&keys[0]).unwrap().is_some());

        verify_empty_map(&mut rt, state.cron_event_queue);
    }
}
