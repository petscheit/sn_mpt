use std::sync::Arc;

use crate::db::ConnectionManager;
use crate::handlers::batch::{create_batch, fetch_batch, list_batches, update_batch_status};
use crate::models::batch::BatchStatus;
use warp::Filter;

pub fn batch_routes(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    list_batches_route(manager.clone())
        .or(fetch_batch_route(manager.clone()))
        .or(create_batch_route(manager.clone()))
        .or(update_batch_status_route(manager.clone()))
}

fn list_batches_route(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("batches")
        .and(warp::get())
        .and(with_manager(manager))
        .and_then(list_batches)
}

fn fetch_batch_route(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("batches" / u64)
        .and(warp::get())
        .and(with_manager(manager))
        .and_then(fetch_batch)
}

fn create_batch_route(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("batches")
        .and(warp::post())
        .and(warp::body::json::<Vec<String>>())
        .and(with_manager(manager))
        .and_then(create_batch)
}

fn update_batch_status_route(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("batches" / u64 / "status" / BatchStatus)
        .and(warp::put())
        .and(with_manager(manager))
        .and_then(update_batch_status)
}

// Helper function to pass ConnectionManager as a Warp filter
fn with_manager(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = (Arc<ConnectionManager>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}
