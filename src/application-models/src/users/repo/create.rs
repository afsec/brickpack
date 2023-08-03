use design_scaffold::oid::OidPool;
use sqlx::{types::chrono::NaiveDateTime, SqlitePool};

use crate::users::{
    model::{UserCreatedAt, UserOid},
    NewUser,
};

// CRUD - CreateOne
pub(super) async fn create_user(
    option_oid_pool: Option<&OidPool>,
    db_conn_pool: &SqlitePool,
    new_user: &NewUser,
) -> design_scaffold::Result<UserOid> {
    let new_user_oid = UserOid::new(option_oid_pool).await?;
    tracing::debug!("NewUser: {:?}", new_user);
    let new_oid = &(*new_user_oid);
    let created_at: NaiveDateTime = UserCreatedAt::now().into();
    let updated_at = &created_at;

    let _ = sqlx::query!(
        r#"
            INSERT INTO `users`
            ("oid","name","email","department","permission","status","created_at","updated_at","is_deleted")
            VALUES (?1,?2,?3,?4,?5,?6,?7,?8,0);
        "#,
        *new_oid,
        *new_user.name,
        *new_user.email,
        *new_user.department,
        *new_user.permission,
        *new_user.status,
        created_at,
        *updated_at
    )
    .execute(db_conn_pool)
    .await?;

    Ok(new_user_oid)
}
