use super::CreateStatus;
use super::{outcome::ModelToViewMessage, view::StatusOidCreated};
use application_models::statuses::NewStatus;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        CreateStatus,
        Sqlite,
        NewStatus,
        ModelToViewMessage,
        Json<StatusOidCreated>,
    > for CreateStatus
{
}
