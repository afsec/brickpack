use design_scaffold::oid::OidPool;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::SqlitePool;

use crate::departments::{
    model::{DepartmentCreatedAt, DepartmentOid},
    NewDepartment,
};

// CRUD - CreateOne
pub(super) async fn create_department(
    option_oid_pool: Option<&OidPool>,
    db_conn_pool: &SqlitePool,
    new_department: &NewDepartment,
) -> design_scaffold::Result<DepartmentOid> {
    tracing::debug!("NewDepartment: {:?}", new_department);
    let new_department_oid = DepartmentOid::new(option_oid_pool).await?;
    let name = &(*new_department.name);
    let created_at: NaiveDateTime = DepartmentCreatedAt::now().into();
    let updated_at = &created_at;
    let _ = sqlx::query!(
        r#"
            INSERT INTO `departments` ("oid","name","created_at","updated_at","is_deleted")
            VALUES (?1,?2,?3,?4,0);
        "#,
        *new_department_oid,
        name,
        created_at,
        updated_at
    )
    .execute(db_conn_pool)
    .await?;

    Ok(new_department_oid)
}
