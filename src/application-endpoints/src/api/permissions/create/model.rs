use super::outcome::ModelToViewMessage;
use super::CreatePermission;
use application_models::permissions::NewPermission;
use application_models::permissions::PermissionsRepo;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, NewPermission, ModelToViewMessage> for CreatePermission {
    async fn model(
        &'endpoint self,
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        new_permission: NewPermission,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data =
            PermissionsRepo::create(option_oid_pool, db_conn_pool, &new_permission).await?;

        Ok(ModelToViewMessage::new(outcome_data))
    }
}
