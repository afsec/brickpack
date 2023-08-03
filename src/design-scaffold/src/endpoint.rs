use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::Database;
use sqlx::Pool;
use std::marker::{Send, Sync};

use crate::error::BrickpackError;
use crate::oid::OidPool;

pub trait Outcome {}

pub trait Endpoint {}

// TODO: Maybe Implement as a feature ["endpoint_name"]
// pub trait Name {
//     fn name(&self) -> &'static str;
// }

#[async_trait]
pub trait Model<'endpoint, DBDRIVER, INPUTDATA, OUTCOME>
where
    DBDRIVER: Database,
    OUTCOME: Outcome + Send,
    INPUTDATA: Send + Sync,
{
    async fn model(
        &'endpoint self,
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &Pool<DBDRIVER>,
        submitted_data: INPUTDATA,
    ) -> Result<OUTCOME, BrickpackError>
    where
        INPUTDATA: 'async_trait,
        Self: Sized;
}

#[async_trait]
pub trait View<'endpoint, OUTCOME, INTORESPONSE>
where
    OUTCOME: Outcome + Send,
    INTORESPONSE: Sized,
{
    async fn view(
        &'endpoint self,
        result: Result<OUTCOME, BrickpackError>,
    ) -> Result<INTORESPONSE, (StatusCode, String)>
    where
        Self: Sized;
}

#[async_trait]
pub trait Presenter<'endpoint, ENDPOINT, DBDRIVER, INPUTDATA, OUTCOME, INTORESPONSE>
where
    ENDPOINT: Endpoint
        // + Name
        + Model<'endpoint, DBDRIVER, INPUTDATA, OUTCOME>
        + View<'endpoint, OUTCOME, INTORESPONSE>
        + Send
        + Sync
        + 'endpoint,
    DBDRIVER: Database,
    INPUTDATA: Send + Sync,
    OUTCOME: Outcome + Send,
    INTORESPONSE: Sized,
{
    async fn presenter(
        endpoint: &'endpoint ENDPOINT,
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &Pool<DBDRIVER>,
        submitted_data: INPUTDATA,
    ) -> Result<INTORESPONSE, (StatusCode, String)>
    where
        INPUTDATA: 'async_trait,
    {
        let model_result = endpoint.model(option_oid_pool, db_conn_pool, submitted_data).await;
        let view_result = endpoint.view(model_result).await;
        view_result
    }
}
