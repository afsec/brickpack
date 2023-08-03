use super::outcome::ModelToViewMessage;
use super::ReadUser;
use application_models::users::{model::UserOid, User};
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, ReadUser, Sqlite, UserOid, ModelToViewMessage, Json<User>>
    for ReadUser
{
}
