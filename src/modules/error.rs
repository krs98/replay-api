use serde::{Serialize, Deserialize};
use serde_json::json;
use tracing::error;

use crate::infra::response;

use super::users::{
    MAX_PASSWORD_LENGTH, 
    MAX_USERNAME_LENGTH, 
    MIN_PASSWORD_LENGTH
};

// TODO: maybe use a custom deserializer?
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Sorry! Something went wrong while processing your request.")]
    Internal,
    #[error("{0} not found.")]
    NotFound(String),
    #[error("{0}")]
    InvalidArgument(String),
    #[error("{0} already exists.")]
    AlreadyExists(String),
}

impl std::convert::From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        error!("{}", err);
        match err {
            sqlx::Error::RowNotFound => Error::NotFound("Not found".into()),
            _ => Error::Internal,
        }
    }
}

impl std::convert::From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        error!("{}", err);
        Error::Internal
    }
}

impl std::convert::From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        error!("{}", err);
        Error::Internal
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        error!("{}", err);
        Error::Internal
    }
}

impl std::convert::From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        match err {
            std::env::VarError::NotPresent => Error::NotFound("Env var".into()),
            _ => Error::Internal,
        }
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::InvalidArgument(err.to_string())
    }
}

impl std::convert::From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::InvalidArgument(format!("url is not valid: {err}"))
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let msg = json!(format!("{self}"));

        match &self {
            Error::Internal => response::internal_error(msg),
            Error::NotFound(_) => response::not_found(msg),
            Error::AlreadyExists(_) => response::conflict(msg),
            Error::InvalidArgument(_) => response::bad_request(msg),
        }
        .into_response()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    Internal,

    // auth
    InvalidPassword,

    // jwt
    RefreshTokenIsNoLongerValid,

    // user
    UserAlreadyExists,
    UserNotFound,
    UsernameIsEmpty,
    UsernameTooLong,
    InvalidEmail,
    PasswordTooShort,
    PasswordTooLong,

    // login connection
    LoginConnectionAlreadyExists
}

impl std::convert::From<AppError> for Error {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Internal => Error::Internal,

            // auth
            AppError::InvalidPassword => Error::InvalidArgument(String::from("Invalid password.")),

            // jwt
            AppError::RefreshTokenIsNoLongerValid => {
                Error::InvalidArgument(String::from("Token is no longer valid."))
            }

            // user
            AppError::UserAlreadyExists => Error::AlreadyExists(String::from("User")),
            AppError::UserNotFound => Error::NotFound(String::from("User")),
            AppError::UsernameIsEmpty => {
                Error::InvalidArgument(String::from("Username cannot be empty."))
            }
            AppError::UsernameTooLong => {
                let msg =
                    format!("Username must be at most {MAX_USERNAME_LENGTH} characters long.");

                Error::InvalidArgument(msg)
            }
            AppError::InvalidEmail => Error::InvalidArgument(String::from("Invalid email.")),
            AppError::PasswordTooShort => {
                let msg =
                    format!("Password must be at least {MIN_PASSWORD_LENGTH} characters long.");

                Error::InvalidArgument(msg)
            }
            AppError::PasswordTooLong => {
                let msg =
                    format!("Password must be at most {MAX_PASSWORD_LENGTH} characters long.");

                Error::InvalidArgument(msg)
            }

            // login connection
            AppError::LoginConnectionAlreadyExists => {
                Error::AlreadyExists(String::from("Login connection"))
            }
        }
    }
}
