mod model;
mod outcome;
mod presenter;
mod view;

use self::view::UserOidUpdated;
use crate::{
    api::REQUEST_BODY_MAX_SIZE,
    extractors::{JsonInput, PathInput},
};
use application_models::users::{model::UserOid, UserToUpdate};
use axum::{
    extract::{ContentLengthLimit, Extension},
    http::StatusCode,
    response::Json,
};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct UpdateUser;
impl Endpoint for UpdateUser {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(user_oid): PathInput<UserOid>,
    ContentLengthLimit(JsonInput(mut user_to_update)): ContentLengthLimit<
        JsonInput<UserToUpdate>,
        REQUEST_BODY_MAX_SIZE,
    >,
) -> Result<Json<UserOidUpdated>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: UpdateUser - [PATCH /api/user/:user_oid]");

    user_to_update.set_oid(user_oid);

    UpdateUser::presenter(&UpdateUser, None, sqlite_pool, user_to_update).await
}
