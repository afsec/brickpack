use super::outcome::ModelToViewMessage;
use super::CountUsers;
use async_trait::async_trait;
use design_scaffold::endpoint::{Model, Outcome};
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

impl Outcome for ModelToViewMessage {}

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, (), ModelToViewMessage> for CountUsers {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        _: (),
    ) -> design_scaffold::Result<ModelToViewMessage> {
        use application_models::users::UsersRepo;
        let outcome_data = UsersRepo::count(db_conn_pool).await?;

        Ok(ModelToViewMessage::new(outcome_data))
    }
}
