use super::outcome::ModelToViewMessage;
use super::ReadStatus;

use application_models::statuses::Status;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<Status>> for ReadStatus {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<Status>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let retrieved_status = data.take();
                let status_oid = &*(*retrieved_status.get_oid());
                tracing::info!("Status id: [{}] found", status_oid);
                Ok(Json(retrieved_status))
            }
            Err(error) => {
                let error_msg = format!("Status can't be read. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
