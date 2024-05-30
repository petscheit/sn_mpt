use std::sync::Arc;

use crate::db::ConnectionManager;
use crate::handlers::batch::{create_batch, fetch_batch, list_batches, update_batch_status};
use crate::models::batch::BatchStatus;
use warp::Filter;

/// Defines the routes for batch operations.
///
/// This function takes a `ConnectionManager` as input and returns a `Filter` that handles batch-related requests.
pub fn batch_routes(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    list_batches_route(manager.clone())
        .or(fetch_batch_route(manager.clone()))
        .or(create_batch_route(manager.clone()))
        .or(update_batch_status_route(manager.clone()))
}

/// Defines the route for listing batches.
///
/// This function takes a `ConnectionManager` as input and returns a `Filter` that handles GET requests to "/batches".
fn list_batches_route(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("batches")
        .and(warp::get())
        .and(with_manager(manager))
        .and_then(list_batches)
}

/// Defines the route for fetching a batch by ID.
///
/// This function takes a `ConnectionManager` as input and returns a `Filter` that handles GET requests to "/batches/{id}".
fn fetch_batch_route(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("batches" / u64)
        .and(warp::get())
        .and(with_manager(manager))
        .and_then(fetch_batch)
}

/// Defines the route for creating a new batch.
///
/// This function takes a `ConnectionManager` as input and returns a `Filter` that handles POST requests to "/batches".
fn create_batch_route(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("batches")
        .and(warp::post())
        .and(warp::body::json::<Vec<String>>())
        .and(with_manager(manager))
        .and_then(create_batch)
}

/// Defines the route for updating the status of a batch.
///
/// This function takes a `ConnectionManager` as input and returns a `Filter` that handles PUT requests to "/batches/{id}/status/{status}".
fn update_batch_status_route(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("batches" / u64 / "status" / BatchStatus)
        .and(warp::put())
        .and(with_manager(manager))
        .and_then(update_batch_status)
}

/// Helper function to pass `ConnectionManager` as a Warp filter.
///
/// This function takes a `ConnectionManager` as input and returns a `Filter` that extracts the `ConnectionManager` from the request.
fn with_manager(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = (Arc<ConnectionManager>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || manager.clone())
}
