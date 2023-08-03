use crate::statuses::model::StatusOid;
use sqlx::SqlitePool;

// CRUD - DeleteOne
pub(super) async fn delete_status(
    db_conn_pool: &SqlitePool,
    status_oid: &StatusOid,
) -> design_scaffold::Result<()> {
    let oid = &*(*status_oid);

    tracing::debug!("StatusOid: {}", oid);
    let rows_affected = sqlx::query!(
        r#"
            UPDATE `statuses`
            SET is_deleted = 1
            WHERE is_deleted = 0 AND oid = ?1;
        "#,
        oid
    )
    .execute(db_conn_pool)
    .await?
    .rows_affected();

    if rows_affected > 0 {
        Ok(())
    } else {
        Err(design_scaffold::Error::EntityIdNotFound(format!("Status id [{oid}] not found")))
    }
}
