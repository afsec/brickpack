mod model;
mod outcome;
mod presenter;
mod view;

use crate::{api::Paging, extractors::QueryInput};
use application_models::departments::Department;
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct ReadDepartments;
impl Endpoint for ReadDepartments {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    QueryInput(paging): QueryInput<Paging>,
) -> Result<Json<Vec<Department>>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: ReadDepartments - [GET /api/departments]");

    ReadDepartments::presenter(&ReadDepartments, None, &sqlite_pool, paging).await
}
