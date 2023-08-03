use crate::departments::{
    model::{DepartmentCreatedAt, DepartmentName, DepartmentOid, DepartmentUpdatedAt},
    Department, DepartmentFromSqlx, DepartmentsLength,
};
use design_scaffold::paging::{PagingLimit, PagingOffset};
use sqlx::SqlitePool;

// * CRUD - ReadALL
pub(super) async fn read_departments<'a>(
    db_conn_pool: &SqlitePool,
    paging_limit: &PagingLimit,
    paging_offset: &PagingOffset,
) -> design_scaffold::Result<Vec<Department>> {
    tracing::debug!("Limit: {:?}", &paging_limit);
    tracing::debug!("Offset: {:?}", &paging_offset);
    let limit = paging_limit.get();
    let offset = paging_offset.get();
    let records: Vec<DepartmentFromSqlx> = sqlx::query_as!(
        DepartmentFromSqlx,
        r#"
            SELECT oid, name, created_at, updated_at
            FROM `departments`
            WHERE is_deleted = 0 LIMIT ?1 OFFSET ?2;
        "#,
        limit,
        offset
    )
    .fetch_all(db_conn_pool)
    .await?;

    // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

    let departments: Vec<Department> = records.into_iter().map(|record| record.into()).collect();

    Ok(departments)
}

// * CRUD - ReadOne
pub(super) async fn read_department(
    db_conn_pool: &SqlitePool,
    department_oid: &DepartmentOid,
) -> design_scaffold::Result<Department> {
    let oid = &*(*department_oid);

    tracing::debug!("DepartmentOid: {}", &oid);
    let record = sqlx::query!(
        r#"
            SELECT oid, name, created_at, updated_at 
            FROM `departments`
            WHERE is_deleted = 0 AND oid = ?1;
        "#,
        oid
    )
    .fetch_one(db_conn_pool)
    .await?;

    let name: String = record.name;
    Ok(Department {
        oid: DepartmentOid::from(record.oid),
        name: DepartmentName::from(name),
        created_at: DepartmentCreatedAt::from(record.created_at),
        updated_at: DepartmentUpdatedAt::from(record.updated_at),
    })
}

// * CRUD - ReadCount
pub(super) async fn count_departments(
    db_conn_pool: &SqlitePool,
) -> design_scaffold::Result<DepartmentsLength> {
    let records = sqlx::query!(
        r#"
            SELECT COUNT(*) as count FROM `departments` WHERE is_deleted = 0;
        "#
    )
    .fetch_one(db_conn_pool)
    .await?;
    Ok(DepartmentsLength::try_from(records.count)?)
}
