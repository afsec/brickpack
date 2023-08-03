use super::outcome::ModelToViewMessage;
use super::ReadApplets;
use crate::api::Paging;
use application_models::applets::AppletMetadata;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, ReadApplets, Sqlite, Paging, ModelToViewMessage, Json<Vec<AppletMetadata>>>
    for ReadApplets
{
}

// impl<'endpoint> Presenter<'endpoint, ReadApplets, Sqlite, Paging, ModelToViewMessage, Response>
//     for ReadApplets
// {
//     async fn presenter(
//         endpoint: &'endpoint ReadApplets,
//         db_conn_pool: &Pool<Sqlite>,
//         submitted_data: Paging,
//     ) -> Response {
//         let model_result = endpoint.model(db_conn_pool submitted_data).await;
//         let view_result = endpoint.view(model_result).await;
//         view_result
//     }
// }
