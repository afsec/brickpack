use super::outcome::ModelToViewMessage;
use super::CreateDepartment;
use application_models::departments::DepartmentsRepo;
use application_models::departments::NewDepartment;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, NewDepartment, ModelToViewMessage> for CreateDepartment {
    async fn model(
        &'endpoint self,
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        new_department: NewDepartment,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data =
            DepartmentsRepo::create(option_oid_pool, db_conn_pool, &new_department).await?;

        Ok(ModelToViewMessage::new(outcome_data))
    }
}
