// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT
#![allow(unused_imports)]

use jsonrpc_core_client::{
    transports::http,
	RpcError,
	RpcChannel,
	TypedClient
};
use jsonrpc_core::futures::future::{self, Future, FutureResult};
use blocks::{
    header::json::BlockHeaderJson, tipset_json::TipsetJson, BlockHeader, Tipset, TipsetKeys,
};
use hyper::rt;

// EXAMPLE
pub fn chain_get_genesis() -> TipsetJson
{
	let (tx, rx) = std::sync::mpsc::channel();
	let uri = "http://127.0.0.1:1234/rpc/v0";
	let run = http::connect(uri)
			.and_then(|client: TestClient| {
				client.chain_get_genesis().and_then(move |result| {
					drop(client);
					let _ = tx.send(result);
					
					Ok(())
				})
			})
			.map_err(|e| println!("RPC Client error: {:?}", e));
	
	rt::run(run);
	
	rx.recv_timeout(std::time::Duration::from_secs(3)).unwrap()
}

#[derive(Clone)]
struct TestClient(TypedClient);

impl From<RpcChannel> for TestClient {
	fn from(channel: RpcChannel) -> Self {
		TestClient(channel.into())
	}
}

impl TestClient {
	fn chain_get_genesis(&self) -> impl Future<Item = TipsetJson, Error = RpcError> {
		self.0.call_method("Filecoin.ChainGetGenesis", "String", ())
	}
}