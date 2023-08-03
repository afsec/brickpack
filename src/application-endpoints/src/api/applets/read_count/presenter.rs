use super::outcome::ModelToViewMessage;
use super::CountApplets;
use application_models::applets::AppletsLength;
use async_trait::async_trait;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, CountApplets, Sqlite, (), ModelToViewMessage, AppletsLength>
    for CountApplets
{
}
