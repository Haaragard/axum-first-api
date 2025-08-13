use std::{env, sync::Arc};

use dotenv::dotenv;

use crate::persistence::{MysqlRepository, UserRepository, SqliteRepository};

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
    let user_routes = axum::Router::new()
        .route("/user", axum::routing::post(handler::user::CreateUser::call))
        .with_state(app_state::user::CreateUserState {
            user_repository: resolve_user_repository()
                .expect("Resolve user repository error.")
        });

    base_routes.merge(user_routes)
}

fn resolve_user_repository() -> Result<Arc<dyn UserRepository>, error::Error> {
    let database_type = std::env::var("DATABASE_TYPE")
        .unwrap_or("sqlite".to_string());
    let database_url = std::env::var("DATABASE_URL")
            .unwrap_or(":memory:".to_string());

    match database_type.as_str() {
        "sqlite" => {
            let repository = SqliteRepository::new(database_url)
                .expect("SQLite database connection error");
            Ok(Arc::new(repository) as Arc<dyn UserRepository>)
        },
        "mysql" => {
            let database_user = std::env::var("DATABASE_USER")
                .unwrap_or("".to_string());
            let database_password = std::env::var("DATABASE_PASSWORD")
                .unwrap_or("".to_string());
            let repository = MysqlRepository::new(
                database_url,
                database_user,
                database_password
            )
                .expect("MYSQL database connection error");
            Ok(Arc::new(repository) as Arc<dyn UserRepository>)
        },
        _ => Err(error::Error::from("Unexpected database type".to_string())),
    }
}
