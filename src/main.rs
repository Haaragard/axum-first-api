mod http;
mod controller;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let mut router: axum::Router = http::routes::create_router();
    router = http::routes::register_routes(router);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
