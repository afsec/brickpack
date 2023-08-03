mod model;
mod outcome;
mod presenter;
mod view;

use self::view::StatusOidUpdated;
use crate::{
    api::REQUEST_BODY_MAX_SIZE,
    extractors::{JsonInput, PathInput},
};
use application_models::statuses::{model::StatusOid, StatusToUpdate};
use axum::{
    extract::{ContentLengthLimit, Extension},
    http::StatusCode,
    response::Json,
};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct UpdateStatus;
impl Endpoint for UpdateStatus {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(status_oid): PathInput<StatusOid>,
    ContentLengthLimit(JsonInput(mut status_to_update)): ContentLengthLimit<
        JsonInput<StatusToUpdate>,
        REQUEST_BODY_MAX_SIZE,
    >,
) -> Result<Json<StatusOidUpdated>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: UpdateStatus - [PATCH /api/status/:id]");

    status_to_update.set_oid(status_oid);

    UpdateStatus::presenter(&UpdateStatus, None, sqlite_pool, status_to_update).await
}
