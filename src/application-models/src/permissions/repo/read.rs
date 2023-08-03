use crate::permissions::{
    model::{PermissionCreatedAt, PermissionName, PermissionOid, PermissionUpdatedAt},
    Permission, PermissionFromSqlx, PermissionsLength,
};
use design_scaffold::paging::{PagingLimit, PagingOffset};
use sqlx::SqlitePool;

// * CRUD - ReadALL
pub(super) async fn read_permissions<'a>(
    db_conn_pool: &SqlitePool,
    paging_limit: &PagingLimit,
    paging_offset: &PagingOffset,
) -> design_scaffold::Result<Vec<Permission>> {
    tracing::debug!("Limit: {:?}", &paging_limit);
    tracing::debug!("Offset: {:?}", &paging_offset);
    let limit = paging_limit.get();
    let offset = paging_offset.get();
    let records: Vec<PermissionFromSqlx> = sqlx::query_as!(
        PermissionFromSqlx,
        r#"
            SELECT oid, name, created_at, updated_at
            FROM `permissions`
            WHERE is_deleted = 0 LIMIT ?1 OFFSET ?2;
        "#,
        limit,
        offset
    )
    .fetch_all(db_conn_pool)
    .await?;

    // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

    let permissions: Vec<Permission> = records.into_iter().map(|record| record.into()).collect();

    Ok(permissions)
}

// * CRUD - ReadOne
pub(super) async fn read_permission(
    db_conn_pool: &SqlitePool,
    permission_oid: &PermissionOid,
) -> design_scaffold::Result<Permission> {
    let oid = &*(*permission_oid);

    tracing::debug!("PermissionOid: {}", oid);
    let record = sqlx::query!(
        r#"
            SELECT oid, name, created_at, updated_at
            FROM `permissions` WHERE is_deleted = 0 AND oid = ?1;
        "#,
        oid
    )
    .fetch_one(db_conn_pool)
    .await?;

    let name: String = record.name;
    Ok(Permission {
        oid: PermissionOid::from(record.oid),
        name: PermissionName::from(name),
        created_at: PermissionCreatedAt::from(record.created_at),
        updated_at: PermissionUpdatedAt::from(record.updated_at),
    })
}

// * CRUD - ReadCount
pub(super) async fn count_permissions(
    db_conn_pool: &SqlitePool,
) -> design_scaffold::Result<PermissionsLength> {
    let records = sqlx::query!(
        r#"
            SELECT COUNT(*) as count FROM `permissions` WHERE is_deleted = 0;
        "#
    )
    .fetch_one(db_conn_pool)
    .await?;
    Ok(PermissionsLength::try_from(records.count)?)
}
