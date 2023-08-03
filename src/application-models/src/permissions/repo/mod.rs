pub(super) mod create;
pub(super) mod delete;
pub(super) mod read;
pub(super) mod update;

use super::{
    model::PermissionOid, NewPermission, Permission, PermissionToUpdate, PermissionsLength,
};
use async_trait::async_trait;
use design_scaffold::{
    oid::OidPool,
    paging::{PagingLimit, PagingOffset},
    repo::{Entity, EntityId, EntityMetadata, EntityNew, EntityToUpdate, RepoLength, Repository},
    validators::DataValidator,
};
use sqlx::Sqlite;
use sqlx::SqlitePool;

// * PermissionsRepo
pub struct PermissionsRepo;
impl EntityNew for NewPermission {}
impl Entity for Permission {}
impl EntityMetadata for Permission {}
impl EntityId for PermissionOid {}

impl EntityToUpdate for PermissionToUpdate {}
impl RepoLength for PermissionsLength {}

#[async_trait]
impl<'entity_update, 'entity_id>
    Repository<
        '_,
        '_,
        'entity_update,
        'entity_id,
        '_,
        Sqlite,
        Permission,
        Permission, // Metadata too
        PermissionOid,
        NewPermission,
        PermissionsLength,
        PermissionToUpdate,
    > for PermissionsRepo
{
    // Read (Count)
    async fn count(db_conn_pool: &SqlitePool) -> design_scaffold::Result<PermissionsLength> {
        read::count_permissions(db_conn_pool).await
    }

    // Create (One)
    async fn create(
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &SqlitePool,
        new_permission: &NewPermission,
    ) -> design_scaffold::Result<PermissionOid> {
        new_permission.validate().await?;
        create::create_permission(option_oid_pool, db_conn_pool, &new_permission).await
    }

    // Read (All)
    async fn read_all(
        db_conn_pool: &SqlitePool,
        paging_limit: &PagingLimit,
        paging_offset: &PagingOffset,
    ) -> design_scaffold::Result<Vec<Permission>> {
        read::read_permissions(db_conn_pool, paging_limit, paging_offset).await
    }

    // Read (One)
    async fn read(
        db_conn_pool: &SqlitePool,
        permission_oid: &PermissionOid,
    ) -> design_scaffold::Result<Permission> {
        permission_oid.validate().await?;
        read::read_permission(db_conn_pool, permission_oid).await
    }

    // Update (One)
    async fn update(
        db_conn_pool: &SqlitePool,
        permission_to_update: &'entity_update PermissionToUpdate,
    ) -> design_scaffold::Result<&'entity_update PermissionToUpdate> {
        permission_to_update.validate().await?;
        update::update_permission(db_conn_pool, &permission_to_update).await?;
        Ok(permission_to_update)
    }

    // Delete (One)
    async fn delete(
        db_conn_pool: &SqlitePool,
        permission_oid: &'entity_id PermissionOid,
    ) -> design_scaffold::Result<&'entity_id PermissionOid> {
        permission_oid.validate().await?;
        delete::delete_permission(db_conn_pool, permission_oid).await?;
        Ok(permission_oid)
    }
}
