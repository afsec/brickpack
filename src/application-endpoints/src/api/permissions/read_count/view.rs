use super::outcome::ModelToViewMessage;
use super::CountPermissions;
use application_models::permissions::PermissionsLength;
use async_trait::async_trait;
use axum::http::StatusCode;
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, PermissionsLength> for CountPermissions {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<PermissionsLength, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let permissions_length = data.take();
                tracing::info!("Permissions found: [{}]", &(*permissions_length));
                Ok(permissions_length)
            }
            Err(error) => {
                let error_msg = format!("Permission can't be created. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
