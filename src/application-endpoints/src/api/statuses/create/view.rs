use super::outcome::ModelToViewMessage;
use super::CreateStatus;
use application_models::statuses::model::StatusOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct StatusOidCreated {
    id: StatusOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<StatusOidCreated>> for CreateStatus {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<StatusOidCreated>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let id = data.take();
                tracing::info!("Status created with id: [{}]", *id);
                Ok(Json(StatusOidCreated { id }))
            }
            Err(error) => {
                let error_msg = format!("Status can't be created. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
