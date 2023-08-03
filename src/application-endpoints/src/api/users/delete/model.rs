use super::outcome::ModelToViewMessage;
use super::DeleteUser;

use application_models::users::model::UserOid;
use application_models::users::UsersRepo;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, UserOid, ModelToViewMessage> for DeleteUser {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        user_oid: UserOid,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data = UsersRepo::delete(db_conn_pool, &user_oid).await?;
        if &user_oid == outcome_data {
            Ok(ModelToViewMessage::new(user_oid))
        } else {
            Err(design_scaffold::Error::ImpossibleState("Id mismatch from UsersRepo".into()))
        }
    }
}
