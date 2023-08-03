use super::outcome::ModelToViewMessage;
use super::ReadPermissions;
use crate::api::Paging;
use application_models::permissions::Permission;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, ReadPermissions, Sqlite, Paging, ModelToViewMessage, Json<Vec<Permission>>>
    for ReadPermissions
{
}

// impl<'endpoint> Presenter<'endpoint, ReadPermissions, Sqlite, Paging, ModelToViewMessage, Response>
//     for ReadPermissions
// {
//     async fn presenter(
//         endpoint: &'endpoint ReadPermissions,
//         db_conn_pool: &Pool<Sqlite>,
//         submitted_data: Paging,
//     ) -> Response {
//         let model_result = endpoint.model(db_conn_pool submitted_data).await;
//         let view_result = endpoint.view(model_result).await;
//         view_result
//     }
// }
