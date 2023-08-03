mod model;
mod outcome;
mod presenter;
mod view;

use self::view::DepartmentOidDeleted;
use crate::extractors::PathInput;
use application_models::departments::model::DepartmentOid;
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct DeleteDepartment;

impl Endpoint for DeleteDepartment {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(department_oid): PathInput<DepartmentOid>,
) -> Result<Json<DepartmentOidDeleted>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: DeleteDepartment - [DELETE /api/department/:department_oid]");

    DeleteDepartment::presenter(&DeleteDepartment, None, sqlite_pool, department_oid).await
}
