use std::sync::Arc;

use axum::routing::{get, post};

use crate::{app_state, handler};

pub fn create_router() -> axum::Router<()> {
    let user_repository = Arc::new(crate::persistence::SqliteRepository::new(":memory".into()));

    let router = axum::Router::new()
        .route("/", get(|| async { "Hello, Axum" }))
        .route("/healthcheck", get(handler::healthcheck::HealthCheck::call));
        
    let router_user = axum::Router::new()
        .route("/user", post(handler::user::CreateUser::call))
        .with_state(app_state::user::CreateUserState {
            repository: Arc::clone(&user_repository)
        });
    
    router.merge(router_user)
}
