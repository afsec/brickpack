mod model;
mod outcome;
mod presenter;
mod view;

use self::view::UserOidDeleted;
use crate::extractors::PathInput;
use application_models::users::model::UserOid;
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct DeleteUser;

impl Endpoint for DeleteUser {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(user_oid): PathInput<UserOid>,
) -> Result<Json<UserOidDeleted>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: DeleteUser - [DELETE /api/user/:user_oid]");

    DeleteUser::presenter(&DeleteUser, None, sqlite_pool, user_oid).await
}
