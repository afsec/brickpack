pub(super) mod create;
pub(super) mod delete;
pub(super) mod read;
pub(super) mod update;

use super::{model::UserOid, NewUser, User, UserToUpdate, UsersLength};
use async_trait::async_trait;
use design_scaffold::{
    oid::OidPool,
    paging::{PagingLimit, PagingOffset},
    repo::{Entity, EntityId, EntityMetadata, EntityNew, EntityToUpdate, RepoLength, Repository},
    validators::DataValidator,
};

use sqlx::Sqlite;
use sqlx::SqlitePool;

// * UsersRepo
pub struct UsersRepo;
impl EntityNew for NewUser {}
impl Entity for User {}
impl EntityMetadata for User {}
impl EntityId for UserOid {}

impl EntityToUpdate for UserToUpdate {}
impl RepoLength for UsersLength {}

#[async_trait]
impl<'entity_update, 'entity_id>
    Repository<
        '_,
        '_,
        'entity_update,
        'entity_id,
        '_,
        Sqlite,
        User,
        User, // Metadata too
        UserOid,
        NewUser,
        UsersLength,
        UserToUpdate,
    > for UsersRepo
{
    // Read (Count)
    async fn count(db_conn_pool: &SqlitePool) -> design_scaffold::Result<UsersLength> {
        read::count_users(db_conn_pool).await
    }

    // Create (One)
    async fn create(
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &SqlitePool,
        new_user: &NewUser,
    ) -> design_scaffold::Result<UserOid> {
        new_user.validate().await?;
        create::create_user(option_oid_pool, db_conn_pool, &new_user).await
    }

    // Read (All)
    async fn read_all(
        db_conn_pool: &SqlitePool,
        paging_limit: &PagingLimit,
        paging_offset: &PagingOffset,
    ) -> design_scaffold::Result<Vec<User>> {
        read::read_users(db_conn_pool, paging_limit, paging_offset).await
    }

    // Read (One)
    async fn read(db_conn_pool: &SqlitePool, user_oid: &UserOid) -> design_scaffold::Result<User> {
        user_oid.validate().await?;
        read::read_user(db_conn_pool, user_oid).await
    }

    // Update (One)
    async fn update(
        db_conn_pool: &SqlitePool,
        user_to_update: &'entity_update UserToUpdate,
    ) -> design_scaffold::Result<&'entity_update UserToUpdate> {
        user_to_update.validate().await?;
        update::update_user(db_conn_pool, &user_to_update).await?;
        Ok(user_to_update)
    }

    // Delete (One)
    async fn delete(
        db_conn_pool: &SqlitePool,
        user_oid: &'entity_id UserOid,
    ) -> design_scaffold::Result<&'entity_id UserOid> {
        user_oid.validate().await?;
        delete::delete_user(db_conn_pool, user_oid).await?;
        Ok(user_oid)
    }
}
