use super::outcome::ModelToViewMessage;
use super::view::PermissionOidUpdated;
use super::UpdatePermission;
use application_models::permissions::PermissionToUpdate;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        UpdatePermission,
        Sqlite,
        PermissionToUpdate,
        ModelToViewMessage,
        Json<PermissionOidUpdated>,
    > for UpdatePermission
{
}
