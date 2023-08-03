use super::outcome::ModelToViewMessage;
use super::ReadPermission;

use application_models::permissions::model::PermissionOid;
use application_models::permissions::PermissionsRepo;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, PermissionOid, ModelToViewMessage> for ReadPermission {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        permission_oid: PermissionOid,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data = PermissionsRepo::read(db_conn_pool, &permission_oid).await?;

        Ok(ModelToViewMessage::new(outcome_data))
    }
}
