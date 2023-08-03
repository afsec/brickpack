use super::outcome::ModelToViewMessage;
use super::UpdateUser;
use application_models::users::model::UserOid;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use design_scaffold::endpoint::View;
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct UserOidUpdated {
    id: UserOid,
}

#[async_trait]
impl<'endpoint> View<'endpoint, ModelToViewMessage, Json<UserOidUpdated>> for UpdateUser {
    async fn view(
        &'endpoint self,
        model_result: design_scaffold::Result<ModelToViewMessage>,
    ) -> Result<Json<UserOidUpdated>, (StatusCode, String)> {
        match model_result {
            Ok(data) => {
                let user_updated = data.take();
                let (option_user_oid, _, _, _, _, _) = user_updated.take();
                match option_user_oid {
                    Some(user_oid) => {
                        tracing::info!("User id: [{}] updated", *user_oid);
                        Ok(Json(UserOidUpdated { id: user_oid }))
                    }
                    None => {
                        let error_msg = format!("User can't be updated. Reason: Impossible state.");
                        tracing::warn!("Model Error: {error_msg}");
                        Err((StatusCode::CONFLICT, error_msg))
                    }
                }
            }
            Err(error) => {
                let error_msg = format!("User can't be updated. Reason: {error}");
                tracing::warn!("Model Error: {error:?}");
                Err((error.into(), error_msg))
            }
        }
    }
}
