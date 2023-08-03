use super::outcome::ModelToViewMessage;
use super::CountApplets;
use application_models::applets::AppletsLength;
use async_trait::async_trait;
use axum::http::StatusCode;
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, AppletsLength> for CountApplets {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<AppletsLength, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let applets_length = data.take();
                tracing::info!("Applets found: [{}]", &(*applets_length));
                Ok(applets_length)
            }
            Err(error) => {
                let error_msg = format!("Applets can't be counted. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
