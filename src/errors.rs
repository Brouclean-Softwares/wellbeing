use crate::app::templates::UnauthorizedPage;
use crate::data::users::Profile;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use oauth2::HttpClientError;
use oauth2::basic::BasicErrorResponse;
use reqwest::Error;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    SQL(#[from] sqlx::Error),
    Request(#[from] reqwest::Error),
    TokenError(#[from] oauth2::RequestTokenError<HttpClientError<Error>, BasicErrorResponse>),
    Unauthorized(Profile),
    OptionError,
    ParseIntError(#[from] std::num::TryFromIntError),
    ParseDateError(#[from] chrono::format::ParseError),
    JsonError(#[from] serde_json::Error),
    FromRequestPartsError(#[from] std::convert::Infallible),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.log();

        match self {
            Self::SQL(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::Request(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
            Self::TokenError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::Unauthorized(profile) => {
                (StatusCode::UNAUTHORIZED, UnauthorizedPage::from(profile)).into_response()
            }
            Self::OptionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Attempted to get a non-none value but found none".to_string(),
            )
                .into_response(),
            Self::ParseIntError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::ParseDateError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::JsonError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
            Self::FromRequestPartsError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::SQL(error) => write!(f, "SQL error : {}", error),
            AppError::Request(error) => write!(f, "Request error : {}", error),
            AppError::TokenError(error) => write!(f, "Token error : {}", error),
            AppError::Unauthorized(_) => write!(f, "Unauthorized"),
            AppError::OptionError => write!(f, "OptionError"),
            AppError::ParseIntError(error) => write!(f, "ParseIntError : {}", error),
            AppError::ParseDateError(error) => write!(f, "ParseDateError : {}", error),
            AppError::JsonError(error) => write!(f, "JsonError : {}", error),
            AppError::FromRequestPartsError(error) => {
                write!(f, "FromRequestPartsError : {}", error)
            }
        }
    }
}

impl AppError {
    pub fn log(&self) {
        tracing::error!("{}", self);
    }

    pub fn log_and_redirect(&self, redirect: Redirect) -> Redirect {
        tracing::error!("{}", self);
        redirect
    }
}
