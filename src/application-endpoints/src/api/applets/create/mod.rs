mod model;
mod outcome;
mod presenter;
mod view;

use self::view::AppletCreated;
use crate::{api::REQUEST_BODY_MAX_SIZE, extractors::JsonInput};
use application_models::applets::NewApplet;

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

struct CreateApplet;
impl Endpoint for CreateApplet {}

pub(super) async fn handler(
    Extension(ref oid_pool): Extension<OidPool>,
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    ContentLengthLimit(JsonInput(newapplet_to_create)): ContentLengthLimit<
        JsonInput<NewApplet>,
        REQUEST_BODY_MAX_SIZE,
    >,
) -> Result<Json<AppletCreated>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: CreateApplet - [POST /api/applet]");
    CreateApplet::presenter(&CreateApplet, Some(oid_pool), sqlite_pool, newapplet_to_create).await
}
