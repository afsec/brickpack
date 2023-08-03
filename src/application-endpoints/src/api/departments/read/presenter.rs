use super::outcome::ModelToViewMessage;
use super::ReadDepartment;
use application_models::departments::{model::DepartmentOid, Department};
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        ReadDepartment,
        Sqlite,
        DepartmentOid,
        ModelToViewMessage,
        Json<Department>,
    > for ReadDepartment
{
}
