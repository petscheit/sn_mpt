use std::sync::Arc;
use tracing::{info, instrument};
use crate::db::ConnectionManager;
use crate::models::batch::BatchStatus;
use crate::trie_cache::item::CachedItem;
use crate::trie_cache::TrieCache;
use crate::{db, TrieCacheError};
use warp::{http::StatusCode, Reply};

/// Handler for listing batches.
///
/// This function retrieves a connection from the connection manager and fetches all batches from the database.
/// It returns a JSON response containing the list of batches.
pub(crate) async fn list_batches(
    manager: Arc<ConnectionManager>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = manager.get_connection()?;
    let batches = db::batch::get_batches(&conn)?;

    Ok(warp::reply::json(&batches))
}

/// Handler for fetching a specific batch.
///
/// This function retrieves a connection from the connection manager and fetches the batch with the given ID from the database.
/// It returns a JSON response containing the batch data.
pub async fn fetch_batch(
    batch_id: u64,
    manager: Arc<ConnectionManager>,
) -> Result<impl Reply, warp::Rejection> {
    let conn = manager.get_connection()?;
    let batch = db::batch::get_batch(&conn, batch_id)?;
    Ok(warp::reply::json(&batch))
}

/// Handler for creating a new batch.
///
/// This function takes a vector of hexadecimal values and converts them into `CachedItem` objects.
/// It then creates a new batch in the database using the `TrieCache` struct and returns the resulting proofs as a JSON response.
pub async fn create_batch(
    hex_values: Vec<String>,
    manager: Arc<ConnectionManager>,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("Received new Batch!");
    let items: Vec<CachedItem> = hex_values
        .into_iter()
        .map(|hex| {
            hex::decode(hex)
                .map(CachedItem::new)
                .map_err(|_| warp::reject::custom(TrieCacheError::InvalidHexString))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let conn = manager.get_connection()?;

    let proofs = TrieCache::create_batch(&conn, items);

    Ok(warp::reply::json(&proofs.unwrap()))
}

/// Handler for updating the status of a batch.
///
/// This function retrieves a connection from the connection manager and updates the status of the batch with the given ID in the database.
/// It returns a JSON response indicating that the batch status has been updated.
pub async fn update_batch_status(
    batch_id: u64,
    new_status: BatchStatus,
    manager: Arc<ConnectionManager>,
) -> Result<impl Reply, warp::Rejection> {
    let conn = manager.get_connection()?;
    TrieCache::update_batch_status(&conn, batch_id, new_status)?;

    Ok(warp::reply::with_status(
        "Batch status updated",
        StatusCode::OK,
    ))
}

// pub async fn query_batch_proof(batch_id: u64) -> Result<impl Reply, warp::Rejection> {
//     // Logic to retrieve proof linked to the batch
//     let proof = "Example proof data"; // Replace with real data retrieval
//     Ok(warp::reply::json(&proof))
// }
