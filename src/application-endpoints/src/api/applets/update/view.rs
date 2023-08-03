use super::outcome::ModelToViewMessage;
use super::UpdateApplet;
use application_models::applets::model::AppletOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct AppletIdUpdated {
    id: AppletOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<AppletIdUpdated>> for UpdateApplet {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<AppletIdUpdated>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let applet_updated = data.take();
                let (option_applet_oid, _, _) = applet_updated.take();
                match option_applet_oid {
                    Some(applet_oid) => {
                        tracing::info!("Applet id: [{}] updated", &*applet_oid);
                        Ok(Json(AppletIdUpdated { id: applet_oid }))
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
                let error_msg = format!("Applet can't be updated. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
