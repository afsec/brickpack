use super::CreateDepartment;
use super::{outcome::ModelToViewMessage, view::DepartmentOidCreated};
use application_models::departments::NewDepartment;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        CreateDepartment,
        Sqlite,
        NewDepartment,
        ModelToViewMessage,
        Json<DepartmentOidCreated>,
    > for CreateDepartment
{
}
