use super::outcome::ModelToViewMessage;
use super::UpdateUser;

use application_models::users::{UserToUpdate, UsersRepo};
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, UserToUpdate, ModelToViewMessage> for UpdateUser {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        user_to_update: UserToUpdate,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let _ = UsersRepo::update(db_conn_pool, &user_to_update).await?;

        Ok(ModelToViewMessage::new(user_to_update))
    }
}
