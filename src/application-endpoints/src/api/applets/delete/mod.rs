mod model;
mod outcome;
mod presenter;
mod view;

use self::view::AppletDeleted;
use crate::extractors::PathInput;
use application_models::applets::model::AppletOid;
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct DeleteApplet;

impl Endpoint for DeleteApplet {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(applet_oid): PathInput<AppletOid>,
) -> Result<Json<AppletDeleted>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: DeleteApplet - [DELETE /api/applet/:applet_oid]");

    DeleteApplet::presenter(&DeleteApplet, None, sqlite_pool, applet_oid).await
}
