use sqlx::SqlitePool;

use crate::permissions::PermissionToUpdate;

// CRUD - UpdateOne
pub(super) async fn update_permission(
    db_conn_pool: &SqlitePool,
    permission_to_update: &PermissionToUpdate,
) -> design_scaffold::Result<()> {
    tracing::debug!("PermissionToUpdate: {permission_to_update:?}");

    let permission_oid = match permission_to_update.get_oid() {
        Some(data) => data,
        None => {
            return Err(design_scaffold::Error::ImpossibleState(
                "Can't get permission_oid from Option".into(),
            ))
        }
    };
    let permission_oid_str = &*(*permission_oid);
    let mut total_rows_affected = 0;

    // * PermissionToUpdate.name
    if let Some(permission_name) = &permission_to_update.name {
        let permission_name_str = &*(*permission_name);
        let query_result = sqlx::query!(
            r#"
                UPDATE `permissions`
                SET name = ?1
                WHERE oid = ?2
             ;
            "#,
            permission_name_str,
            permission_oid_str
        )
        .execute(db_conn_pool)
        .await?;
        total_rows_affected += query_result.rows_affected();
    }

    tracing::debug!("PermissionToUpdate: Total rows affected {}", total_rows_affected);

    Ok(())
}
