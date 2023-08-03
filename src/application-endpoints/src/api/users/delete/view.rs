use super::outcome::ModelToViewMessage;
use super::DeleteUser;
use application_models::users::model::UserOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct UserOidDeleted {
    id: UserOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<UserOidDeleted>> for DeleteUser {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<UserOidDeleted>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let id = data.take();
                tracing::info!("User id: [{}] deleted", *id);

                Ok(Json(UserOidDeleted { id }))
            }
            Err(error) => {
                let error_msg = format!("User can't be deleted. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
