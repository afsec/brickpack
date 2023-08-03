use super::outcome::ModelToViewMessage;
use super::ReadPermission;

use application_models::permissions::Permission;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<Permission>> for ReadPermission {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<Permission>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let retrieved_permission = data.take();
                let permission_oid = &*(*retrieved_permission.get_oid());
                tracing::info!("Permission id: [{}] found", permission_oid);
                Ok(Json(retrieved_permission))
            }
            Err(error) => {
                let error_msg = format!("Permission can't be created. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
