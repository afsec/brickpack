use super::outcome::ModelToViewMessage;
use super::CountUsers;
use application_models::users::UsersLength;
use async_trait::async_trait;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, CountUsers, Sqlite, (), ModelToViewMessage, UsersLength>
    for CountUsers
{
}
