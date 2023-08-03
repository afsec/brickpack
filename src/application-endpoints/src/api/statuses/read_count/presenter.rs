use super::outcome::ModelToViewMessage;
use super::CountStatuses;
use application_models::statuses::StatusesLength;
use async_trait::async_trait;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, CountStatuses, Sqlite, (), ModelToViewMessage, StatusesLength>
    for CountStatuses
{
}
