use super::outcome::ModelToViewMessage;
use super::ReadStatus;

use application_models::statuses::model::StatusOid;
use application_models::statuses::StatusesRepo;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, StatusOid, ModelToViewMessage> for ReadStatus {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        status_oid: StatusOid,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data = StatusesRepo::read(db_conn_pool, &status_oid).await?;

        Ok(ModelToViewMessage::new(outcome_data))
    }
}
