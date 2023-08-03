use super::outcome::ModelToViewMessage;
use super::ReadUser;

use application_models::users::User;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<User>> for ReadUser {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<User>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let retrieved_user = data.take();
                let user_oid = &*(*retrieved_user.get_oid());
                tracing::info!("User id: [{user_oid}] read");
                Ok(Json(retrieved_user))
            }
            Err(error) => {
                let error_msg = format!("User can't be read. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
