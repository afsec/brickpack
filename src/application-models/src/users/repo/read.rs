use crate::users::{
    model::{
        UserCreatedAt, UserDepartment, UserEmail, UserName, UserOid, UserPermission, UserStatus,
        UserUpdatedAt,
    },
    User, UserFromSqlx, UsersLength,
};
use design_scaffold::paging::{PagingLimit, PagingOffset};
use sqlx::SqlitePool;

// * CRUD - ReadALL
pub(super) async fn read_users<'a>(
    db_conn_pool: &SqlitePool,
    paging_limit: &PagingLimit,
    paging_offset: &PagingOffset,
) -> design_scaffold::Result<Vec<User>> {
    tracing::debug!("Limit: {:?}", &paging_limit);
    tracing::debug!("Offset: {:?}", &paging_offset);
    let limit = paging_limit.get();
    let offset = paging_offset.get();

    let records: Vec<UserFromSqlx> = sqlx::query_as!(
        UserFromSqlx,
        r#"
            SELECT oid, email, name, department, permission, status, created_at, updated_at
            FROM `users`
            WHERE is_deleted = 0 LIMIT ?1 OFFSET ?2;
        "#,
        limit,
        offset
    )
    .fetch_all(db_conn_pool)
    .await?;

    // * To improve performance -> https://github.com/launchbadge/sqlx/issues/117

    let users: Vec<User> = records.into_iter().map(|record| record.into()).collect();

    Ok(users)
}

// * CRUD - ReadOne
pub(super) async fn read_user(
    db_conn_pool: &SqlitePool,
    user_oid: &UserOid,
) -> design_scaffold::Result<User> {
    let oid = &*(*user_oid);

    tracing::debug!("UserOid: {oid}");
    // TODO: Implement:
    // TODO:  `let records:Vec<UserFromSqlx> = sqlx::query_as!(UserFromSqlx,`

    let record = sqlx::query!(
        r#"
            SELECT oid, email, name, department, permission, status, created_at, updated_at
            FROM `users`
            WHERE is_deleted = 0 AND oid = ?1;
        "#,
        oid
    )
    .fetch_one(db_conn_pool)
    .await?;
    let email: String = record.email;
    let name: String = record.name;
    Ok(User {
        oid: UserOid::from(record.oid),
        email: UserEmail::from(email),
        name: UserName::from(name),
        department: UserDepartment::from(record.department),
        permission: UserPermission::from(record.permission),
        status: UserStatus::from(record.status),
        created_at: UserCreatedAt::from(record.created_at),
        updated_at: UserUpdatedAt::from(record.updated_at),
    })
}

// * CRUD - ReadCount
pub(super) async fn count_users(db_conn_pool: &SqlitePool) -> design_scaffold::Result<UsersLength> {
    let records = sqlx::query!(
        r#"
            SELECT COUNT(*) as count FROM `users` WHERE is_deleted = 0;
        "#
    )
    .fetch_one(db_conn_pool)
    .await?;
    Ok(UsersLength::try_from(records.count)?)
}
