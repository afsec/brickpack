use super::outcome::ModelToViewMessage;
use super::CountDepartments;
use application_models::departments::DepartmentsLength;
use async_trait::async_trait;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, CountDepartments, Sqlite, (), ModelToViewMessage, DepartmentsLength>
    for CountDepartments
{
}
