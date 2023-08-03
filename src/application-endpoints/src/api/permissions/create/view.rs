use super::outcome::ModelToViewMessage;
use super::CreatePermission;
use application_models::permissions::model::PermissionOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct PermissionOidCreated {
    id: PermissionOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<PermissionOidCreated>>
    for CreatePermission
{
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<PermissionOidCreated>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let id = data.take();
                tracing::info!("Permission created with id: [{}]", *id);
                Ok(Json(PermissionOidCreated { id }))
            }
            Err(error) => {
                let error_msg = format!("Permission can't be created. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
