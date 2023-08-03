use design_scaffold::oid::OidPool;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::SqlitePool;

use crate::permissions::{
    model::{PermissionCreatedAt, PermissionOid},
    NewPermission,
};

// CRUD - CreateOne
pub(super) async fn create_permission(
    option_oid_pool: Option<&OidPool>,
    db_conn_pool: &SqlitePool,
    new_permission: &NewPermission,
) -> design_scaffold::Result<PermissionOid> {
    tracing::debug!("NewPermission: {:?}", new_permission);
    let new_permission_oid = PermissionOid::new(option_oid_pool).await?;
    let name = &(*new_permission.name);
    let created_at: NaiveDateTime = PermissionCreatedAt::now().into();
    let updated_at = &created_at;
    let _ = sqlx::query!(
        r#"
            INSERT INTO `permissions` ("oid","name","created_at","updated_at","is_deleted")
            VALUES (?1,?2,?3,?4,0);
        "#,
        *new_permission_oid,
        name,
        created_at,
        updated_at
    )
    .execute(db_conn_pool)
    .await?;

    Ok(new_permission_oid)
}
