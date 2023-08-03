use super::outcome::ModelToViewMessage;
use super::UpdateApplet;

use application_models::applets::{AppletToUpdate, AppletsRepo};
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, AppletToUpdate, ModelToViewMessage> for UpdateApplet {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        applet_to_update: AppletToUpdate,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let _ = AppletsRepo::update(db_conn_pool, &applet_to_update).await?;
        Ok(ModelToViewMessage::new(applet_to_update))
    }
}
