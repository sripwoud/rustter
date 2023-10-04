use crate::State;
use axum::routing::get;
use axum::Router;

pub fn new_router(state: State) -> axum::Router {
    let public_routes = Router::new().route("/", get(move || async { "this is the root route" }));
    let authorized_routes = Router::new();

    Router::new().merge(public_routes).merge(authorized_routes)
}
