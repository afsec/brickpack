use super::outcome::ModelToViewMessage;
use super::ReadDepartment;

use application_models::departments::model::DepartmentOid;
use application_models::departments::DepartmentsRepo;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, DepartmentOid, ModelToViewMessage> for ReadDepartment {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        department_oid: DepartmentOid,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data = DepartmentsRepo::read(db_conn_pool, &department_oid).await?;

        Ok(ModelToViewMessage::new(outcome_data))
    }
}
