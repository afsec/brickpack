use super::outcome::ModelToViewMessage;
use super::UpdateDepartment;
use application_models::departments::model::DepartmentOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct DepartmentOidUpdated {
    id: DepartmentOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<DepartmentOidUpdated>>
    for UpdateDepartment
{
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<DepartmentOidUpdated>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let department_updated = data.take();
                let (option_department_oid, _) = department_updated.take();
                match option_department_oid {
                    Some(department_oid) => {
                        tracing::info!("Department id: [{}] updated", *department_oid);
                        Ok(Json(DepartmentOidUpdated { id: department_oid }))
                    }
                    None => {
                        let error_msg =
                            format!("Applet can't be updated. Reason: Impossible state.");
                        tracing::warn!("Model Error: {error_msg}");
                        Err((StatusCode::CONFLICT, error_msg))
                    }
                }
            }
            Err(error) => {
                let error_msg = format!("Department can't be updated. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
