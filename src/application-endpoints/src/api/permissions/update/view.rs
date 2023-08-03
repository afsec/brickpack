use super::outcome::ModelToViewMessage;
use super::UpdatePermission;
use application_models::permissions::model::PermissionOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct PermissionOidUpdated {
    id: PermissionOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<PermissionOidUpdated>>
    for UpdatePermission
{
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<PermissionOidUpdated>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let permission_updated = data.take();
                let (option_permission_oid, _) = permission_updated.take();
                match option_permission_oid {
                    Some(permission_oid) => {
                        tracing::info!("Permission id: [{}] updated", *permission_oid);
                        Ok(Json(PermissionOidUpdated { id: permission_oid }))
                    }
                    None => {
                        let error_msg =
                            format!("Permission can't be updated. Reason: Impossible state.");
                        tracing::warn!("Model Error: {error_msg}");
                        Err((StatusCode::CONFLICT, error_msg))
                    }
                }
            }
            Err(error) => {
                let error_msg = format!("Permission can't be created. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
