use std::{env, sync::Arc};

use dotenv::dotenv;

mod error;
mod app_state;
mod persistence;
mod handler;
mod entity;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // build our application with a single route
    let router: axum::Router = create_router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", app_port)).await.unwrap();

    println!("App serving into port: {}", app_port);

    axum::serve(listener, router).await.unwrap();
}

pub fn create_router() -> axum::Router<()> {
    let base_routes = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, Axum" }))
        .route("/healthcheck", axum::routing::get(handler::healthcheck::HealthCheck::call));

    let user_repository = Arc::new(
        crate::persistence::SqliteRepository::new(
            std::env::var("DATABASE_URL")
                .unwrap_or(":memory:".to_string())
        )
            .expect("Could not connect into database")
    );
    let user_routes = axum::Router::new()
        .route("/user", axum::routing::post(handler::user::CreateUser::call))
        .with_state(app_state::user::CreateUserState {
            repository: Arc::clone(&user_repository)
        });

    base_routes.merge(user_routes)
}
