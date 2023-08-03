use super::outcome::ModelToViewMessage;
use super::CountStatuses;
use application_models::statuses::StatusesLength;
use async_trait::async_trait;
use axum::http::StatusCode;
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, StatusesLength> for CountStatuses {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<StatusesLength, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let statuses_length = data.take();
                tracing::info!("Statuses found: [{}]", &(*statuses_length));
                Ok(statuses_length)
            }
            Err(error) => {
                let error_msg = format!("Statuses can't be counted. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
