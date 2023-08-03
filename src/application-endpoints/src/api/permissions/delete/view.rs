use super::outcome::ModelToViewMessage;
use super::DeletePermission;
use application_models::permissions::model::PermissionOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct PermissionOidDeleted {
    id: PermissionOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<PermissionOidDeleted>>
    for DeletePermission
{
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<PermissionOidDeleted>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let id = data.take();
                tracing::info!("Permission id: [{}] deleted", *id);

                Ok(Json(PermissionOidDeleted { id }))
            }
            Err(error) => {
                let error_msg = format!("Permission can't be created. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
