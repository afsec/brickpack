use super::CreateUser;
use super::{outcome::ModelToViewMessage, view::UserOidCreated};
use application_models::users::NewUser;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, CreateUser, Sqlite, NewUser, ModelToViewMessage, Json<UserOidCreated>>
    for CreateUser
{
}
