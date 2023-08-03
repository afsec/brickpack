mod model;
mod outcome;
mod presenter;
mod view;

use self::view::UserOidCreated;
use crate::{api::REQUEST_BODY_MAX_SIZE, extractors::JsonInput};
use application_models::users::NewUser;
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

struct CreateUser;
impl Endpoint for CreateUser {}

pub(super) async fn handler(
    Extension(ref oid_pool): Extension<OidPool>,
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    ContentLengthLimit(JsonInput(newuser_to_create)): ContentLengthLimit<
        JsonInput<NewUser>,
        REQUEST_BODY_MAX_SIZE,
    >,
) -> Result<Json<UserOidCreated>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: CreateUser - [POST /api/user]");
    CreateUser::presenter(&CreateUser, Some(oid_pool), sqlite_pool, newuser_to_create).await
}
