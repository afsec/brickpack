use super::outcome::ModelToViewMessage;
use super::ReadStatuses;
use crate::api::Paging;
use application_models::statuses::StatusesRepo;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, Paging, ModelToViewMessage> for ReadStatuses {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        submitted_data: Paging,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data = StatusesRepo::read_all(
            db_conn_pool,
            submitted_data.get_limit(),
            submitted_data.get_offset(),
        )
        .await?;

        Ok(ModelToViewMessage::new(outcome_data))
    }
}
