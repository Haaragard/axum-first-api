mod error;

mod route;
mod app_state;

mod persistence;

mod handler;
mod entity;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let router: axum::Router = route::create_router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
