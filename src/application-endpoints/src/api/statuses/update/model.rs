use super::outcome::ModelToViewMessage;
use super::UpdateStatus;

use application_models::statuses::{StatusToUpdate, StatusesRepo};
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, StatusToUpdate, ModelToViewMessage> for UpdateStatus {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        status_to_update: StatusToUpdate,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let _ = StatusesRepo::update(db_conn_pool, &status_to_update).await?;

        Ok(ModelToViewMessage::new(status_to_update))
    }
}
