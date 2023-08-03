use super::outcome::ModelToViewMessage;
use super::view::StatusOidDeleted;
use super::DeleteStatus;
use application_models::statuses::model::StatusOid;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        DeleteStatus,
        Sqlite,
        StatusOid,
        ModelToViewMessage,
        Json<StatusOidDeleted>,
    > for DeleteStatus
{
}
