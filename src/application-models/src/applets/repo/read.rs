use crate::applets::{
    model::{AppletCode, AppletCreatedAt, AppletFilename, AppletOid, AppletSize, AppletUpdatedAt},
    Applet, AppletMetadata, AppletMetadataFromSqlx, AppletsLength,
};
use design_scaffold::paging::{PagingLimit, PagingOffset};
use sqlx::SqlitePool;

// * CRUD - ReadALL
pub(super) async fn read_applets<'a>(
    db_conn_pool: &SqlitePool,
    paging_limit: &PagingLimit,
    paging_offset: &PagingOffset,
) -> design_scaffold::Result<Vec<AppletMetadata>> {
    tracing::debug!("Limit: {:?}", &paging_limit);
    tracing::debug!("Offset: {:?}", &paging_offset);
    let limit = paging_limit.get();
    let offset = paging_offset.get();
    let records: Vec<AppletMetadataFromSqlx> = sqlx::query_as!(
        AppletMetadataFromSqlx,
        r#"
            SELECT oid, filename, size, created_at, updated_at
            FROM `applets`
            WHERE is_deleted = 0 LIMIT ?1 OFFSET ?2;
        "#,
        limit,
        offset
    )
    .fetch_all(db_conn_pool)
    .await?;

    // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

    let applets: Vec<AppletMetadata> = records.into_iter().map(|record| record.into()).collect();

    Ok(applets)
}

// * CRUD - ReadOne
pub(super) async fn read_applet(
    db_conn_pool: &SqlitePool,
    applet_oid: &AppletOid,
) -> design_scaffold::Result<Applet> {
    let applet_oid_str = &*(*applet_oid);

    tracing::debug!("AppletOid: {}", &*applet_oid_str);
    let record = sqlx::query!(
        r#"
            SELECT oid, filename, code, size, created_at, updated_at FROM `applets` WHERE is_deleted = 0 AND oid = ?1;
        "#,
        applet_oid_str
    )
    .fetch_one(db_conn_pool)
    .await?;

    // TODO: To increase performance, implement -> https://github.com/launchbadge/sqlx/issues/117
    Ok(Applet {
        oid: AppletOid::from(record.oid),
        filename: AppletFilename::from(record.filename),
        code: AppletCode::from(record.code),
        size: AppletSize::from(record.size),
        created_at: AppletCreatedAt::from(record.created_at),
        updated_at: AppletUpdatedAt::from(record.updated_at),
    })
}

// * CRUD - ReadCount
pub(super) async fn count_applets(
    db_conn_pool: &SqlitePool,
) -> design_scaffold::Result<AppletsLength> {
    let records = sqlx::query!(
        r#"
            SELECT COUNT(*) as count FROM `applets` WHERE is_deleted = 0;
        "#
    )
    .fetch_one(db_conn_pool)
    .await?;
    Ok(AppletsLength::try_from(records.count)?)
}
