use std::sync::Arc;

use crate::db;
use crate::db::ConnectionManager;
use crate::models::batch::BatchStatus;
use crate::trie_cache::item::CachedItem;
use crate::trie_cache::TrieCache;
use warp::{http::StatusCode, Reply};
//
// pub async fn list_batches(db: Arc<DBConn>) -> Result<impl Reply, warp::Rejection> {
//     let batches = db.batch.get_batches()?;
//     Ok(warp::reply::json(&batches))
// }

pub(crate) async fn list_batches(
    manager: Arc<ConnectionManager>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = manager.get_connection().unwrap();
    let batches = db::batch::get_batches(&conn).unwrap();

    Ok(warp::reply::json(&batches))
}

pub async fn fetch_batch(
    batch_id: u64,
    manager: Arc<ConnectionManager>,
) -> Result<impl Reply, warp::Rejection> {
    let conn = manager.get_connection().unwrap();
    let batch = db::batch::get_batch(&conn, batch_id).unwrap();
    Ok(warp::reply::json(&batch))
}

pub async fn create_batch(
    hex_values: Vec<String>,
    manager: Arc<ConnectionManager>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Convert each hex string to Vec<u8>
    let items: Vec<CachedItem> = hex_values
        .into_iter()
        .map(|hex| hex::decode(hex).map(CachedItem::new).unwrap())
        .collect();

    let conn = manager.get_connection().unwrap();

    let proofs = TrieCache::create_batch(&conn, items);
    println!("{:?}", proofs);

    Ok(warp::reply::json(&proofs.unwrap()))
}

pub async fn update_batch_status(
    batch_id: u64,
    new_status: BatchStatus,
    manager: Arc<ConnectionManager>,
) -> Result<impl Reply, warp::Rejection> {
    let conn = manager.get_connection().unwrap();
    TrieCache::update_batch_status(&conn, batch_id, new_status).unwrap();

    Ok(warp::reply::with_status(
        "Batch status updated",
        StatusCode::OK,
    ))
}
//
// pub async fn query_batch_proof(batch_id: u64) -> Result<impl Reply, warp::Rejection> {
//     // Logic to retrieve proof linked to the batch
//     let proof = "Example proof data"; // Replace with real data retrieval
//     Ok(warp::reply::json(&proof))
// }
