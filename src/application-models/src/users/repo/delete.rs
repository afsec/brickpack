use crate::users::model::UserOid;
use sqlx::SqlitePool;

// CRUD - DeleteOne
pub(super) async fn delete_user(
    db_conn_pool: &SqlitePool,
    user_oid: &UserOid,
) -> design_scaffold::Result<()> {
    let oid = &*(*user_oid);

    tracing::debug!("UserOid: {oid}");
    let rows_affected = sqlx::query!(
        r#"
            UPDATE `users`
            SET is_deleted = 1
            WHERE is_deleted = 0 AND oid = ?1;
        "#,
        oid
    )
    .execute(db_conn_pool)
    .await?
    .rows_affected();
    // TODO: Replicate to other repos
    if rows_affected > 0 {
        Ok(())
    } else {
        Err(design_scaffold::Error::EntityIdNotFound(format!("User id [{oid}] not found")))
    }
}
