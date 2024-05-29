use r2d2::Error as R2d2Error;
use rusqlite::Error as RusqliteError;
use std::sync::Arc;

mod db;
mod handlers;
mod models;
mod routes;
mod trie_cache;

use crate::db::ConnectionManager;

#[tokio::main]
async fn main() {
    let manager = Arc::new(ConnectionManager::new("database.db", false));
    manager.create_table().unwrap();

    let routes = routes::routes(manager.clone());
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Debug)]
pub enum TrieCacheError {
    DatabaseConnectionError(R2d2Error),
    DatabaseOperationError(RusqliteError),
    InvalidBatchStatus,
    BatchNotFound,
    ProofGenerationError,
    TrieWriteError,
    NodeEncodingError,
    NodeNotFound,
    ArbitraryError(anyhow::Error),
    BatchParentNotFinalized,
    InvalidHexString,
}

impl warp::reject::Reject for TrieCacheError {}
impl From<R2d2Error> for TrieCacheError {
    fn from(err: R2d2Error) -> Self {
        TrieCacheError::DatabaseConnectionError(err)
    }
}

impl From<RusqliteError> for TrieCacheError {
    fn from(err: RusqliteError) -> Self {
        TrieCacheError::DatabaseOperationError(err)
    }
}

impl From<anyhow::Error> for TrieCacheError {
    fn from(err: anyhow::Error) -> Self {
        TrieCacheError::ArbitraryError(err)
    }
}
