use std::{
    convert::Infallible,
    fmt::Display,
    num::{ParseIntError, TryFromIntError},
    string::FromUtf8Error,
};

use axum::http::StatusCode;
use thiserror::Error;
use ulid::{EncodeError, MonotonicError};

#[derive(Error, Debug)]
pub enum BrickpackError {
    DataValidation(String),
    DataConversion(String),
    SqlxError(#[from] sqlx::Error),
    EntityIdNotFound(String),
    InfallibleError(#[from] Infallible),
    TryFromInt(#[from] TryFromIntError),
    ParseInt(#[from] ParseIntError),
    UlidEncodeError(#[from] EncodeError),
    UlidMonotonicError(#[from] MonotonicError),
    MutexError(String),
    ImpossibleState(String),
    SerdeQsError(#[from] serde_qs::Error),
    MluaError(#[from] mlua::Error),
    AppletNewCodeValidation(String),
    AppletRunnerValidation(String),
    Base64DecodeError(#[from] base64::DecodeError),
    FromUtf8(#[from] FromUtf8Error),
    Unknown,
}

impl Display for BrickpackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: String = match &*self {
            BrickpackError::DataValidation(_)
            | BrickpackError::DataConversion(_)
            | BrickpackError::InfallibleError(_)
            | BrickpackError::AppletRunnerValidation(_)
            | BrickpackError::TryFromInt(_)
            | BrickpackError::ParseInt(_)
            | BrickpackError::Base64DecodeError(_)
            | BrickpackError::UlidEncodeError(_)
            | BrickpackError::FromUtf8(_) => "Datastore invalid data".into(),
            BrickpackError::AppletNewCodeValidation(error) => {
                tracing::warn!("Error on add a new lua code. Reason: {error:?}");
                "Invalid code: Lua syntax error".into()
            }
            BrickpackError::SqlxError(error) => match &*error {
                sqlx::Error::RowNotFound => "Datastore No data".into(),
                sqlx::Error::ColumnIndexOutOfBounds { .. } => "Invalid search parameter".into(),
                sqlx::Error::PoolTimedOut => "Datastore Internal Error".into(),
                sqlx::Error::Configuration(_)
                | sqlx::Error::Database(_)
                | sqlx::Error::Io(_)
                | sqlx::Error::Tls(_)
                | sqlx::Error::Protocol(_)
                | sqlx::Error::TypeNotFound { .. }
                | sqlx::Error::ColumnNotFound(_)
                | sqlx::Error::ColumnDecode { .. }
                | sqlx::Error::Decode(_)
                | sqlx::Error::PoolClosed
                | sqlx::Error::WorkerCrashed
                | sqlx::Error::Migrate(_) => "Datastore internal error".into(),
                _ => "Datastore impossible state".into(),
            },
            BrickpackError::EntityIdNotFound(_) => "No data".into(),
            BrickpackError::ImpossibleState(_) => "ImpossibleState".into(),
            BrickpackError::SerdeQsError(_) => "Invalid input".into(),
            BrickpackError::MluaError(_) => "Applet runner error".into(),
            BrickpackError::UlidMonotonicError(_) => "Impossible to generate Oid".into(),
            BrickpackError::MutexError(_) => "Impossible to access Oid generator".into(),
            BrickpackError::Unknown => "Unknown".into(),
        };
        write!(f, "{message}")
    }
}

impl Into<StatusCode> for BrickpackError {
    fn into(self) -> StatusCode {
        match self {
            BrickpackError::DataValidation(_) => StatusCode::BAD_REQUEST,
            BrickpackError::DataConversion(_) => StatusCode::CONFLICT,
            BrickpackError::SqlxError(_) => StatusCode::CONFLICT,
            BrickpackError::EntityIdNotFound(_) => StatusCode::BAD_REQUEST,
            BrickpackError::InfallibleError(_) => StatusCode::CONFLICT,
            BrickpackError::TryFromInt(_) => StatusCode::CONFLICT,
            BrickpackError::ParseInt(_) => StatusCode::CONFLICT,
            BrickpackError::ImpossibleState(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BrickpackError::SerdeQsError(_) => StatusCode::CONFLICT,
            BrickpackError::MluaError(_) => StatusCode::CONFLICT,
            BrickpackError::AppletNewCodeValidation(_) => StatusCode::BAD_REQUEST,
            BrickpackError::AppletRunnerValidation(_) => StatusCode::CONFLICT,
            BrickpackError::UlidMonotonicError(_) => StatusCode::CONFLICT,
            BrickpackError::MutexError(_) => StatusCode::CONFLICT,
            BrickpackError::Base64DecodeError(_) => StatusCode::CONFLICT,
            BrickpackError::UlidEncodeError(_) => StatusCode::CONFLICT,
            BrickpackError::FromUtf8(_) => StatusCode::CONFLICT,
            BrickpackError::Unknown => StatusCode::NOT_IMPLEMENTED,
        }
    }
}
