use super::outcome::ModelToViewMessage;
use super::view::DepartmentOidDeleted;
use super::DeleteDepartment;
use application_models::departments::model::DepartmentOid;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        DeleteDepartment,
        Sqlite,
        DepartmentOid,
        ModelToViewMessage,
        Json<DepartmentOidDeleted>,
    > for DeleteDepartment
{
}
