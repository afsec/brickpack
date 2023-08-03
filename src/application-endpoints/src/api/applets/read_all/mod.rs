mod model;
mod outcome;
mod presenter;
mod view;

use crate::{api::Paging, extractors::QueryInput};
use application_models::applets::AppletMetadata;
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct ReadApplets;
impl Endpoint for ReadApplets {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    QueryInput(paging): QueryInput<Paging>,
) -> Result<Json<Vec<AppletMetadata>>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: ReadApplets - [GET /api/applets]");

    ReadApplets::presenter(&ReadApplets, None, &sqlite_pool, paging).await
}
