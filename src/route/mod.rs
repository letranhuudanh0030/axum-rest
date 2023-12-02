use axum::Router;

mod cake_route;
mod login_route;

use crate::ModelManager;

pub mod route_static;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .merge(login_route::routes())
        .nest("/cake", cake_route::routes())
        .with_state(mm)
}
