use super::outcome::ModelToViewMessage;
use super::DeleteStatus;

use application_models::statuses::model::StatusOid;
use application_models::statuses::StatusesRepo;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, StatusOid, ModelToViewMessage> for DeleteStatus {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        status_oid: StatusOid,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data = StatusesRepo::delete(db_conn_pool, &status_oid).await?;
        if &status_oid == outcome_data {
            Ok(ModelToViewMessage::new(status_oid))
        } else {
            Err(design_scaffold::Error::ImpossibleState("Id mismatch from StatusesRepo".into()))
        }
    }
}
