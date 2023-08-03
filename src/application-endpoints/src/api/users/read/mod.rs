mod model;
mod outcome;
mod presenter;
mod view;

use crate::extractors::PathInput;
use application_models::users::{model::UserOid, User};
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct ReadUser;
impl Endpoint for ReadUser {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(user_oid): PathInput<UserOid>,
) -> Result<Json<User>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: ReadUser - [GET /api/user/:user_oid]");

    ReadUser::presenter(&ReadUser, None, sqlite_pool, user_oid).await
}
