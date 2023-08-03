use super::outcome::ModelToViewMessage;
use super::DeleteApplet;

use application_models::applets::model::AppletOid;
use application_models::applets::AppletsRepo;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, AppletOid, ModelToViewMessage> for DeleteApplet {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        applet_oid: AppletOid,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let outcome_data = AppletsRepo::delete(db_conn_pool, &applet_oid).await?;

        if &applet_oid == outcome_data {
            Ok(ModelToViewMessage::new(applet_oid))
        } else {
            Err(design_scaffold::Error::ImpossibleState("Id mismatch from AppletsRepo".into()))
        }
    }
}
