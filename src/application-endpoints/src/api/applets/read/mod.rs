mod model;
mod outcome;
mod presenter;
mod view;

use crate::extractors::PathInput;
use application_models::applets::{model::AppletOid, Applet};
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct ReadApplet;
impl Endpoint for ReadApplet {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(applet_oid): PathInput<AppletOid>,
) -> Result<Json<Applet>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: ReadApplet - [GET /api/applet/:applet_oid]");

    ReadApplet::presenter(&ReadApplet, None, sqlite_pool, applet_oid).await
}
