use crate::controller::{Controller, healthcheck::HealthCheck};

pub fn create_router() -> axum::Router {
    axum::Router::new()
}

pub fn register_routes(router: axum::Router) -> axum::Router {
    router
        .route("/", axum::routing::get(|| async { "Hello, Axum" }))
        .route("/healthcheck", axum::routing::get(HealthCheck::call))
}
