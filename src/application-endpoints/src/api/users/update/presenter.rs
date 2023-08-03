use super::outcome::ModelToViewMessage;
use super::view::UserOidUpdated;
use super::UpdateUser;
use application_models::users::UserToUpdate;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, UpdateUser, Sqlite, UserToUpdate, ModelToViewMessage, Json<UserOidUpdated>>
    for UpdateUser
{
}
