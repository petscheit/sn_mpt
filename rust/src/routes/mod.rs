
mod batch;

use std::sync::Arc;
use pathfinder_common::hash::FeltHash;
use tokio::sync::Mutex;
use batch::batch_routes;
use warp::Filter;
use crate::db::ConnectionManager;
use crate::trie_cache::TrieCache;

pub fn routes(manager: Arc<ConnectionManager>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    batch_routes(manager)
}