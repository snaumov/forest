// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use super::{policy::*, types::*, DealProposal, DealState, DEAL_UPDATES_INTERVAL};
use crate::{make_map_with_root, BalanceTable, DealID, Map, SetMultimap};
use address::Address;
use cid::Cid;
use clock::{ChainEpoch, EPOCH_UNDEFINED};
use encoding::tuple::*;
use encoding::Cbor;
use ipld_blockstore::BlockStore;
use num_bigint::{bigint_ser, Sign};
use num_traits::Zero;
use vm::{actor_error, ActorError, ExitCode, TokenAmount};

/// Market actor state
#[derive(Default, Serialize_tuple, Deserialize_tuple)]
pub struct State {
    /// Amt<DealID, DealProposal>
    pub proposals: Cid,
    /// Amt<DealID, DealState>
    pub states: Cid,

    /// PendingProposals tracks dealProposals that have not yet reached their deal start date.
    /// We track them here to ensure that miners can't publish the same deal proposal twice
    pub pending_proposals: Cid,

    /// Total amount held in escrow, indexed by actor address (including both locked and unlocked amounts).
    pub escrow_table: Cid,

    /// Amount locked, indexed by actor address.
    /// Note: the amounts in this table do not affect the overall amount in escrow:
    /// only the _portion_ of the total escrow amount that is locked.
    pub locked_table: Cid,

    /// Deal id state sequential incrementer
    pub next_id: DealID,

    /// Metadata cached for efficient iteration over deals.
    /// SetMultimap<Address>
    pub deal_ops_by_epoch: Cid,
    pub last_cron: ChainEpoch,

    /// Total Client Collateral that is locked -> unlocked when deal is terminated
    #[serde(with = "bigint_ser")]
    pub total_client_locked_colateral: TokenAmount,
    /// Total Provider Collateral that is locked -> unlocked when deal is terminated
    #[serde(with = "bigint_ser")]
    pub total_provider_locked_colateral: TokenAmount,
    /// Total storage fee that is locked in escrow -> unlocked when payments are made
    #[serde(with = "bigint_ser")]
    pub total_client_storage_fee: TokenAmount,
}

impl State {
    pub fn new(empty_arr: Cid, empty_map: Cid, empty_mset: Cid) -> Self {
        Self {
            proposals: empty_arr.clone(),
            states: empty_arr,
            pending_proposals: empty_map.clone(),
            escrow_table: empty_map.clone(),
            locked_table: empty_map,
            next_id: 0,
            deal_ops_by_epoch: empty_mset,
            last_cron: EPOCH_UNDEFINED,

            total_client_locked_colateral: TokenAmount::default(),
            total_provider_locked_colateral: TokenAmount::default(),
            total_client_storage_fee: TokenAmount::default(),
        }
    }

    pub(super) fn mutator<'bs, BS: BlockStore>(
        &mut self,
        store: &'bs BS,
    ) -> MarketStateMutation<'bs, '_, BS> {
        MarketStateMutation::new(self, store)
    }
}

fn deal_get_payment_remaining(deal: &DealProposal, mut slash_epoch: ChainEpoch) -> TokenAmount {
    assert!(
        slash_epoch <= deal.end_epoch,
        "Current epoch must be before the end epoch of the deal"
    );

    // Payments are always for start -> end epoch irrespective of when the deal is slashed.
    slash_epoch = std::cmp::max(slash_epoch, deal.start_epoch);

    let duration_remaining = deal.end_epoch - slash_epoch;
    assert!(duration_remaining >= 0);

    &deal.storage_price_per_epoch * duration_remaining as u64
}

impl Cbor for State {}

#[derive(Debug, PartialEq)]
pub(super) enum Permission {
    Invalid,
    ReadOnly,
    Write,
}

pub(super) enum Reason {
    ClientCollateral,
    ClientStorageFee,
    ProviderCollateral,
}

pub(super) struct MarketStateMutation<'bs, 's, BS> {
    pub(super) st: &'s mut State,
    pub(super) store: &'bs BS,

