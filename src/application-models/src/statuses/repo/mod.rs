pub(super) mod create;
pub(super) mod delete;
pub(super) mod read;
pub(super) mod update;

use super::{model::StatusOid, NewStatus, Status, StatusToUpdate, StatusesLength};
use async_trait::async_trait;
use design_scaffold::{
    oid::OidPool,
    paging::{PagingLimit, PagingOffset},
    repo::{Entity, EntityId, EntityMetadata, EntityNew, EntityToUpdate, RepoLength, Repository},
    validators::DataValidator,
};

use sqlx::Sqlite;
use sqlx::SqlitePool;

// * StatusesRepo
pub struct StatusesRepo;
impl EntityNew for NewStatus {}
impl Entity for Status {}
impl EntityMetadata for Status {}
impl EntityId for StatusOid {}

impl EntityToUpdate for StatusToUpdate {}
impl RepoLength for StatusesLength {}

#[async_trait]
impl<'entity_update, 'entity_id>
    Repository<
        '_,
        '_,
        'entity_update,
        'entity_id,
        '_,
        Sqlite,
        Status,
        Status, // Metadata too
        StatusOid,
        NewStatus,
        StatusesLength,
        StatusToUpdate,
    > for StatusesRepo
{
    // Read (Count)
    async fn count(db_conn_pool: &SqlitePool) -> design_scaffold::Result<StatusesLength> {
        read::count_statuses(db_conn_pool).await
    }

    // Create (One)
    async fn create(
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &SqlitePool,
        new_status: &NewStatus,
    ) -> design_scaffold::Result<StatusOid> {
        new_status.validate().await?;
        create::create_status(option_oid_pool, db_conn_pool, &new_status).await
    }

    // Read (All)
    async fn read_all(
        db_conn_pool: &SqlitePool,
        paging_limit: &PagingLimit,
        paging_offset: &PagingOffset,
    ) -> design_scaffold::Result<Vec<Status>> {
        read::read_statuses(db_conn_pool, paging_limit, paging_offset).await
    }

    // Read (One)
    async fn read(
        db_conn_pool: &SqlitePool,
        status_oid: &StatusOid,
    ) -> design_scaffold::Result<Status> {
        status_oid.validate().await?;
        read::read_status(db_conn_pool, status_oid).await
    }

    // Update (One)
    async fn update(
        db_conn_pool: &SqlitePool,
        status_to_update: &'entity_update StatusToUpdate,
    ) -> design_scaffold::Result<&'entity_update StatusToUpdate> {
        status_to_update.validate().await?;
        update::update_status(db_conn_pool, &status_to_update).await?;
        Ok(status_to_update)
    }

    // Delete (One)
    async fn delete(
        db_conn_pool: &SqlitePool,
        status_oid: &'entity_id StatusOid,
    ) -> design_scaffold::Result<&'entity_id StatusOid> {
        status_oid.validate().await?;
        delete::delete_status(db_conn_pool, status_oid).await?;
        Ok(status_oid)
    }
}
