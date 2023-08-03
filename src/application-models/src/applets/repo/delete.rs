use crate::applets::model::AppletOid;
use sqlx::SqlitePool;

// CRUD - DeleteOne
pub(super) async fn delete_applet(
    db_conn_pool: &SqlitePool,
    applet_oid: &AppletOid,
) -> design_scaffold::Result<()> {
    let applet_oid_str = &*(*applet_oid);

    tracing::debug!("AppletOid: {}", &*applet_oid_str);
    let rows_affected = sqlx::query!(
        r#"
            UPDATE `applets`
            SET is_deleted = 1
            WHERE is_deleted = 0 AND oid = ?1;
        "#,
        applet_oid_str
    )
    .execute(db_conn_pool)
    .await?
    .rows_affected();

    if rows_affected > 0 {
        Ok(())
    } else {
        Err(design_scaffold::Error::EntityIdNotFound(format!(
            "Applet id [{applet_oid_str}] not found"
        )))
    }
}
