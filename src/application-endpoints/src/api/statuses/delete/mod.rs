mod model;
mod outcome;
mod presenter;
mod view;

use self::view::StatusOidDeleted;
use crate::extractors::PathInput;
use application_models::statuses::model::StatusOid;
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct DeleteStatus;

impl Endpoint for DeleteStatus {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(status_oid): PathInput<StatusOid>,
) -> Result<Json<StatusOidDeleted>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: DeleteStatus - [DELETE /api/status/:status_oid]");

    DeleteStatus::presenter(&DeleteStatus, None, sqlite_pool, status_oid).await
}
