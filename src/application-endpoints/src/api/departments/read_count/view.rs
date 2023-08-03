use super::outcome::ModelToViewMessage;
use super::CountDepartments;
use application_models::departments::DepartmentsLength;
use async_trait::async_trait;
use axum::http::StatusCode;
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, DepartmentsLength> for CountDepartments {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<DepartmentsLength, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let departments_length = data.take();
                tracing::info!("Departments found: [{}]", &(*departments_length));
                Ok(departments_length)
            }
            Err(error) => {
                let error_msg = format!("Departments can't be counted. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
