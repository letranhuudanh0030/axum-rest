use axum::Router;

use crate::model::AppState;

mod cake_route;
mod error;
pub mod route_static;

pub fn routes(state: AppState) -> Router {
    Router::new().with_state(state).merge(cake_route::routes())
}
