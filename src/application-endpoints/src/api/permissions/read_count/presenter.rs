use super::outcome::ModelToViewMessage;
use super::CountPermissions;
use application_models::permissions::PermissionsLength;
use async_trait::async_trait;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, CountPermissions, Sqlite, (), ModelToViewMessage, PermissionsLength>
    for CountPermissions
{
}
