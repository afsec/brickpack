mod model;
mod outcome;
mod presenter;
mod view;

use crate::extractors::PathInput;
use application_models::statuses::{model::StatusOid, Status};
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct ReadStatus;
impl Endpoint for ReadStatus {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(status_oid): PathInput<StatusOid>,
) -> Result<Json<Status>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: ReadStatus - [GET /api/status/:status_oid]");

    ReadStatus::presenter(&ReadStatus, None, sqlite_pool, status_oid).await
}
