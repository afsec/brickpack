mod model;
mod outcome;
mod presenter;
mod view;

use crate::extractors::PathInput;
use application_models::departments::{model::DepartmentOid, Department};
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct ReadDepartment;
impl Endpoint for ReadDepartment {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(department_oid): PathInput<DepartmentOid>,
) -> Result<Json<Department>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: ReadDepartment - [GET /api/department/:department_oid]");

    ReadDepartment::presenter(&ReadDepartment, None, sqlite_pool, department_oid).await
}
