use std::sync::Arc;
use pathfinder_common::hash::{FeltHash, PoseidonHash};
use tokio::sync::Mutex;

mod items;
mod routes;
mod handlers;
mod models;
mod db;
mod trie_cache;

use warp::Filter;
use crate::db::ConnectionManager;
// use crate::trie_cache::proxy::TrieCacheProxy;

#[tokio::main]
async fn main() {
    let manager = Arc::new(ConnectionManager::new("database.db", false));
    manager.create_table().unwrap();

    let routes = routes::routes(manager.clone());
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
