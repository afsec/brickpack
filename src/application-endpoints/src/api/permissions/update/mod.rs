mod model;
mod outcome;
mod presenter;
mod view;

use self::view::PermissionOidUpdated;
use crate::{
    api::REQUEST_BODY_MAX_SIZE,
    extractors::{JsonInput, PathInput},
};
use application_models::permissions::{model::PermissionOid, PermissionToUpdate};
use axum::{
    extract::{ContentLengthLimit, Extension},
    http::StatusCode,
    response::Json,
};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct UpdatePermission;
impl Endpoint for UpdatePermission {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(permission_oid): PathInput<PermissionOid>,
    ContentLengthLimit(JsonInput(mut permission_to_update)): ContentLengthLimit<
        JsonInput<PermissionToUpdate>,
        REQUEST_BODY_MAX_SIZE,
    >,
) -> Result<Json<PermissionOidUpdated>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: UpdatePermission - [PATCH /api/permission/:permission_oid]");

    permission_to_update.set_oid(permission_oid);

    UpdatePermission::presenter(&UpdatePermission, None, sqlite_pool, permission_to_update).await
}
