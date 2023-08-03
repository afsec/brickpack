use super::outcome::ModelToViewMessage;
use super::CountUsers;
use application_models::users::UsersLength;
use async_trait::async_trait;
use axum::http::StatusCode;
use design_scaffold::endpoint::View;

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, UsersLength> for CountUsers {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<UsersLength, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let users_length = data.take();
                tracing::info!("Users found: [{}]", &(*users_length));
                Ok(users_length)
            }
            Err(error) => {
                let error_msg = format!("Users can't be counted. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
