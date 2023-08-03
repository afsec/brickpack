use super::outcome::ModelToViewMessage;
use super::ReadStatus;
use application_models::statuses::{model::StatusOid, Status};
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, ReadStatus, Sqlite, StatusOid, ModelToViewMessage, Json<Status>>
    for ReadStatus
{
}
