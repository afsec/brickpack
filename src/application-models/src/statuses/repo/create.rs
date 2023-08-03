use crate::statuses::{
    model::{StatusCreatedAt, StatusOid},
    NewStatus,
};
use design_scaffold::oid::OidPool;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::SqlitePool;

// CRUD - CreateOne
pub(super) async fn create_status(
    option_oid_pool: Option<&OidPool>,
    db_conn_pool: &SqlitePool,
    new_status: &NewStatus,
) -> design_scaffold::Result<StatusOid> {
    tracing::debug!("NewStatus: {:?}", new_status);
    let new_statud_oid = StatusOid::new(option_oid_pool).await?;
    let name = &(*new_status.name);
    let created_at: NaiveDateTime = StatusCreatedAt::now().into();
    let updated_at = &created_at;
    let _ = sqlx::query!(
        r#"
            INSERT INTO `statuses` ("oid","name","created_at","updated_at","is_deleted")
            VALUES (?1,?2,?3,?4,0);
        "#,
        *new_statud_oid,
        name,
        created_at,
        updated_at
    )
    .execute(db_conn_pool)
    .await?;

    Ok(new_statud_oid)
}
