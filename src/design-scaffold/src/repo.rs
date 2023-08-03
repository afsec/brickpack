use crate::{
    error::BrickpackError,
    oid::OidPool,
    paging::{PagingLimit, PagingOffset},
    validators::DataValidator,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::Pool;

#[async_trait]
pub trait Repository<
    'repo_length,
    'entity_new,
    'entity_update,
    'entity_id,
    'entity,
    DBDRIVER,
    ENTITY,
    M,
    I,
    N,
    L,
    U,
> where
    DBDRIVER: sqlx::Database,
    ENTITY: Serialize + Deserialize<'entity> + Entity,
    M: Serialize + Deserialize<'entity> + EntityMetadata,
    I: Serialize + Deserialize<'entity_id> + EntityId + DataValidator,
    N: Serialize + Deserialize<'entity_new> + EntityNew,
    L: Deserialize<'repo_length> + RepoLength,
    U: Serialize + Deserialize<'entity_update> + EntityToUpdate,
{
    async fn count(db_conn_pool: &Pool<DBDRIVER>) -> Result<L, BrickpackError>;
    async fn create(
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &Pool<DBDRIVER>,
        entity: &N,
    ) -> Result<I, BrickpackError>;
    async fn read_all(
        db_conn_pool: &Pool<DBDRIVER>,
        paging_limit: &PagingLimit,
        paging_offset: &PagingOffset,
    ) -> Result<Vec<M>, BrickpackError>;
    async fn read(db_conn_pool: &Pool<DBDRIVER>, entity_id: &I) -> Result<ENTITY, BrickpackError>;
    async fn update(
        db_conn_pool: &Pool<DBDRIVER>,
        entity: &'entity_update U,
    ) -> Result<&'entity_update U, BrickpackError>;
    async fn delete(
        db_conn_pool: &Pool<DBDRIVER>,
        entity_id: &'entity_id I,
    ) -> Result<&'entity_id I, BrickpackError>;
}

pub trait EntityId {}
pub trait EntityNew {}
pub trait Entity {}

pub trait EntityMetadata {}
pub trait EntityToUpdate {}

pub trait RepoLength {}
