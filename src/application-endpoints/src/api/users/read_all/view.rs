use super::outcome::ModelToViewMessage;
use super::ReadUsers;
use application_models::users::User;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<Vec<User>>> for ReadUsers {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<Vec<User>>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let users = data.take();
                tracing::info!("Users found: [{}]", users.len());
                Ok(Json(users))
            }
            Err(error) => {
                let error_msg = format!("Users can't be read. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
