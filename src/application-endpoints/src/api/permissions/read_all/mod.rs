mod model;
mod outcome;
mod presenter;
mod view;

use crate::{api::Paging, extractors::QueryInput};
use application_models::permissions::Permission;
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct ReadPermissions;
impl Endpoint for ReadPermissions {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    QueryInput(paging): QueryInput<Paging>,
) -> Result<Json<Vec<Permission>>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: ReadPermissions - [GET /api/permissions]");

    ReadPermissions::presenter(&ReadPermissions, None, &sqlite_pool, paging).await
}
