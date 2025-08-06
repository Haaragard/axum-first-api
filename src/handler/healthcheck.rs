use serde::Serialize;
use axum::{
    http::StatusCode,
    Json
};

pub struct HealthCheck;
impl HealthCheck {
    pub async fn call() -> (StatusCode, Json<Response>) {
        (StatusCode::OK, Json(Response {
            message: String::from("Alive and kicking!"),
        }))
    }
}

#[derive(PartialEq, Eq, Debug, Serialize)]
pub struct Response {
    message: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_should_call_healthcheck_and_return_status_ok() {
        let result = HealthCheck::call().await;

        // Assert the status code
        assert_eq!(result.0, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_should_call_healthcheck_and_return_response_body_json() {
        let result = HealthCheck::call().await;

        let response = result
            .1
            .0;

        let expected_response = Response {
            message: String::from("Alive and kicking!"),
        };

        // Assert the status code
        assert_eq!(response, expected_response);
    }
}
