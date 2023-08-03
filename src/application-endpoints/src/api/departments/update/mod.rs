mod model;
mod outcome;
mod presenter;
mod view;

use self::view::DepartmentOidUpdated;
use crate::{
    api::REQUEST_BODY_MAX_SIZE,
    extractors::{JsonInput, PathInput},
};
use application_models::departments::{model::DepartmentOid, DepartmentToUpdate};
use axum::{
    extract::{ContentLengthLimit, Extension},
    http::StatusCode,
    response::Json,
};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct UpdateDepartment;
impl Endpoint for UpdateDepartment {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(department_oid): PathInput<DepartmentOid>,
    ContentLengthLimit(JsonInput(mut department_to_update)): ContentLengthLimit<
        JsonInput<DepartmentToUpdate>,
        REQUEST_BODY_MAX_SIZE,
    >,
) -> Result<Json<DepartmentOidUpdated>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: UpdateDepartment - [PATCH /api/department/:department_oid]");

    department_to_update.set_oid(department_oid);

    UpdateDepartment::presenter(&UpdateDepartment, None, sqlite_pool, department_to_update).await
}
