pub(super) mod create;
pub(super) mod delete;
pub(super) mod read;
pub(super) mod update;

use super::{
    model::DepartmentOid, Department, DepartmentToUpdate, DepartmentsLength, NewDepartment,
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

// * DepartmentsRepo
pub struct DepartmentsRepo;
impl EntityNew for NewDepartment {}
impl Entity for Department {}
impl EntityMetadata for Department {}
impl EntityId for DepartmentOid {}

impl EntityToUpdate for DepartmentToUpdate {}
impl RepoLength for DepartmentsLength {}

#[async_trait]
impl<'entity_update, 'entity_id>
    Repository<
        '_,
        '_,
        'entity_update,
        'entity_id,
        '_,
        Sqlite,
        Department,
        Department, // Metadata too
        DepartmentOid,
        NewDepartment,
        DepartmentsLength,
        DepartmentToUpdate,
    > for DepartmentsRepo
{
    // Read (Count)
    async fn count(db_conn_pool: &SqlitePool) -> design_scaffold::Result<DepartmentsLength> {
        read::count_departments(db_conn_pool).await
    }

    // Create (One)
    async fn create(
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &SqlitePool,
        new_department: &NewDepartment,
    ) -> design_scaffold::Result<DepartmentOid> {
        new_department.validate().await?;
        create::create_department(option_oid_pool, db_conn_pool, &new_department).await
    }

    // Read (All)
    async fn read_all(
        db_conn_pool: &SqlitePool,
        paging_limit: &PagingLimit,
        paging_offset: &PagingOffset,
    ) -> design_scaffold::Result<Vec<Department>> {
        read::read_departments(db_conn_pool, paging_limit, paging_offset).await
    }

    // Read (One)
    async fn read(
        db_conn_pool: &SqlitePool,
        department_oid: &DepartmentOid,
    ) -> design_scaffold::Result<Department> {
        department_oid.validate().await?;
        read::read_department(db_conn_pool, department_oid).await
    }

    // Update (One)
    async fn update(
        db_conn_pool: &SqlitePool,
        department_to_update: &'entity_update DepartmentToUpdate,
    ) -> design_scaffold::Result<&'entity_update DepartmentToUpdate> {
        department_to_update.validate().await?;
        update::update_department(db_conn_pool, &department_to_update).await?;

        Ok(department_to_update)
    }

    // Delete (One)
    async fn delete(
        db_conn_pool: &SqlitePool,
        department_oid: &'entity_id DepartmentOid,
    ) -> design_scaffold::Result<&'entity_id DepartmentOid> {
        department_oid.validate().await?;
        delete::delete_department(db_conn_pool, department_oid).await?;
        Ok(department_oid)
    }
}
