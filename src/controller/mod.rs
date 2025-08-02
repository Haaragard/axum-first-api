pub mod healthcheck;

pub trait Controller {
    async fn call() -> axum::response::Response;
}

struct ResponseBuilder;
impl ResponseBuilder {
    pub fn json_response(status_code: axum::http::StatusCode, content: &str) -> axum::response::Response {
        axum::response::Response::builder()
            .header("Content-Type", "application/json")
            .status(status_code)
            .body(axum::body::Body::from(content.to_owned()))
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_should_implement_controller_trait() {
        pub struct TestController;
        impl Controller for TestController {
            async fn call() -> axum::response::Response {
                axum::response::Response::builder()
                    .status(hyper::StatusCode::OK)
                    .body(axum::body::Body::empty())
                    .unwrap()
            }
        }

        let result = TestController::call().await;
        
        assert_eq!(result.status(), axum::http::StatusCode::OK);
    }
}
