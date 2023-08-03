use super::outcome::ModelToViewMessage;
use super::UpdateStatus;
use application_models::statuses::model::StatusOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct StatusOidUpdated {
    id: StatusOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<StatusOidUpdated>> for UpdateStatus {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<StatusOidUpdated>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let status_updated = data.take();
                let (option_status_oid, _) = status_updated.take();
                match option_status_oid {
                    Some(status_oid) => {
                        tracing::info!("Status id: [{}] updated", *status_oid);
                        Ok(Json(StatusOidUpdated { id: status_oid }))
                    }
                    None => {
                        let error_msg =
                            format!("Status can't be updated. Reason: Impossible state.");
                        tracing::warn!("Model Error: {error_msg}");
                        Err((StatusCode::CONFLICT, error_msg))
                    }
                }
            }
            Err(error) => {
                let error_msg = format!("Status can't be updated. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
