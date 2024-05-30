use std::convert::Infallible;
use r2d2::Error as R2d2Error;
use rusqlite::Error as RusqliteError;
use std::sync::Arc;
use serde_derive::Serialize;
use tracing::info;
use tracing_subscriber::EnvFilter;
use warp::{Rejection, Reply, Filter};
use warp::http::StatusCode;

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

    let routes = routes::routes(manager.clone()).recover(handle_rejection);

    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    info!("Received Rejection: {:?}", err);

    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "NOT_FOUND";
    } else if let Some(TrieCacheError::InvalidHexString) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "BAD_REQUEST_INPUTS";
    } else if let Some(TrieCacheError::BatchNotFound) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "BATCH_NOT_FOUND";
    } else if let Some(TrieCacheError::BatchParentNotFinalized) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = "PARENT_BATCH_NOT_FINALIZED";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "INTERNAL_SERVER_ERROR";
    }

    let json = warp::reply::json(&Message {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}

#[derive(Debug, Serialize)]
struct Message {
    code: u16,
    message: String,
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
