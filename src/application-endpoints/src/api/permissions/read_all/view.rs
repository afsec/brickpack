use super::outcome::ModelToViewMessage;
use super::ReadPermissions;
use application_models::permissions::Permission;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<Vec<Permission>>> for ReadPermissions {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<Vec<Permission>>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let permissions = data.take();
                tracing::info!("Permissions found: [{}]", permissions.len());
                Ok(Json(permissions))
            }
            Err(error) => {
                let error_msg = format!("Permission can't be created. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
