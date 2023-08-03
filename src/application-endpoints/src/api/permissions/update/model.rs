use super::outcome::ModelToViewMessage;
use super::UpdatePermission;

use application_models::permissions::{PermissionToUpdate, PermissionsRepo};
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, PermissionToUpdate, ModelToViewMessage>
    for UpdatePermission
{
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        permission_to_update: PermissionToUpdate,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let _ = PermissionsRepo::update(db_conn_pool, &permission_to_update).await?;

        Ok(ModelToViewMessage::new(permission_to_update))
    }
}
