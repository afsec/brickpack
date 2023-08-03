use crate::common::db_bootstrapping;

use application_models::{
    departments::DepartmentsRepo,
    permissions::PermissionsRepo,
    statuses::StatusesRepo,
    users::{model::*, *},
};
use design_scaffold::paging::{PagingLimit, PagingOffset};
use design_scaffold::repo::Repository;

#[tokio::test]
async fn err_on_try_to_delete_nonexistent_user() {
    let (_, database) = db_bootstrapping().await;
    let db_conn_pool = database.take();
    let user_oid = UserOid::from("99999999".to_string());
    let result = UsersRepo::delete(&db_conn_pool, &user_oid).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn ok_on_try_to_delete_three_users() {
    let (_, database) = db_bootstrapping().await;
    let db_conn_pool = database.take();

    let mut detected_errors = 0;

    let users_oid: Vec<UserOid> =
        UsersRepo::read_all(&db_conn_pool, &PagingLimit::default(), &PagingOffset::default())
            .await
            .unwrap()
            .into_iter()
            .map(|user| {
                let (user_oid, _, _, _, _, _) = user.take();
                user_oid
            })
            .collect();
    for user_oid in users_oid {
        if let Err(error) = UsersRepo::delete(&db_conn_pool, &user_oid).await {
            dbg!(&error); // `cargo test -- --nocapture`
            detected_errors += 1;
        }
    }

    assert!(detected_errors == 0);
}

#[tokio::test]
async fn ok_on_try_to_create_user() {
    let (oid_pool, database) = db_bootstrapping().await;
    let db_conn_pool = database.take();

    // * Get valid Department
    let (valid_department_oid, _, _, _) =
        DepartmentsRepo::read_all(&db_conn_pool, &PagingLimit::default(), &PagingOffset::default())
            .await
            .unwrap()
            .pop()
            .unwrap()
            .take();

    // * Get valid Permission
    let (valid_permission_oid, _, _, _) =
        PermissionsRepo::read_all(&db_conn_pool, &PagingLimit::default(), &PagingOffset::default())
            .await
            .unwrap()
            .pop()
            .unwrap()
            .take();

    // * Get valid Status
    let (valid_status_oid, _, _, _) =
        StatusesRepo::read_all(&db_conn_pool, &PagingLimit::default(), &PagingOffset::default())
            .await
            .unwrap()
            .pop()
            .unwrap()
            .take();

    let new_user = NewUser::from((
        UserName::from("User full name".to_owned()),
        UserEmail::from("user@example.net".to_owned()),
        UserDepartment::from(valid_department_oid),
        UserPermission::from(valid_permission_oid),
        UserStatus::from(valid_status_oid),
    ));
    let result = UsersRepo::create(Some(&oid_pool), &db_conn_pool, &new_user).await;
    if let Err(error) = &result {
        dbg!(error); // `cargo test -- --nocapture`
    }
    assert!(result.is_ok());
}
