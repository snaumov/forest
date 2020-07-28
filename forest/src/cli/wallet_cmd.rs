// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use rpc_client::{genesis, head, messages};
use cid::Cid;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct WalletCommands {
    #[structopt(long, help = "Creates new wallet")]
    pub new: bool,
}

impl ChainCommands {
    pub async fn run(&self) {
        if self.new {
            // do something
        }
    }
}
