mod batch;

use std::sync::Arc;

use crate::db::ConnectionManager;
use batch::batch_routes;
use warp::Filter;

pub fn routes(
    manager: Arc<ConnectionManager>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    batch_routes(manager)
}
