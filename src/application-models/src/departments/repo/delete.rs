use crate::departments::model::DepartmentOid;
use sqlx::SqlitePool;

// CRUD - DeleteOne
pub(super) async fn delete_department(
    db_conn_pool: &SqlitePool,
    department_oid: &DepartmentOid,
) -> design_scaffold::Result<()> {
    let oid = &*(*department_oid);

    tracing::debug!("DepartmentOid: {}", &oid);
    let rows_affected = sqlx::query!(
        r#"
            UPDATE `departments`
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
        Err(design_scaffold::Error::EntityIdNotFound(format!("Department id [{oid}] not found")))
    }
}
