// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use chain::Error as ChainError;
use forest_blocks::{Tipset, TipsetKeys};
use forest_cid::Cid;
use ipld_blockstore::BlockStore;
use log::debug;
use std::collections::HashMap;
use std::sync::Arc;

use super::{
    BlockSyncRequest, BlockSyncResponse, BlockSyncResponseStatus, CompactedMessages, TipsetBundle,
};

/// Builds blocksync response out of chain data.
pub fn make_blocksync_response<DB>(db: Arc<DB>, request: &BlockSyncRequest) -> BlockSyncResponse
where
    DB: BlockStore,
{
    let mut response_chain: Vec<TipsetBundle> = vec![];

    let mut curr_tipset_cids = request.start.clone();

    loop {
        let mut tipset_bundle: TipsetBundle = Default::default();
        let tipset = match chain::tipset_from_keys(db.as_ref(), &TipsetKeys::new(curr_tipset_cids))
        {
            Ok(tipset) => tipset,
            Err(err) => {
                debug!("Cannot get tipset from keys: {}", err);

                return BlockSyncResponse {
                    chain: vec![],
                    status: BlockSyncResponseStatus::InternalError,
                    message: "Tipset was not found in the database".to_owned(),
                };
            }
        };

        if request.include_blocks() {
            tipset_bundle.blocks = tipset.blocks().to_vec();
        }

        if request.include_messages() {
            match compact_messages(db.as_ref(), &tipset) {
                Ok(compacted_messages) => tipset_bundle.messages = Some(compacted_messages),
                Err(err) => {
                    debug!("Cannot compact messages for tipset: {}", err);

                    return BlockSyncResponse {
                        chain: vec![],
                        status: BlockSyncResponseStatus::InternalError,
                        message: "Can not fullfil the request".to_owned(),
                    };
                }
            }
        }

        response_chain.push(tipset_bundle);

        if response_chain.len() as u64 >= request.request_len || tipset.epoch() == 0 {
            break;
        }

        curr_tipset_cids = tipset.parents().cids().to_vec();
    }

    let result_chain_length = response_chain.len() as u64;

    BlockSyncResponse {
        chain: response_chain,
        status: if result_chain_length < request.request_len {
            BlockSyncResponseStatus::PartialResponse
        } else {
            BlockSyncResponseStatus::Success
        },
        message: "Success".to_owned(),
    }
}

