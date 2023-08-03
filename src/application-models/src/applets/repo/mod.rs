pub(super) mod create;
pub(super) mod delete;
pub(super) mod read;
pub(super) mod update;

use super::{model::AppletOid, Applet, AppletMetadata, AppletToUpdate, AppletsLength, NewApplet};
use async_trait::async_trait;
use design_scaffold::{
    oid::OidPool,
    paging::{PagingLimit, PagingOffset},
    repo::{Entity, EntityId, EntityMetadata, EntityNew, EntityToUpdate, RepoLength, Repository},
    validators::DataValidator,
};

use sqlx::Sqlite;
use sqlx::SqlitePool;

// * AppletsRepo
pub struct AppletsRepo;

impl EntityNew for NewApplet {}
impl Entity for Applet {}
impl EntityMetadata for AppletMetadata {}
impl EntityId for AppletOid {}

impl EntityToUpdate for AppletToUpdate {}
impl RepoLength for AppletsLength {}

#[async_trait]
impl<'entity_update, 'entity_id>
    Repository<
        '_,
        '_,
        'entity_update,
        'entity_id,
        '_,
        Sqlite,
        Applet,
        AppletMetadata,
        AppletOid,
        NewApplet,
        AppletsLength,
        AppletToUpdate,
    > for AppletsRepo
{
    // Read (Count)
    async fn count(db_conn_pool: &SqlitePool) -> design_scaffold::Result<AppletsLength> {
        read::count_applets(db_conn_pool).await
    }

    // Create (One)
    async fn create(
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &SqlitePool,
        new_applet: &NewApplet,
    ) -> design_scaffold::Result<AppletOid> {
        new_applet.validate().await?;
        create::create_applet(option_oid_pool, db_conn_pool, &new_applet).await
    }

    // Read (All)
    async fn read_all(
        db_conn_pool: &SqlitePool,
        paging_limit: &PagingLimit,
        paging_offset: &PagingOffset,
    ) -> design_scaffold::Result<Vec<AppletMetadata>> {
        read::read_applets(db_conn_pool, paging_limit, paging_offset).await
    }

    // Read (One)
    async fn read(
        db_conn_pool: &SqlitePool,
        applet_oid: &AppletOid,
    ) -> design_scaffold::Result<Applet> {
        applet_oid.validate().await?;
        read::read_applet(db_conn_pool, applet_oid).await
    }

    // Update (One)
    async fn update(
        db_conn_pool: &SqlitePool,
        applet_to_update: &'entity_update AppletToUpdate,
    ) -> design_scaffold::Result<&'entity_update AppletToUpdate> {
        applet_to_update.validate().await?;

        update::update_applet(db_conn_pool, &applet_to_update).await?;
        Ok(applet_to_update)
    }

    // Delete (One)
    async fn delete(
        db_conn_pool: &SqlitePool,
        applet_oid: &'entity_id AppletOid,
    ) -> design_scaffold::Result<&'entity_id AppletOid> {
        applet_oid.validate().await?;
        delete::delete_applet(db_conn_pool, applet_oid).await?;
        Ok(applet_oid)
    }
}
