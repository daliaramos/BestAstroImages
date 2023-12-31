use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::json;
use sqlx::Error;

#[derive(Debug)]
pub enum AppError {
    Question(QuestionError),
    Database(sqlx::Error),
    MissingCredentials,
    InvalidPassword,
    AccountBanned,
    UserDoesNotExist,
    UserAlreadyExists,
    InvalidToken,
    InternalServerError,

    #[allow(dead_code)]
    Any(anyhow::Error)
}

#[derive(derive_more::Display, Debug)]
>>>>>>> roles
pub enum QuestionError {
    InvalidId,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Question(err) => match err {
                QuestionError::InvalidId => (StatusCode::NOT_FOUND, err.to_string()),
            },

            AppError::Database(err) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            AppError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Your credentials where missing or incorrect".to_string()),
            AppError::InvalidPassword => (StatusCode::UNAUTHORIZED, "Invalid Password".to_string()),
            AppError::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "Your account does not exist".to_string()),
            AppError::AccountBanned => (StatusCode::UNAUTHORIZED, "Your account has violated our site policy and has been banned".to_string()),
            AppError::UserAlreadyExists => (StatusCode::UNAUTHORIZED, "There is already an account with that email address".to_string()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token".to_string()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Something horrible happened".to_string()),

            AppError::Any(err) => {
                let message = format!("Internal server error! {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
        };

        let body = Json(json!({"error": error_message}));
        (status, body).into_response()
    }
}


impl From<sqlx::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::Database(value)
    }
}

