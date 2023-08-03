mod model;
mod outcome;
mod presenter;
mod view;

use self::view::PermissionOidDeleted;
use crate::extractors::PathInput;
use application_models::permissions::model::PermissionOid;
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct DeletePermission;

impl Endpoint for DeletePermission {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(permission_oid): PathInput<PermissionOid>,
) -> Result<Json<PermissionOidDeleted>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: DeletePermission - [DELETE /api/permission/:permission_oid]");

    DeletePermission::presenter(&DeletePermission, None, sqlite_pool, permission_oid).await
}
