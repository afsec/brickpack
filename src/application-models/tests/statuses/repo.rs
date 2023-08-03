use super::NewStatus;
// use super::{model::StatusName, NewStatus, StatusesRepo};
// use crate::DatabaseConnection;
// use design_scaffold::{
// oid::{ObjectIdReactor, OidGenerator},
// paging::{PagingLimit, PagingOffset},
// repo::Repository,
// };

// #[tokio::test]
// async fn err_on_try_to_delete_nonexistent_status() {
//     use crate::statuses::model::StatusOid;
//     // let database = DatabaseConnection::new(":memory:").await.unwrap();
//     let oid_reactor = ObjectIdReactor::new().await.unwrap().take();

//     let database = DatabaseConnection::new(&oid_reactor, ":memory:").await.unwrap();
//     let db_conn_pool = database.take();

//     let status_oid = StatusOid::from("99999999".to_string());

//     let result = StatusesRepo::delete(&db_conn_pool, &status_oid).await;
//     assert!(result.is_err());
// }

// #[tokio::test]
// async fn ok_on_try_to_delete_three_statuses() {
//     let oid_reactor = ObjectIdReactor::new().await.unwrap().take();

//     let database = DatabaseConnection::new(&oid_reactor, ":memory:").await.unwrap();
//     let db_conn_pool = database.take();

//     let mut detected_errors = 0;
//     let statuses =
//         StatusesRepo::read_all(&db_conn_pool, &PagingLimit::default(), &PagingOffset::default())
//             .await
//             .unwrap();

//     for status in statuses.iter() {
//         let status_oid = status.get_oid();

//         if let Err(error) = StatusesRepo::delete(&db_conn_pool, status_oid).await {
//             dbg!(&error); // `cargo test -- --nocapture`
//             detected_errors += 1;
//         }
//     }

//     assert!(detected_errors == 0);

//     assert!(detected_errors == 0);
// }

// #[tokio::test]
// async fn ok_on_try_to_create_status() {
//     let oid_reactor = ObjectIdReactor::new().await.unwrap().take();

//     let database = DatabaseConnection::new(&oid_reactor, ":memory:").await.unwrap();
//     let db_conn_pool = database.take();
//     let option_oid_pool: Option<&OidReactorSynced> = Some(&oid_reactor);
//     let new_status = NewStatus {
//         // TODO: Check Status validation
//         name: StatusName::from("OK".to_owned()),
//     };
//     let result = StatusesRepo::create(oid_generator, &db_conn_pool, &new_status).await;
//     if let Err(error) = &result {
//         dbg!(error); // `cargo test -- --nocapture`
//     }
//     assert!(result.is_ok());
// }

#[tokio::test]
async fn ok_on_validate_new_status() {
    use design_scaffold::validators::DataValidator;

    let new_status_str = r#"
{
    "name": "Marketing"
}
"#;
    let new_status: NewStatus = serde_json::from_str(new_status_str).unwrap();

    let test_result = new_status.validate().await;
    assert!(test_result.is_ok());
}

#[tokio::test]
async fn err_on_validate_new_status_with_wrong_name() {
    use super::NewStatus;
    use design_scaffold::validators::DataValidator;

    let new_status_str = r#"
{
    "name": "Marke9ting"
}
"#;
    let new_status: NewStatus = serde_json::from_str(new_status_str).unwrap();

    let test_result = new_status.validate().await;
    assert!(test_result.is_err());
}
