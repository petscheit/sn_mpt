use std::sync::Arc;
use tracing_subscriber::EnvFilter;
use warp::{Filter};

mod db;
pub mod errors;
mod handlers;
mod models;
mod routes;
mod trie_cache;
use crate::db::ConnectionManager;
use crate::errors::handle_rejection;

#[tokio::main]
async fn main() {
    let manager = Arc::new(ConnectionManager::new("database.db"));
    manager.create_table().unwrap();

    let routes = routes::routes(manager.clone()).recover(handle_rejection);

    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
