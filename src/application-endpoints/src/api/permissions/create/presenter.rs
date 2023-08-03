use super::CreatePermission;
use super::{outcome::ModelToViewMessage, view::PermissionOidCreated};
use application_models::permissions::NewPermission;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        CreatePermission,
        Sqlite,
        NewPermission,
        ModelToViewMessage,
        Json<PermissionOidCreated>,
    > for CreatePermission
{
}
