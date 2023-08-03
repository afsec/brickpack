use super::outcome::ModelToViewMessage;
use super::RunApplet;

use async_trait::async_trait;
use axum::http::StatusCode;
use axum::response::Html;
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Html<String>> for RunApplet {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Html<String>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let code_result = data.take();
                tracing::info!("Code with {} Bytes runned successfully", code_result.len());
                Ok(Html(code_result))
            }
            Err(error) => {
                let error_msg = format!("Impossible to run the Applet. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
