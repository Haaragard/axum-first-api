use serde::{Deserialize, Serialize};
use axum::{
    http::StatusCode,
    Json
};

use axum::extract::State;

use crate::app_state::user::CreateUserState;
use crate::entity::user::User;
use crate::persistence::CreateUserInputDTO;

#[derive(Debug)]
pub struct CreateUser;
impl CreateUser {
    pub async fn call(
        State(state): State<CreateUserState>,
        payload: Json<CreateUserDTO>
    ) -> (StatusCode, Json<CreateUserResponse>) {
        let dto = CreateUserInputDTO {
            name: payload.name.clone(),
        };

        match state.repository.create_user(dto) {
            Ok(result) => {
                let response = Response { user: result.user };
                return (StatusCode::CREATED, Json(CreateUserResponse::Success(response)));
            },
            Err(error) => {
                println!("Error: {:?}", error);

                let response = ResponseError {
                    message: "Error, user not created.".into(),
                };
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateUserResponse::Error(response)));
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDTO {
    pub name: String,
}

// Enum para resposta do endpoint
#[derive(PartialEq, Eq, Debug, Serialize)]
#[serde(untagged)]
pub enum CreateUserResponse {
    Success(Response),
    Error(ResponseError),
}

// Resposta de sucesso
#[derive(PartialEq, Eq, Debug, Serialize)]
pub struct Response {
    pub user: User,
}

// Resposta de erro
#[derive(PartialEq, Eq, Debug, Serialize)]
pub struct ResponseError {
    pub message: String,
}

#[cfg(test)]
mod test {
    // use std::sync::Arc;

    // use super::*;

    #[tokio::test]
    async fn test_should_create_user_call_success() {
        todo!("Needs fix!")
    //     let create_user_dto = Json(CreateUserDTO {
    //         name: "fake-name".into(),
    //     });

    //     let create_user_state = crate::app_state::user::CreateUserState {
    //         repository: Arc::new(crate::persistence::SqliteRepository::new(
    //             ":memory:".into()
    //         )),
    //     };

    //     let (
    //         response_status_code,
    //         json_response
    //     ) = CreateUser::call(State(create_user_state), create_user_dto).await;

        // Assert status code is CREATED
        // assert_eq!(response_status_code, StatusCode::CREATED);

        // Assert Response
        // assert_eq!(
        //     json_response.0,
        //     CreateUserResponse::Success(Response {
        //         id: None,
        //         name: "fake-name".into(),
        //     })
        // );
    }

    #[tokio::test]
    async fn test_should_create_user_call_error() {
        todo!("Needs fix!")
        // let create_user_dto = Json(CreateUserDTO {
        //     name: "fake-name".into(),
        // });

        // let create_user_state = crate::app_state::user::CreateUserState {
        //     repository: Arc::new(crate::persistence::SqliteRepository::new(
        //         ":memory:".into()
        //     )),
        // };

        // let (
        //     response_status_code,
        //     json_response
        // ) = CreateUser::call(State(create_user_state), create_user_dto).await;

        // // Assert status code is CREATED
        // assert_eq!(response_status_code, StatusCode::CREATED);

        // // Assert Response
        // assert_eq!(
        //     json_response.0,
        //     CreateUserResponse::Error(ResponseError {
        //         message: "fake-message-error".into(),
        //     })
        // );
    }

    #[tokio::test]
    async fn test_should_create_response_success_with_null_id() {
        todo!("Needs fix!")
        // let response_json = serde_json::to_string(&Response {
        //     id: None,
        //     name: "fake-name".into(),
        // })
        // .unwrap();

        // assert_eq!(response_json, "{\"id\":null,\"name\":\"fake-name\"}");
    }

    #[tokio::test]
    async fn test_should_create_response_success() {
        todo!("Needs fix!")
        // let response_json = serde_json::to_string(&Response {
        //     id: Some("id-123-456-789".into()),
        //     name: "fake-name".into(),
        // })
        // .unwrap();

        // assert_eq!(response_json, "{\"id\":123,\"name\":\"fake-name\"}");
    }
}
