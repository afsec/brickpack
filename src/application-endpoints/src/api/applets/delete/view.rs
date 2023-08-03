use super::outcome::ModelToViewMessage;
use super::DeleteApplet;
use application_models::applets::model::AppletOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct AppletDeleted {
    id: AppletOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<AppletDeleted>> for DeleteApplet {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<AppletDeleted>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let id = data.take();
                tracing::info!("Applet id: [{}] deleted", *id);

                Ok(Json(AppletDeleted { id }))
            }
            Err(error) => {
                let error_msg = format!("Applet can't be deleted. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
