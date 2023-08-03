use super::outcome::ModelToViewMessage;
use super::ReadApplet;

use application_models::applets::Applet;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<Applet>> for ReadApplet {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<Applet>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let retrieved_applet = data.take();
                let applet_oid = &*(*retrieved_applet.get_oid());
                tracing::info!("Applet id: [{}] found", applet_oid);
                Ok(Json(retrieved_applet))
            }
            Err(error) => {
                let error_msg = format!("Applet can't be read. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
