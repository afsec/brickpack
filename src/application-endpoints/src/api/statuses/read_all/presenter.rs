use super::outcome::ModelToViewMessage;
use super::ReadStatuses;
use crate::api::Paging;
use application_models::statuses::Status;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, ReadStatuses, Sqlite, Paging, ModelToViewMessage, Json<Vec<Status>>>
    for ReadStatuses
{
}

// impl<'endpoint> Presenter<'endpoint, ReadStatuses, Sqlite, Paging, ModelToViewMessage, Response>
//     for ReadStatuses
// {
//     async fn presenter(
//         endpoint: &'endpoint ReadStatuses,
//         db_conn_pool: &Pool<Sqlite>,
//         submitted_data: Paging,
//     ) -> Response {
//         let model_result = endpoint.model(db_conn_pool submitted_data).await;
//         let view_result = endpoint.view(model_result).await;
//         view_result
//     }
// }
