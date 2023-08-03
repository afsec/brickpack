mod model;
mod outcome;
mod presenter;
mod view;

use self::view::StatusOidCreated;
use crate::{api::REQUEST_BODY_MAX_SIZE, extractors::JsonInput};
use application_models::statuses::NewStatus;
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

struct CreateStatus;
impl Endpoint for CreateStatus {}

pub(super) async fn handler(
    Extension(ref oid_pool): Extension<OidPool>,
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    ContentLengthLimit(JsonInput(newstatus_to_create)): ContentLengthLimit<
        JsonInput<NewStatus>,
        REQUEST_BODY_MAX_SIZE,
    >,
) -> Result<Json<StatusOidCreated>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: CreateStatus - [POST /api/status]");
    CreateStatus::presenter(&CreateStatus, Some(oid_pool), sqlite_pool, newstatus_to_create).await
}
