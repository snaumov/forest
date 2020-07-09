// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use crate::State;

use blockstore::BlockStore;
use cid::json::CidJson;
use jsonrpc_v2::{Data, Error as JsonRpcError, Params};

// TODO implement Query and Batch operations

/// Get retrieves the object `value` named by `key`.
/// Get will return ErrNotFound if the key is not mapped to a value.
pub(crate) async fn get<DB>(
    _data: Data<State<DB>>,
    Params(_params): Params<(CidJson,)>,
) -> Result<String, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
{
    unimplemented!()
}

/// Has returns whether the `key` is mapped to a `value`.
/// In some contexts, it may be much cheaper only to check for existence of
/// a value, rather than retrieving the value itself. (e.g. HTTP HEAD).
pub(crate) async fn _has<DB>(
    _data: Data<State<DB>>,
    Params(_params): Params<(CidJson,)>,
) -> Result<String, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
{
    unimplemented!()
}

/// GetSize returns the size of the `value` named by `key`.
/// In some contexts, it may be much cheaper to only get the size of the
/// value rather than retrieving the value itself.
pub(crate) async fn _get_size<DB>(
    _data: Data<State<DB>>,
    Params(_params): Params<(CidJson,)>,
) -> Result<String, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
{
    unimplemented!()
}

/// Put stores the object `value` named by `key`.
///
/// The generalized Datastore interface does not impose a value type,
/// allowing various datastore middleware implementations (which do not
/// handle the values directly) to be composed together.
///
/// Ultimately, the lowest-level datastore will need to do some value checking
/// or risk getting incorrect values. It may also be useful to expose a more
/// type-safe interface to your application, and do the checking up-front.
pub(crate) async fn _put<DB>(
    _data: Data<State<DB>>,
    Params(_params): Params<(CidJson,)>,
) -> Result<String, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
{
    unimplemented!()
}

/// Delete removes the value for given `key`. If the key is not in the
/// datastore, this method returns no error.
pub(crate) async fn _delete<DB>(
    _data: Data<State<DB>>,
    Params(_params): Params<(CidJson,)>,
) -> Result<String, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
{
    unimplemented!()
}

// TODO update comment

/// Query searches the datastore and returns a query result. This function
/// may return before the query actually runs. To wait for the query:
///
///   result, _ := ds.Query(q)
///
///   use the channel interface; result may come in at different times
///   for entry := range result.Next() { ... }
///
///   or wait for the query to be completely done
///   entries, _ := result.Rest()
///   for entry := range entries { ... }
///
pub(crate) async fn _query<DB>(
    _data: Data<State<DB>>,
    Params(_params): Params<(CidJson,)>,
) -> Result<String, JsonRpcError>
where
    DB: BlockStore + Send + Sync + 'static,
{
    unimplemented!()
}
