use sqlx::SqlitePool;

use crate::departments::DepartmentToUpdate;

// CRUD - UpdateOne
pub(super) async fn update_department(
    db_conn_pool: &SqlitePool,
    department_to_update: &DepartmentToUpdate,
) -> design_scaffold::Result<()> {
    tracing::debug!("DepartmentToUpdate: {:?}", department_to_update);
    let department_oid = match department_to_update.get_oid() {
        Some(data) => data,
        None => {
            return Err(design_scaffold::Error::ImpossibleState(
                "Can't get department_oid from Option".into(),
            ))
        }
    };
    let department_oid_str = &*(*department_oid);

    let mut total_rows_affected = 0;

    // * DepartmentToUpdate.name
    if let Some(department_name) = &department_to_update.name {
        let department_name_str = &*(*department_name);
        let query_result = sqlx::query!(
            r#"
                UPDATE `departments`
                SET name = ?1
                WHERE oid = ?2
             ;
            "#,
            *department_name_str,
            *department_oid_str
        )
        .execute(db_conn_pool)
        .await?;
        total_rows_affected += query_result.rows_affected();
    }

    tracing::debug!("DepartmentToUpdate: Total rows affected {}", total_rows_affected);

    Ok(())
}
