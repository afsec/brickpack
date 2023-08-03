use super::outcome::ModelToViewMessage;
use super::ReadStatuses;
use application_models::statuses::Status;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<Vec<Status>>> for ReadStatuses {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<Vec<Status>>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let statuses = data.take();
                tracing::info!("Statuses found: [{}]", statuses.len());
                Ok(Json(statuses))
            }
            Err(error) => {
                let error_msg = format!("Statuses can't be read. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
