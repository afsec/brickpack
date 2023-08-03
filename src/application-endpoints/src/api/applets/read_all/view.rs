use super::outcome::ModelToViewMessage;
use super::ReadApplets;
use application_models::applets::AppletMetadata;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<Vec<AppletMetadata>>> for ReadApplets {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<Vec<AppletMetadata>>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let applets = data.take();
                tracing::info!("Applets found: [{}]", applets.len());
                Ok(Json(applets))
            }
            Err(error) => {
                let error_msg = format!("Applets can't be read. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
