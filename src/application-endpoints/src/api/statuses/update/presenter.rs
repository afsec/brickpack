use super::outcome::ModelToViewMessage;
use super::view::StatusOidUpdated;
use super::UpdateStatus;
use application_models::statuses::StatusToUpdate;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        UpdateStatus,
        Sqlite,
        StatusToUpdate,
        ModelToViewMessage,
        Json<StatusOidUpdated>,
    > for UpdateStatus
{
}
