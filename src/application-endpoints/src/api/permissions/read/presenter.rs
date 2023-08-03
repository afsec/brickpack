use super::outcome::ModelToViewMessage;
use super::ReadPermission;
use application_models::permissions::{model::PermissionOid, Permission};
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        ReadPermission,
        Sqlite,
        PermissionOid,
        ModelToViewMessage,
        Json<Permission>,
    > for ReadPermission
{
}
