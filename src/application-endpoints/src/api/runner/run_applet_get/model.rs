use super::outcome::ModelToViewMessage;
use super::RunApplet;

use applet_runner::AppletRequestData;
use application_models::applets::model::AppletOid;
use application_models::applets::AppletsRepo;
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, AppletRequestData, ModelToViewMessage> for RunApplet {
    async fn model(
        &'endpoint self,
        _: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        request_data: AppletRequestData,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        use applet_runner::AppletRunner;
        use applet_runner::AppletRunnerArtifact;
        let (applet_oid, applet_method, applet_query_string, applet_cookies, applet_form) =
            request_data.take();
        let applet_oid_from_request = AppletOid::from(applet_oid);

        let applet_from_repo = AppletsRepo::read(db_conn_pool, &applet_oid_from_request).await?;

        let applet_runner = AppletRunner::new(
            applet_method,
            applet_query_string,
            applet_cookies,
            applet_form,
            AppletRunnerArtifact::from(applet_from_repo),
        );

        let code_output = applet_runner.run()?;

        Ok(ModelToViewMessage::new(code_output))
    }
}
