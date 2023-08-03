use super::outcome::ModelToViewMessage;
use super::view::PermissionOidDeleted;
use super::DeletePermission;
use application_models::permissions::model::PermissionOid;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        DeletePermission,
        Sqlite,
        PermissionOid,
        ModelToViewMessage,
        Json<PermissionOidDeleted>,
    > for DeletePermission
{
}
