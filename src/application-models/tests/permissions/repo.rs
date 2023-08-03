// use crate::{
//     permissions::{model::PermissionName, NewPermission, PermissionsRepo},
//     DatabaseConnection,
// };

// use design_scaffold::{
//     oid::{ObjectIdReactor, OidGenerator},
//     paging::{PagingLimit, PagingOffset},
//     repo::Repository,
// };

#[tokio::test]
async fn ok_on_validate_new_permission() {
    use super::NewPermission;
    use design_scaffold::validators::DataValidator;

    let new_permission_str = r#"
{
    "name": "Administrator"
}
"#;
    let new_permission: NewPermission = serde_json::from_str(new_permission_str).unwrap();

    let test_result = new_permission.validate().await;
    assert!(test_result.is_ok());
}

#[tokio::test]
async fn err_on_validate_new_permission_with_wrong_name() {
    use super::NewPermission;
    use design_scaffold::validators::DataValidator;

    let new_permission_str = r#"
{
    "name": "Adminis9traor"
}
"#;
    let new_permission: NewPermission = serde_json::from_str(new_permission_str).unwrap();

    let test_result = new_permission.validate().await;
    assert!(test_result.is_err());
}

// #[tokio::test]
// async fn err_on_try_to_delete_nonexistent_permission() {
//     use crate::permissions::model::PermissionOid;
//     let oid_reactor = ObjectIdReactor::new().await.unwrap().take();
//     let database = DatabaseConnection::new(&oid_reactor, ":memory:").await.unwrap();
//     let db_conn_pool = database.take();

//     let permission_oid = PermissionOid::from("99999999".to_string());
//     let result = PermissionsRepo::delete(&db_conn_pool, &permission_oid).await;

//     assert!(result.is_err());
// }

// #[tokio::test]
// async fn ok_on_try_to_delete_three_permissions() {
//     let oid_reactor = ObjectIdReactor::new().await.unwrap().take();

//     let database = DatabaseConnection::new(&oid_reactor, ":memory:").await.unwrap();
//     let db_conn_pool = database.take();

//     let mut detected_errors = 0;
//     let permissions =
//         PermissionsRepo::read_all(&db_conn_pool, &PagingLimit::default(), &PagingOffset::default())
//             .await
//             .unwrap();

//     for permission in permissions.iter() {
//         let permission_oid = permission.get_oid();

//         if let Err(error) = PermissionsRepo::delete(&db_conn_pool, permission_oid).await {
//             dbg!(&error); // `cargo test -- --nocapture`
//             detected_errors += 1;
//         }
//     }
//     assert!(detected_errors == 0);
// }

// #[tokio::test]
// async fn ok_on_try_to_create_permission() {
//     // let database = DatabaseConnection::new(":memory:").await.unwrap();
//     let oid_reactor = ObjectIdReactor::new().await.unwrap().take();

//     let database = DatabaseConnection::new(&oid_reactor, ":memory:").await.unwrap();
//     let db_conn_pool = database.take();

//     let option_oid_pool: Option<&OidReactorSynced> = Some(&oid_reactor);

//     let new_permission = NewPermission { name: PermissionName::from("Some name".to_owned()) };
//     let result = PermissionsRepo::create(oid_generator, &db_conn_pool, &new_permission).await;
//     if let Err(error) = &result {
//         dbg!(error); // `cargo test -- --nocapture`
//     }
//     assert!(result.is_ok());
// }
