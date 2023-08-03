mod model;
mod outcome;
mod presenter;
mod view;

use crate::extractors::PathInput;
use application_models::permissions::{model::PermissionOid, Permission};
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct ReadPermission;
impl Endpoint for ReadPermission {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(permission_oid): PathInput<PermissionOid>,
) -> Result<Json<Permission>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: ReadPermission - [GET /api/permission/:permission_oid]");

    ReadPermission::presenter(&ReadPermission, None, sqlite_pool, permission_oid).await
}
