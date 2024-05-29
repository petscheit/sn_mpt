use std::sync::Arc;

mod db;
mod handlers;
mod models;
mod routes;
mod trie_cache;

use crate::db::ConnectionManager;
// use crate::trie_cache::proxy::TrieCacheProxy;

#[tokio::main]
async fn main() {
    let manager = Arc::new(ConnectionManager::new("database.db", false));
    manager.create_table().unwrap();

    let routes = routes::routes(manager.clone());
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
