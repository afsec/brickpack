use super::outcome::ModelToViewMessage;
use super::ReadDepartments;
use crate::api::Paging;
use application_models::departments::Department;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, ReadDepartments, Sqlite, Paging, ModelToViewMessage, Json<Vec<Department>>>
    for ReadDepartments
{
}

// impl<'endpoint> Presenter<'endpoint, ReadDepartments, Sqlite, Paging, ModelToViewMessage, Response>
//     for ReadDepartments
// {
//     async fn presenter(
//         endpoint: &'endpoint ReadDepartments,
//         db_conn_pool: &Pool<Sqlite>,
//         submitted_data: Paging,
//     ) -> Response {
//         let model_result = endpoint.model(db_conn_pool submitted_data).await;
//         let view_result = endpoint.view(model_result).await;
//         view_result
//     }
// }
