use super::outcome::ModelToViewMessage;
use super::DeleteStatus;
use application_models::statuses::model::StatusOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct StatusOidDeleted {
    id: StatusOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<StatusOidDeleted>> for DeleteStatus {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<StatusOidDeleted>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let id = data.take();
                tracing::info!("Status id: [{}] deleted", *id);

                Ok(Json(StatusOidDeleted { id }))
            }
            Err(error) => {
                let error_msg = format!("Status can't be deleted. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
