// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

mod chain_api;
mod sync_api;
mod wallet_api;
mod state_api;

use async_std::sync::{RwLock, Sender};
use blockstore::BlockStore;
use chain_sync::{BadBlockCache, SyncState};
use forest_libp2p::NetworkMessage;
use jsonrpc_v2::{Data, MapRouter, RequestObject, Server};
use std::sync::Arc;
use tide::{Request, Response, StatusCode};
use wallet::KeyStore;

/// This is where you store persistant data, or at least access to stateful data.

pub struct RpcState<DB, KS>
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    pub store: Arc<DB>,
    pub keystore: Arc<RwLock<KS>>,
    pub bad_blocks: Arc<BadBlockCache>,
    pub sync_state: Arc<RwLock<SyncState>>,
    pub network_send: Sender<NetworkMessage>,
    pub network_name: String,
}

async fn handle_json_rpc(mut req: Request<Server<MapRouter>>) -> tide::Result {
    let call: RequestObject = req.body_json().await?;
    let res = req.state().handle(call).await;
    Ok(Response::new(StatusCode::Ok).body_json(&res)?)
}

pub async fn start_rpc<DB, KS>(state: RpcState<DB, KS>, rpc_endpoint: &str)
where
    DB: BlockStore + Send + Sync + 'static,
    KS: KeyStore + Send + Sync + 'static,
{
    use chain_api::*;
    use sync_api::*;
    use wallet_api::*;

    let rpc = Server::new()
        .with_data(Data::new(state))
        .with_method(
            "Filecoin.ChainGetMessage",
            chain_api::chain_get_message::<DB, KS>,
        )
        .with_method("Filecoin.ChainGetObj", chain_read_obj::<DB, KS>)
        .with_method("Filecoin.ChainHasObj", chain_has_obj::<DB, KS>)
        .with_method(
            "Filecoin.ChainGetBlockMessages",
            chain_block_messages::<DB, KS>,
        )
        .with_method(
            "Filecoin.ChainGetTipsetByHeight",
            chain_get_tipset_by_height::<DB, KS>,
        )
        .with_method("Filecoin.ChainGetGenesis", chain_get_genesis::<DB, KS>)
        .with_method("Filecoin.ChainTipsetWeight", chain_tipset_weight::<DB, KS>)
        .with_method("Filecoin.ChainGetTipset", chain_get_tipset::<DB, KS>)
        .with_method("Filecoin.GetRandomness", chain_get_randomness::<DB, KS>)
        .with_method(
            "Filecoin.ChainGetBlock",
            chain_api::chain_get_block::<DB, KS>,
        )
        .with_method("Filecoin.ChainHead", chain_head::<DB, KS>)
        // Sync API
        .with_method("Filecoin.SyncCheckBad", sync_check_bad::<DB, KS>)
        .with_method("Filecoin.SyncMarkBad", sync_mark_bad::<DB, KS>)
        .with_method("Filecoin.SyncState", sync_state::<DB, KS>)
        .with_method("Filecoin.SyncSubmitBlock", sync_submit_block::<DB, KS>)
        // Wallet API
        .with_method("Filecoin.WalletBalance", wallet_balance::<DB, KS>)
        .with_method(
            "Filecoin.WalletDefaultAddress",
            wallet_default_address::<DB, KS>,
        )
        .with_method("Filecoin.WalletExport", wallet_export::<DB, KS>)
        .with_method("Filecoin.WalletHas", wallet_has::<DB, KS>)
        .with_method("Filecoin.WalletImport", wallet_import::<DB, KS>)
        .with_method("Filecoin.WalletList", wallet_list::<DB, KS>)
        .with_method("Filecoin.WalletNew", wallet_new::<DB, KS>)
        .with_method("Filecoin.WalletSetDefault", wallet_set_default::<DB, KS>)
        .with_method("Filecoin.WalletSign", wallet_sign::<DB, KS>)
        .with_method("Filecoin.WalletSignMessage", wallet_sign_message::<DB, KS>)
        .with_method("Filecoin.WalletVerify", wallet_verify::<DB, KS>)
        .finish_unwrapped();

    let mut app = tide::Server::with_state(rpc);
    app.at("/rpc/v0").post(handle_json_rpc);
    app.listen(rpc_endpoint).await.unwrap();
}
