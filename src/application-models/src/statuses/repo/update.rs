use sqlx::SqlitePool;

use crate::statuses::StatusToUpdate;

// CRUD - UpdateOne
pub(super) async fn update_status(
    db_conn_pool: &SqlitePool,
    status_to_update: &StatusToUpdate,
) -> design_scaffold::Result<()> {
    tracing::debug!("StatusToUpdate: {:?}", status_to_update);
    let status_oid = match status_to_update.get_oid() {
        Some(data) => data,
        None => {
            return Err(design_scaffold::Error::ImpossibleState(
                "Can't get status_oid from Option".into(),
            ))
        }
    };
    let status_oid_str = &*(*status_oid);
    let mut total_rows_affected = 0;

    // * StatusToUpdate.name
    if let Some(status_name) = &status_to_update.name {
        let status_name_str = &*(*status_name);
        let query_result = sqlx::query!(
            r#"
                UPDATE `statuses`
                SET name = ?1
                WHERE oid = ?2
             ;
            "#,
            *status_name_str,
            status_oid_str
        )
        .execute(db_conn_pool)
        .await?;
        total_rows_affected += query_result.rows_affected();
    }

    tracing::debug!("StatusToUpdate: Total rows affected {}", total_rows_affected);

    Ok(())
}