    pub(super) proposal_permit: Permission,
    pub(super) deal_proposals: Option<DealArray<'bs, BS>>,

    pub(super) state_permit: Permission,
    pub(super) deal_states: Option<DealMetaArray<'bs, BS>>,

    pub(super) escrow_permit: Permission,
    pub(super) escrow_table: Option<BalanceTable<'bs, BS>>,

    pub(super) pending_permit: Permission,
    pub(super) pending_deals: Option<Map<'bs, BS, DealProposal>>,

    pub(super) dpe_permit: Permission,
    pub(super) deals_by_epoch: Option<SetMultimap<'bs, BS>>,

    pub(super) locked_permit: Permission,
    pub(super) locked_table: Option<BalanceTable<'bs, BS>>,
    pub(super) total_client_locked_colateral: Option<TokenAmount>,
    pub(super) total_provider_locked_colateral: Option<TokenAmount>,
    pub(super) total_client_storage_fee: Option<TokenAmount>,

    pub(super) next_deal_id: DealID,
}

impl<'bs, 's, BS> MarketStateMutation<'bs, 's, BS>
where
    BS: BlockStore,
{
    pub(super) fn new(st: &'s mut State, store: &'bs BS) -> Self {
        Self {
            next_deal_id: st.next_id,
            st,
            store,
            proposal_permit: Permission::Invalid,
            deal_proposals: None,
            state_permit: Permission::Invalid,
            deal_states: None,
            escrow_permit: Permission::Invalid,
            escrow_table: None,
            pending_permit: Permission::Invalid,
            pending_deals: None,
            dpe_permit: Permission::Invalid,
            deals_by_epoch: None,
            locked_permit: Permission::Invalid,
            locked_table: None,
            total_client_locked_colateral: None,
            total_provider_locked_colateral: None,
            total_client_storage_fee: None,
        }
    }

    pub(super) fn build(&mut self) -> Result<&mut Self, String> {
        if self.proposal_permit != Permission::Invalid {
            self.deal_proposals = Some(DealArray::load(&self.st.proposals, self.store)?);
        }

        if self.state_permit != Permission::Invalid {
            self.deal_states = Some(DealMetaArray::load(&self.st.states, self.store)?);
        }

        if self.locked_permit != Permission::Invalid {
            self.locked_table = Some(BalanceTable::from_root(self.store, &self.st.locked_table)?);
            self.total_client_locked_colateral =
                Some(self.st.total_client_locked_colateral.clone());
            self.total_client_storage_fee = Some(self.st.total_client_storage_fee.clone());
            self.total_provider_locked_colateral =
                Some(self.st.total_provider_locked_colateral.clone());
        }

        if self.escrow_permit != Permission::Invalid {
            self.escrow_table = Some(BalanceTable::from_root(self.store, &self.st.escrow_table)?);
        }

        if self.pending_permit != Permission::Invalid {
            self.pending_deals = Some(make_map_with_root(&self.st.pending_proposals, self.store)?);
        }

        if self.dpe_permit != Permission::Invalid {
            self.deals_by_epoch = Some(SetMultimap::from_root(
                self.store,
                &self.st.deal_ops_by_epoch,
            )?);
        }

        self.next_deal_id = self.st.next_id;

        Ok(self)
    }

    pub(super) fn with_deal_proposals(&mut self, permit: Permission) -> &mut Self {
        self.proposal_permit = permit;
        self
    }

    pub(super) fn with_deal_states(&mut self, permit: Permission) -> &mut Self {
        self.state_permit = permit;
        self
    }

    pub(super) fn with_escrow_table(&mut self, permit: Permission) -> &mut Self {
        self.escrow_permit = permit;
        self
    }

    pub(super) fn with_locked_table(&mut self, permit: Permission) -> &mut Self {
        self.locked_permit = permit;
        self
    }

    pub(super) fn with_pending_proposals(&mut self, permit: Permission) -> &mut Self {
        self.pending_permit = permit;
        self
    }

    pub(super) fn with_deals_by_epoch(&mut self, permit: Permission) -> &mut Self {
        self.dpe_permit = permit;
        self
    }

    pub(super) fn commit_state(&mut self) -> Result<(), String> {
        if self.proposal_permit == Permission::Write {
            if let Some(s) = &mut self.deal_proposals {
                self.st.proposals = s
                    .flush()
                    .map_err(|e| format!("failed to flush deal proposals: {}", e))?;
            }
        }

        if self.state_permit == Permission::Write {
            if let Some(s) = &mut self.deal_states {
                self.st.states = s
                    .flush()
                    .map_err(|e| format!("failed to flush deal states: {}", e))?;
            }
        }

        if self.locked_permit == Permission::Write {
            if let Some(s) = &mut self.locked_table {
                self.st.locked_table = s
                    .root()
                    .map_err(|e| format!("failed to flush locked table: {}", e))?;
            }
            if let Some(s) = &mut self.total_client_locked_colateral {
                self.st.total_client_locked_colateral = s.clone();
            }
            if let Some(s) = &mut self.total_provider_locked_colateral {
                self.st.total_provider_locked_colateral = s.clone();
            }
            if let Some(s) = &mut self.total_client_storage_fee {
                self.st.total_client_storage_fee = s.clone();
            }
        }

        if self.escrow_permit == Permission::Write {
            if let Some(s) = &mut self.escrow_table {
                self.st.escrow_table = s
                    .root()
                    .map_err(|e| format!("failed to flush escrow table: {}", e))?;
            }
        }

        if self.pending_permit == Permission::Write {
            if let Some(s) = &mut self.pending_deals {
                self.st.pending_proposals = s
                    .flush()
                    .map_err(|e| format!("failed to flush escrow table: {}", e))?;
            }
        }

        if self.dpe_permit == Permission::Write {
            if let Some(s) = &mut self.deals_by_epoch {
                self.st.deal_ops_by_epoch = s
                    .root()
                    .map_err(|e| format!("failed to flush escrow table: {}", e))?;
            }
        }

        self.st.next_id = self.next_deal_id;

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Deal state operations
    ////////////////////////////////////////////////////////////////////////////////
    #[allow(clippy::too_many_arguments)]
    pub(super) fn update_pending_deal_state(
        &mut self,
        state: DealState,
        deal: DealProposal,
        epoch: ChainEpoch,
    ) -> Result<(TokenAmount, ChainEpoch, bool), ActorError> {
        let ever_updated = state.last_updated_epoch != EPOCH_UNDEFINED;
        let ever_slashed = state.slash_epoch != EPOCH_UNDEFINED;

        // if the deal was ever updated, make sure it didn't happen in the future
        assert!(!ever_updated || state.last_updated_epoch <= epoch);

        // This would be the case that the first callback somehow triggers before it is scheduled to
        // This is expected not to be able to happen
        if deal.start_epoch > epoch {
            return Ok((TokenAmount::zero(), EPOCH_UNDEFINED, false));
        }

        let payment_end_epoch = if ever_slashed {
            assert!(
                state.slash_epoch <= deal.end_epoch,
                "Epoch slashed must be less or equal to the end epoch"
            );
            assert!(
                epoch >= state.slash_epoch,
                "Epoch slashed must be less or equal to the end epoch"
            );
            state.slash_epoch
        } else {
            deal.end_epoch
        };

        let payment_start_epoch = if ever_updated && state.last_updated_epoch > deal.start_epoch {
            state.last_updated_epoch
        } else {
            deal.start_epoch
        };

        let elapsed_end = std::cmp::min(epoch, payment_end_epoch);

        let num_epochs_elapsed = elapsed_end - payment_start_epoch;

        self.transfer_balance(
            &deal.client,
            &deal.provider,
            &(&deal.storage_price_per_epoch * num_epochs_elapsed as u64),
        )?;

        if ever_slashed {
            // unlock client collateral and locked storage fee
            let payment_remaining = deal_get_payment_remaining(&deal, state.slash_epoch);

            // Unlock remaining storage fee
            self.unlock_balance(&deal.client, &payment_remaining, Reason::ClientStorageFee)
                .map_err(|e| {
                    actor_error!(ErrIllegalState;
                        "failed to unlock remaining client storage fee: {}", e)
                })?;

            // Unlock client collateral
            self.unlock_balance(
                &deal.client,
                &deal.client_collateral,
                Reason::ClientCollateral,
            )
            .map_err(|e| {
                actor_error!(ErrIllegalState;
                    "failed to unlock client collateral: {}", e)
            })?;

            // slash provider collateral
            let slashed = deal.provider_collateral.clone();
            self.slash_balance(&deal.provider, &slashed, Reason::ProviderCollateral)
                .map_err(|e| actor_error!(ErrIllegalState; "slashing balance: {}", e))?;

            return Ok((slashed, EPOCH_UNDEFINED, true));
        }

        if epoch >= deal.end_epoch {
            self.process_deal_expired(&deal, state)?;
            return Ok((TokenAmount::zero(), EPOCH_UNDEFINED, true));
        }

        // We're explicitly not inspecting the end epoch and may process a deal's expiration late,
        // in order to prevent an outsider from loading a cron tick by activating too many deals
        // with the same end epoch.
        let next = epoch + DEAL_UPDATES_INTERVAL;

        Ok((TokenAmount::zero(), next, false))
    }

    /// Deal start deadline elapsed without appearing in a proven sector.
    /// Slash a portion of provider's collateral, and unlock remaining collaterals
    /// for both provider and client.
    pub(super) fn process_deal_init_timed_out(
        &mut self,
        deal: &DealProposal,
    ) -> Result<TokenAmount, ActorError> {
        self.unlock_balance(
            &deal.client,
            &deal.total_storage_fee(),
            Reason::ClientStorageFee,
        )
        .map_err(
            |e| actor_error!(ErrIllegalState; "failure unlocking client storage fee: {}", e),
        )?;

        self.unlock_balance(
            &deal.client,
            &deal.client_collateral,
            Reason::ClientCollateral,
        )
        .map_err(|e| actor_error!(ErrIllegalState; "failure unlocking client collateral: {}", e))?;

        let amount_slashed =
            collateral_penalty_for_deal_activation_missed(deal.provider_collateral.clone());
        let amount_remaining = deal.provider_balance_requirement() - &amount_slashed;

        self.slash_balance(&deal.provider, &amount_slashed, Reason::ProviderCollateral)
            .map_err(|e| actor_error!(ErrIllegalState; "failed to slash balance: {}", e))?;

        self.unlock_balance(
            &deal.provider,
            &amount_remaining,
            Reason::ProviderCollateral,
        )
        .map_err(
            |e| actor_error!(ErrIllegalState; "failed to unlock deal provider balance: {}", e),
        )?;

        Ok(amount_slashed)
    }

    /// Normal expiration. Unlock collaterals for both miner and client.
    fn process_deal_expired(
        &mut self,
        deal: &DealProposal,
        state: DealState,
    ) -> Result<(), ActorError>
    where
        BS: BlockStore,
    {
        assert_ne!(
            state.sector_start_epoch, EPOCH_UNDEFINED,
            "Sector start epoch must be initialized at this point"
        );

        self.unlock_balance(
            &deal.provider,
            &deal.provider_collateral,
            Reason::ProviderCollateral,
        )
        .map_err(
            |e| actor_error!(ErrIllegalArgument; "failed unlocking deal provider balance: {}", e),
        )?;

        self.unlock_balance(
            &deal.client,
            &deal.client_collateral,
            Reason::ClientCollateral,
        )
        .map_err(
            |e| actor_error!(ErrIllegalArgument; "failed unlocking deal client balance: {}", e),
        )?;

        Ok(())
    }

    pub(super) fn generate_storage_deal_id(&mut self) -> DealID {
        let ret = self.next_deal_id;
        self.next_deal_id += 1;
        ret
    }

    pub(super) fn maybe_lock_balance(
        &mut self,
        addr: &Address,
        amount: &TokenAmount,
    ) -> Result<(), ActorError> {
        assert_ne!(amount.sign(), Sign::Minus);

        let prev_locked =
            self.locked_table.as_ref().unwrap().get(addr).map_err(
                |e| actor_error!(ErrIllegalState; "failed to get locked balance: {}", e),
            )?;

        let escrow_balance =
            self.escrow_table.as_ref().unwrap().get(addr).map_err(
                |e| actor_error!(ErrIllegalState; "failed to get escrow balance: {}", e),
            )?;

        if &prev_locked + amount > escrow_balance {
            return Err(actor_error!(ErrInsufficientFunds;
                    "not enough balance to lock for addr{}: \
                    escrow balance {} < prev locked {} + amount {}", 
                    addr, escrow_balance, prev_locked, amount));
        }

        self.locked_table
            .as_mut()
            .unwrap()
            .add(addr, amount)
            .map_err(|e| actor_error!(ErrIllegalState; "failed to add locked balance: {}", e))?;
        Ok(())
    }

    pub(super) fn lock_client_and_provider_balances(
        &mut self,
        proposal: &DealProposal,
    ) -> Result<(), ActorError> {
        self.maybe_lock_balance(&proposal.client, &proposal.client_balance_requirement())
            .map_err(|e| e.wrap("failed to lock client funds"))?;

        self.maybe_lock_balance(&proposal.provider, &proposal.provider_collateral)
            .map_err(|e| e.wrap("failed to lock provider funds"))?;

        if let Some(v) = self.total_client_locked_colateral.as_mut() {
            *v += &proposal.client_collateral;
        }
        if let Some(v) = self.total_client_storage_fee.as_mut() {
            *v += proposal.total_storage_fee();
        }
        if let Some(v) = self.total_provider_locked_colateral.as_mut() {
            *v += &proposal.provider_collateral;
        }
        Ok(())
    }

    fn unlock_balance(
        &mut self,
        addr: &Address,
        amount: &TokenAmount,
        lock_reason: Reason,
    ) -> Result<(), String> {
        assert_ne!(amount.sign(), Sign::Minus);
        self.locked_table
            .as_mut()
            .unwrap()
            .must_subtract(addr, amount)?;

        match lock_reason {
            Reason::ClientCollateral => self.total_client_locked_colateral.as_mut().map(|v| {
                *v -= amount;
            }),
            Reason::ClientStorageFee => self.total_client_storage_fee.as_mut().map(|v| {
                *v -= amount;
            }),
            Reason::ProviderCollateral => self.total_provider_locked_colateral.as_mut().map(|v| {
                *v -= amount;
            }),
        };

        Ok(())
    }

    /// move funds from locked in client to available in provider
    fn transfer_balance(
        &mut self,
        from_addr: &Address,
        to_addr: &Address,
        amount: &TokenAmount,
    ) -> Result<(), ActorError> {
        assert!(amount >= &TokenAmount::from(0));

        // Subtract from locked and escrow tables
        self.escrow_table
            .as_mut()
            .unwrap()
            .must_subtract(from_addr, &amount)
            .map_err(|e| actor_error!(ErrIllegalState; "subtract from escrow: {}", e))?;

        self.unlock_balance(from_addr, &amount, Reason::ClientStorageFee)
            .map_err(|e| actor_error!(ErrIllegalState; "subtract from locked: {}", e))?;

        // Add subtracted amount to the recipient
        self.escrow_table
            .as_mut()
            .unwrap()
            .add(to_addr, &amount)
            .map_err(|e| actor_error!(ErrIllegalState; "add to escrow: {}", e))?;

        Ok(())
    }

    fn slash_balance(
        &mut self,
        addr: &Address,
        amount: &TokenAmount,
        lock_reason: Reason,
    ) -> Result<(), String> {
        assert!(amount >= &TokenAmount::from(0));

        // Subtract from locked and escrow tables
        self.escrow_table
            .as_mut()
            .unwrap()
            .must_subtract(addr, &amount)?;
        self.unlock_balance(addr, amount, lock_reason)
    }
}
