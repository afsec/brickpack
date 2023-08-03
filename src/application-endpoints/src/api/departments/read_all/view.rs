use super::outcome::ModelToViewMessage;
use super::ReadDepartments;
use application_models::departments::Department;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<Vec<Department>>> for ReadDepartments {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<Vec<Department>>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let departments = data.take();
                tracing::info!("Departments found: [{}]", departments.len());
                Ok(Json(departments))
            }
            Err(error) => {
                let error_msg = format!("Departments can't be read. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
