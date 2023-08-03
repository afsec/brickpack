use super::outcome::ModelToViewMessage;
use super::CreateApplet;
use application_models::applets::model::AppletOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct AppletCreated {
    id: AppletOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<AppletCreated>> for CreateApplet {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<AppletCreated>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let id = data.take();
                tracing::info!("Applet created with id: [{}]", *id);
                Ok(Json(AppletCreated { id }))
            }
            Err(error) => {
                let error_msg = format!("Applet can't be created. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
