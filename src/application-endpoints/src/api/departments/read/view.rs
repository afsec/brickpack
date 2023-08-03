use super::outcome::ModelToViewMessage;
use super::ReadDepartment;

use application_models::departments::Department;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<Department>> for ReadDepartment {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<Department>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let retrieved_department = data.take();
                let department_oid = &*(*retrieved_department.get_oid());
                tracing::info!("Department id: [{department_oid}] found");
                Ok(Json(retrieved_department))
            }
            Err(error) => {
                let error_msg = format!("Department can't be read. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
