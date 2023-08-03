mod model;
mod outcome;
mod presenter;
mod view;

use self::view::PermissionOidCreated;
use crate::{api::REQUEST_BODY_MAX_SIZE, extractors::JsonInput};
use application_models::permissions::NewPermission;
use axum::{
    extract::{ContentLengthLimit, Extension},
    http::StatusCode,
    response::Json,
};
use design_scaffold::{
    endpoint::{Endpoint, Presenter},
    oid::OidPool,
};
use sqlx::SqlitePool;

struct CreatePermission;
impl Endpoint for CreatePermission {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    Extension(ref oid_pool): Extension<OidPool>,
    ContentLengthLimit(JsonInput(newpermission_to_create)): ContentLengthLimit<
        JsonInput<NewPermission>,
        REQUEST_BODY_MAX_SIZE,
    >,
) -> Result<Json<PermissionOidCreated>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: CreatePermission - [POST /api/permission]");

    CreatePermission::presenter(
        &CreatePermission,
        Some(oid_pool),
        sqlite_pool,
        newpermission_to_create,
    )
    .await
}
