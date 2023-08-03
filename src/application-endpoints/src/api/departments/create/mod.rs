mod model;
mod outcome;
mod presenter;
mod view;

use self::view::DepartmentOidCreated;
use crate::{api::REQUEST_BODY_MAX_SIZE, extractors::JsonInput};
use application_models::departments::NewDepartment;
use axum::{
    extract::{ContentLengthLimit, Extension},
    http::StatusCode,
    response::Json,
};
use design_scaffold::{
    endpoint::{Endpoint, Presenter},
    oid::OidPool,
};
use sqlx::SqlitePool;

struct CreateDepartment;
impl Endpoint for CreateDepartment {}

pub(super) async fn handler(
    Extension(ref oid_pool): Extension<OidPool>,
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    ContentLengthLimit(JsonInput(newdepartment_to_create)): ContentLengthLimit<
        JsonInput<NewDepartment>,
        REQUEST_BODY_MAX_SIZE,
    >,
) -> Result<Json<DepartmentOidCreated>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: CreateDepartment - [POST /api/department]");

    CreateDepartment::presenter(
        &CreateDepartment,
        Some(oid_pool),
        sqlite_pool,
        newdepartment_to_create,
    )
    .await
}
