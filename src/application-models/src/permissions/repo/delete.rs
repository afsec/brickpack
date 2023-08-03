use crate::permissions::model::PermissionOid;
use sqlx::SqlitePool;

// CRUD - DeleteOne
pub(super) async fn delete_permission(
    db_conn_pool: &SqlitePool,
    permission_oid: &PermissionOid,
) -> design_scaffold::Result<()> {
    let oid = &*(*permission_oid);

    tracing::debug!("PermissionId: {}", oid);
    let rows_affected = sqlx::query!(
        r#"
            UPDATE `permissions`
            SET is_deleted = 1
            WHERE is_deleted = 0 AND id = ?1;
        "#,
        oid
    )
    .execute(db_conn_pool)
    .await?
    .rows_affected();

    if rows_affected > 0 {
        Ok(())
    } else {
        Err(design_scaffold::Error::EntityIdNotFound(format!("Permission id [{oid}] not found")))
    }
}
