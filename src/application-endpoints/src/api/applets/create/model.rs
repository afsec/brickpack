use super::outcome::ModelToViewMessage;
use super::CreateApplet;
use application_models::applets::AppletsRepo;
use application_models::applets::NewApplet;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, NewApplet, ModelToViewMessage> for CreateApplet {
    async fn model(
        &'endpoint self,
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        new_applet: NewApplet,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data = AppletsRepo::create(option_oid_pool, db_conn_pool, &new_applet).await?;

        Ok(ModelToViewMessage::new(outcome_data))
    }
}
