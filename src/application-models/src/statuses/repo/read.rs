use crate::statuses::{
    model::{StatusCreatedAt, StatusName, StatusOid, StatusUpdatedAt},
    Status, StatusFromSqlx, StatusesLength,
};
use design_scaffold::paging::{PagingLimit, PagingOffset};
use sqlx::SqlitePool;

// * CRUD - ReadALL
pub(super) async fn read_statuses<'a>(
    db_conn_pool: &SqlitePool,
    paging_limit: &PagingLimit,
    paging_offset: &PagingOffset,
) -> design_scaffold::Result<Vec<Status>> {
    tracing::debug!("Limit: {:?}", &paging_limit);
    tracing::debug!("Offset: {:?}", &paging_offset);
    let limit = paging_limit.get();
    let offset = paging_offset.get();
    let records: Vec<StatusFromSqlx> = sqlx::query_as!(
        StatusFromSqlx,
        r#"
            SELECT oid, name, created_at, updated_at
            FROM `statuses`
            WHERE is_deleted = 0 LIMIT ?1 OFFSET ?2;
        "#,
        limit,
        offset
    )
    .fetch_all(db_conn_pool)
    .await?;

    // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

    let statuses: Vec<Status> = records.into_iter().map(|record| record.into()).collect();

    Ok(statuses)
}

// * CRUD - ReadOne
pub(super) async fn read_status(
    db_conn_pool: &SqlitePool,
    status_oid: &StatusOid,
) -> design_scaffold::Result<Status> {
    let oid = &*(*status_oid);

    tracing::debug!("StatusOid: {}", oid);
    let record = sqlx::query!(
        r#"
            SELECT oid, name, created_at, updated_at
            FROM `statuses`
            WHERE is_deleted = 0 AND oid = ?1;
        "#,
        oid
    )
    .fetch_one(db_conn_pool)
    .await?;

    let name: String = record.name;
    Ok(Status {
        oid: StatusOid::from(record.oid),
        name: StatusName::from(name),
        created_at: StatusCreatedAt::from(record.created_at),
        updated_at: StatusUpdatedAt::from(record.updated_at),
    })
}

// * CRUD - ReadCount
pub(super) async fn count_statuses(
    db_conn_pool: &SqlitePool,
) -> design_scaffold::Result<StatusesLength> {
    let records = sqlx::query!(
        r#"
            SELECT COUNT(*) as count FROM `statuses` WHERE is_deleted = 0;
        "#
    )
    .fetch_one(db_conn_pool)
    .await?;
    Ok(StatusesLength::try_from(records.count)?)
}