// Builds CompactedMessages for given Tipset.
fn compact_messages<DB>(db: &DB, tipset: &Tipset) -> Result<CompactedMessages, ChainError>
where
    DB: BlockStore,
{
    let mut bls_messages_order = HashMap::new();
    let mut secp_messages_order = HashMap::new();
    let mut bls_cids_combined: Vec<Cid> = vec![];
    let mut secp_cids_combined: Vec<Cid> = vec![];
    let mut bls_msg_includes: Vec<Vec<u64>> = vec![];
    let mut secp_msg_includes: Vec<Vec<u64>> = vec![];

    for block_header in tipset.blocks().iter() {
        let (bls_cids, secp_cids) = chain::read_msg_cids(db, block_header.messages())?;

        let mut block_include = vec![];
        let mut secp_include = vec![];

        for bls_cid in bls_cids.iter() {
            let order = match bls_messages_order.get(bls_cid) {
                Some(order) => *order,
                None => {
                    let order = block_include.len() as u64;
                    bls_cids_combined.push(bls_cid.clone());
                    bls_messages_order.insert(bls_cid.clone(), order);
                    order
                }
            };

            block_include.push(order);
        }

        for secp_cid in secp_cids.iter() {
            let order = match secp_messages_order.get(secp_cid) {
                Some(order) => *order,
                None => {
                    let order = secp_include.len() as u64;
                    secp_cids_combined.push(secp_cid.clone());
                    secp_messages_order.insert(secp_cid.clone(), order);
                    order
                }
            };

            secp_include.push(order);
        }

        bls_msg_includes.push(block_include);
        secp_msg_includes.push(secp_include);
    }

    let (bls_msgs, secp_msgs) =
        chain::block_messages_from_cids(db, &bls_cids_combined, &secp_cids_combined)?;

    Ok(CompactedMessages {
        bls_msgs,
        bls_msg_includes,
        secp_msgs,
        secp_msg_includes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::MemoryDB;
    use forest_car::load_car;
    use std::io::BufReader;

    fn populate_db() -> (Vec<Cid>, MemoryDB) {
        let db = MemoryDB::default();
        let bz = std::include_bytes!("chain_test.car");
        let reader = BufReader::<&[u8]>::new(bz.as_ref());
        // The cids are the tipset cids of the most recent tipset (39th)
        let cids: Vec<Cid> = load_car(&db, reader).unwrap();
        return (cids, db);
    }

    #[test]
    fn compact_messages_test() {
        let (cids, db) = populate_db();

        let response = make_blocksync_response(
            Arc::new(db),
            &BlockSyncRequest {
                start: cids,
                request_len: 2,
                options: BLOCKS_MESSAGES,
            },
        );

        // The response will be loaded with tipsets 39 and 38.
        // See:
        // https://filfox.info/en/tipset/39
        // https://filfox.info/en/tipset/38

        // Response chain should contain 2 tipsets.
        assert_eq!(response.chain.len(), 2);

        let tipset_39_idx = 0;
        let tipset_38_idx = 1;

        let tipset_39_compacted_messages = response.chain[tipset_39_idx].messages.as_ref().unwrap();
        let tipset_38_compacted_messages = response.chain[tipset_38_idx].messages.as_ref().unwrap();

        // Tipset at height 39...
        // ... has 22 signed messages
        assert_eq!(tipset_39_compacted_messages.secp_msgs.len(), 22);
        // ... 12 unsigned messages
        assert_eq!(tipset_39_compacted_messages.bls_msgs.len(), 12);
        // Compacted message will contain 1 secp_includes array (since only 1 block in tipset).
        assert_eq!(tipset_39_compacted_messages.secp_msg_includes.len(), 1);
        // and 1 bls_includes.
        assert_eq!(tipset_39_compacted_messages.bls_msg_includes.len(), 1);

        let tipset_39_first_block_idx = 0;
        // Secp_includes will point to all of the signed messages.
        assert_eq!(
            tipset_39_compacted_messages.secp_msg_includes[tipset_39_first_block_idx].len(),
            22
        );
        // bls_includes will point to all of the unsigned messages.
        assert_eq!(
            tipset_39_compacted_messages.bls_msg_includes[tipset_39_first_block_idx].len(),
            12
        );

        // Tipset at height 38.
        // ... has 2 blocks
        assert_eq!(response.chain[tipset_38_idx].blocks.len(), 2);
        // ... has same signed and unsigned messages across two blocks. 1 signed total
        assert_eq!(tipset_38_compacted_messages.secp_msgs.len(), 1);
        // ... 11 unsigned.
        assert_eq!(tipset_38_compacted_messages.bls_msgs.len(), 11);

        // 2 blocks exist in tipset, hence 2 secp_includes and 2 bls_includes
        assert_eq!(tipset_38_compacted_messages.secp_msg_includes.len(), 2);
        assert_eq!(tipset_38_compacted_messages.bls_msg_includes.len(), 2);

        let tipset_38_first_block_idx = 0;
        let tipset_38_second_block_idx = 1;
        // Since the messages are duplicated in blocks, each `include` will have them all
        assert_eq!(
            tipset_38_compacted_messages.secp_msg_includes[tipset_38_first_block_idx].len(),
            1
        );
        assert_eq!(
            tipset_38_compacted_messages.bls_msg_includes[tipset_38_first_block_idx].len(),
            11
        );
        assert_eq!(
            tipset_38_compacted_messages.secp_msg_includes[tipset_38_second_block_idx].len(),
            1
        );
        assert_eq!(
            tipset_38_compacted_messages.bls_msg_includes[tipset_38_second_block_idx].len(),
            11
        );
    }
}
