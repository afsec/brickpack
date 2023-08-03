use super::outcome::ModelToViewMessage;
use super::view::UserOidDeleted;
use super::DeleteUser;
use application_models::users::model::UserOid;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, DeleteUser, Sqlite, UserOid, ModelToViewMessage, Json<UserOidDeleted>>
    for DeleteUser
{
}
