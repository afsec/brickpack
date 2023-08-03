use super::outcome::ModelToViewMessage;
use super::DeleteDepartment;

use application_models::departments::model::DepartmentOid;
use application_models::departments::DepartmentsRepo;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, DepartmentOid, ModelToViewMessage> for DeleteDepartment {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        department_oid: DepartmentOid,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data = DepartmentsRepo::delete(db_conn_pool, &department_oid).await?;

        if &department_oid == outcome_data {
            Ok(ModelToViewMessage::from(department_oid))
        } else {
            Err(design_scaffold::Error::ImpossibleState("Id mismatch from DepartmentsRepo".into()))
        }
    }
}
