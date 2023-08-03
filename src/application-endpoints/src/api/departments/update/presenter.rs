use super::outcome::ModelToViewMessage;
use super::view::DepartmentOidUpdated;
use super::UpdateDepartment;
use application_models::departments::DepartmentToUpdate;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        UpdateDepartment,
        Sqlite,
        DepartmentToUpdate,
        ModelToViewMessage,
        Json<DepartmentOidUpdated>,
    > for UpdateDepartment
{
}
