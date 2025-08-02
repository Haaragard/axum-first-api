use crate::controller::{Controller, ResponseBuilder};

pub struct HealthCheck;
impl Controller for HealthCheck {
    async fn call() -> axum::response::Response {
        ResponseBuilder::json_response(
            hyper::StatusCode::OK,
            "{\"status\": true}"
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hyper::body::Body;

    #[tokio::test]
    async fn test_should_call_healthcheck_and_return_status_200() {
        let result = HealthCheck::call().await;

        // Assert the status code
        assert_eq!(result.status(), hyper::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_should_call_healthcheck_and_return_header_json() {
        let result = HealthCheck::call().await;

        // Assert the header
        assert_eq!(
            result.headers().get("Content-Type").unwrap(),
            "application/json"
        );
    }

    #[tokio::test]
    async fn test_should_call_healthcheck_and_return_body_json_status_true() {
        let result = HealthCheck::call().await;

        let body_size = result
            .body()
            .size_hint()
            .exact()
            .unwrap();
        // Extract the body as bytes and convert to String
        let body_bytes = axum::body::to_bytes(result.into_body(), body_size as usize).await.unwrap();
        assert_eq!(body_bytes.to_vec(), b"{\"status\": true}");
    }
}
