use super::outcome::ModelToViewMessage;
use super::ReadUsers;
use crate::api::Paging;
use application_models::users::User;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Presenter<'endpoint, ReadUsers, Sqlite, Paging, ModelToViewMessage, Json<Vec<User>>>
    for ReadUsers
{
}

// impl<'endpoint> Presenter<'endpoint, ReadUsers, Sqlite, Paging, ModelToViewMessage, Response>
//     for ReadUsers
// {
//     async fn presenter(
//         endpoint: &'endpoint ReadUsers,
//         db_conn_pool: &Pool<Sqlite>,
//         submitted_data: Paging,
//     ) -> Response {
//         let model_result = endpoint.model(db_conn_pool submitted_data).await;
//         let view_result = endpoint.view(model_result).await;
//         view_result
//     }
// }
