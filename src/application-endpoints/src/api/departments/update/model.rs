use super::outcome::ModelToViewMessage;
use super::UpdateDepartment;

use application_models::departments::{DepartmentToUpdate, DepartmentsRepo};
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, DepartmentToUpdate, ModelToViewMessage>
    for UpdateDepartment
{
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        department_to_update: DepartmentToUpdate,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let _ = DepartmentsRepo::update(db_conn_pool, &department_to_update).await?;

        Ok(ModelToViewMessage::new(department_to_update))
    }
}
