mod model;
mod outcome;
mod presenter;
mod view;

use crate::{api::Paging, extractors::QueryInput};
use application_models::users::User;
use axum::{extract::Extension, http::StatusCode, response::Json};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

pub(crate) struct ReadUsers;
impl Endpoint for ReadUsers {}

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    QueryInput(paging): QueryInput<Paging>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    tracing::info!("Endpoint Found: ReadUsers - [GET /api/users]");

    ReadUsers::presenter(&ReadUsers, None, &sqlite_pool, paging).await
}
