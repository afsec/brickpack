use super::outcome::ModelToViewMessage;
use super::DeleteDepartment;
use application_models::departments::model::DepartmentOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct DepartmentOidDeleted {
    id: DepartmentOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<DepartmentOidDeleted>>
    for DeleteDepartment
{
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<DepartmentOidDeleted>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let id = data.take();
                tracing::info!("Department id: [{}] deleted", *id);

                Ok(Json(DepartmentOidDeleted { id }))
            }
            Err(error) => {
                let error_msg = format!("Department can't be deleted. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
