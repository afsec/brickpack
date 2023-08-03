mod model;
mod outcome;
mod presenter;
mod view;

use self::view::AppletIdUpdated;
use crate::{
    api::REQUEST_BODY_MAX_SIZE,
    extractors::{JsonInput, PathInput},
};
use application_models::applets::{model::AppletOid, AppletToUpdate};
use axum::{
    extract::{ContentLengthLimit, Extension},
    http::StatusCode,
    response::Json,
};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct UpdateApplet;
impl Endpoint for UpdateApplet {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(applet_oid): PathInput<AppletOid>,
    ContentLengthLimit(JsonInput(mut applet_to_update)): ContentLengthLimit<
        JsonInput<AppletToUpdate>,
        REQUEST_BODY_MAX_SIZE,
    >,
) -> Result<Json<AppletIdUpdated>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: UpdateApplet - [PATCH /api/applet/:applet_oid]");

    applet_to_update.set_oid(applet_oid);

    UpdateApplet::presenter(&UpdateApplet, None, sqlite_pool, applet_to_update).await
}
