use super::outcome::ModelToViewMessage;
use super::CreateDepartment;
use application_models::departments::model::DepartmentOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct DepartmentOidCreated {
    id: DepartmentOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<DepartmentOidCreated>>
    for CreateDepartment
{
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<DepartmentOidCreated>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let id = data.take();
                tracing::info!("Department created with id: [{}]", *id);
                Ok(Json(DepartmentOidCreated { id }))
            }
            Err(error) => {
                let error_msg = format!("Department can't be created. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
