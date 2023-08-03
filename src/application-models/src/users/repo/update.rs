use sqlx::SqlitePool;

use crate::users::UserToUpdate;

// CRUD - UpdateOne
pub(super) async fn update_user(
    db_conn_pool: &SqlitePool,
    user_to_update: &UserToUpdate,
) -> design_scaffold::Result<()> {
    tracing::debug!("UserToUpdate: {:?}", user_to_update);

    let user_oid = match user_to_update.get_oid() {
        Some(data) => data,
        None => {
            return Err(design_scaffold::Error::ImpossibleState(
                "Can't get user_oid from Option".into(),
            ))
        }
    };
    let user_oid_str = &*(*user_oid);
    let mut total_rows_affected = 0;

    // * UserToUpdate.name
    if let Some(user_name) = &user_to_update.name {
        let user_name_str = &*(*user_name);
        let query_result = sqlx::query!(
            r#"
                UPDATE `users`
                SET name = ?1
                WHERE id = ?2
             ;
            "#,
            *user_name_str,
            *user_oid_str
        )
        .execute(db_conn_pool)
        .await?;
        total_rows_affected += query_result.rows_affected();
    }

    // * UserToUpdate.email
    if let Some(user_email) = &user_to_update.email {
        let user_email_str = &*(*user_email);
        let query_result = sqlx::query!(
            r#"
                UPDATE `users`
                SET email = ?1
                WHERE oid = ?2
             ;
            "#,
            *user_email_str,
            *user_oid_str
        )
        .execute(db_conn_pool)
        .await?;
        total_rows_affected += query_result.rows_affected();
    }

    // * UserToUpdate.department
    if let Some(user_department) = &user_to_update.department {
        let user_department_str = &*(*user_department);
        let query_result = sqlx::query!(
            r#"
                UPDATE `users`
                SET department = ?1
                WHERE oid = ?2
             ;
            "#,
            *user_department_str,
            *user_oid_str
        )
        .execute(db_conn_pool)
        .await?;
        total_rows_affected += query_result.rows_affected();
    }

    // * UserToUpdate.permission
    if let Some(user_permission) = &user_to_update.permission {
        let user_permission_str = &*(*user_permission);
        let query_result = sqlx::query!(
            r#"
                UPDATE `users`
                SET permission = ?1
                WHERE oid = ?2
             ;
            "#,
            *user_permission_str,
            *user_oid_str
        )
        .execute(db_conn_pool)
        .await?;
        total_rows_affected += query_result.rows_affected();
    }

    // * UserToUpdate.status
    if let Some(user_status) = &user_to_update.status {
        let user_status_str = &*(*user_status);
        let query_result = sqlx::query!(
            r#"
                UPDATE `users`
                SET status = ?1
                WHERE oid = ?2
             ;
            "#,
            *user_status_str,
            *user_oid_str
        )
        .execute(db_conn_pool)
        .await?;
        total_rows_affected += query_result.rows_affected();
    }

    tracing::debug!("UserToUpdate: Total rows affected {}", total_rows_affected);

    Ok(())
}
