mod model;
mod outcome;
mod presenter;
mod view;

use crate::{api::Paging, extractors::QueryInput};
use application_models::statuses::Status;
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct ReadStatuses;
impl Endpoint for ReadStatuses {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    QueryInput(paging): QueryInput<Paging>,
) -> Result<Json<Vec<Status>>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: ReadStatuses - [GET /api/statuses]");

    ReadStatuses::presenter(&ReadStatuses, None, &sqlite_pool, paging).await
}
