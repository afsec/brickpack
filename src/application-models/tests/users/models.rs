use crate::common::db_bootstrapping;
use application_models::{
    departments::{model::DepartmentName, DepartmentsRepo, NewDepartment},
    permissions::{model::PermissionName, NewPermission, PermissionsRepo},
    statuses::{
        model::{StatusName, StatusOid},
        NewStatus, StatusesRepo,
    },
    users::{
        model::{UserDepartment, UserEmail, UserId, UserName, UserPermission, UserStatus},
        NewUser,
    },
};
use design_scaffold::{repo::Repository, validators::DataValidator};

#[tokio::test]
async fn err_on_validate_new_user_with_wrong_department_oid() {
    // TODO: Check on BDD concepts, crates, etc. (?Jasmine?)

    let new_user_str = r#"
{
    "name": "John Doe",
    "email": "john@example.net",
    "department": "-1",
    "permission": "1",
    "status": "1"
}
"#;
    let new_user: NewUser = serde_json::from_str(new_user_str).unwrap();

    let test_result = new_user.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_new_user_with_wrong_invalid_name() {
    let new_user_str = r#"
{
    "name": "John%00Doe",
    "email": "john@example.net",
    "department": "1",
    "permission": "1",
    "status": "1"
}
"#;
    let new_user: NewUser = serde_json::from_str(new_user_str).unwrap();

    let test_result = new_user.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_new_user_with_wrong_permission_oid() {
    let new_user_str = r#"
{
    "name": "John Doe",
    "email": "john@example.net",
    "department": "1",
    "permission": "-1",
    "status": "1"
}
"#;
    let new_user: NewUser = serde_json::from_str(new_user_str).unwrap();

    let test_result = new_user.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_new_user_with_wrong_permission_oid_zero() {
    let new_user_str = r#"
{
    "name": "John Doe",
    "email": "john@example.net",
    "department": "1",
    "permission": "0",
    "status": "1"
}
"#;
    let new_user: NewUser = serde_json::from_str(new_user_str).unwrap();

    let test_result = new_user.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_new_user_with_wrong_status_oid() {
    let new_user_str = r#"
{
    "name": "John Doe",
    "email": "john@example.net",
    "department": "1",
    "permission": "1",
    "status": "-1"
}
"#;
    let new_user: NewUser = serde_json::from_str(new_user_str).unwrap();

    let test_result = new_user.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_new_user_with_wrong_status_oid_zero() {
    let new_user_str = r#"
{
    "name": "John Doe",
    "email": "john@example.net",
    "department": "1",
    "permission": "1",
    "status": "0"
}
"#;
    let new_user: NewUser = serde_json::from_str(new_user_str).unwrap();

    let test_result = new_user.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_new_user_with_wrong_name() {
    let new_user_str = r#"
{
    "name": "John D9oe",
    "email": "john@example.net",
    "department": "1",
    "permission": "1",
    "status": "1"

}
"#;
    let new_user: NewUser = serde_json::from_str(new_user_str).unwrap();

    let test_result = new_user.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_new_user_with_wrong_email() {
    let new_user_str = r#"
{
    "name": "John Doe",
    "email": "john@example.net@example.com",
    "department": "1",
    "permission": "1",
    "status": "1"

}
"#;
    let new_user: NewUser = serde_json::from_str(new_user_str).unwrap();

    let test_result = new_user.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_department_oid_user_part() {
    let department_oid_str = "http://localhost/#john.doe@example.net";

    let department_oid = UserPermission::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_department_oid_domain_part() {
    let department_oid_str = "john.doe@example.net:http://localhost/";

    let department_oid = UserPermission::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_department_oid_more_than_one_at_char() {
    let department_oid_str = "john.doe@example.net@http://localhost/";

    let department_oid = UserPermission::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_department_oid_empty() {
    let department_oid_str = "";

    let department_oid = UserPermission::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_department_oid_injection() {
    // Trying to inject: "john.<script>alert(1)</script>doe@example.net"
    let department_oid_str = "john%2E%3Cscript%3Ealert%281%29%3C%2Fscript%3Edoe%40example%2Enet";

    let department_oid = UserPermission::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_user_name_with_invalid_chars() {
    let status_oid_str = "John%00Doe";

    let status_oid = UserName::try_from(status_oid_str.to_owned()).unwrap();

    let test_result = status_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_user_id_with_invalid_data_negavite_number() {
    let input = -1;

    let test_result = UserId::try_from(input);

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_user_id_with_invalid_data_zero() {
    let input: i32 = 0;

    let test_result = UserId::try_from(input);

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_user_id_with_invalid_data_greater_than_u32_max() {
    let input = i64::from(u32::MAX) + 1;

    let user_id = UserId::try_from(input).unwrap();

    let test_result = user_id.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_email_address_user_part() {
    let input = "http://localhost/#john.doe@example.net";

    let email_address = UserEmail::try_from(input.to_owned()).unwrap();

    let test_result = email_address.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_email_address_domain_part() {
    let input = "john.doe@example.net:http://localhost/";

    let email_address = UserEmail::try_from(input.to_owned()).unwrap();

    let test_result = email_address.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_email_address_more_than_one_at_char() {
    let input = "john.doe@example.net@http://localhost/";

    let email_address = UserEmail::try_from(input.to_owned()).unwrap();

    let test_result = email_address.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_email_address_empty() {
    let input = "";

    let email_address = UserEmail::try_from(input.to_owned()).unwrap();

    let test_result = email_address.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_email_address_injection() {
    // Trying to inject: "john.<script>alert(1)</script>doe@example.net"
    let input = "john%2E%3Cscript%3Ealert%281%29%3C%2Fscript%3Edoe%40example%2Enet";

    let email_address = UserEmail::try_from(input.to_owned()).unwrap();

    let test_result = email_address.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_user_department_with_invalid_data_negavite_number() {
    let input = "-1";

    let user_department = UserDepartment::from(input.to_owned());

    let test_result = user_department.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_user_department_with_invalid_data_zero() {
    let input = "0";

    let user_department = UserDepartment::from(input.to_owned());

    let test_result = user_department.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_user_department_with_invalid_data() {
    let input = "john%2E%3Cscript%3Ealert%281%29%3C%2Fscript%3Edoe%40example%2Enet";

    let user_department = UserDepartment::from(input.to_owned());

    let test_result = user_department.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_status_oid_user_part() {
    let input = "http://localhost/#john.doe@example.net";

    let department_oid = StatusOid::try_from(input.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_status_oid_domain_part() {
    let input = "john.doe@example.net:http://localhost/";

    let department_oid = StatusOid::try_from(input.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_status_oid_more_than_one_at_char() {
    let input = "john.doe@example.net@http://localhost/";

    let status_oid = StatusOid::try_from(input.to_owned()).unwrap();

    let test_result = status_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_status_oid_empty() {
    let input = "";

    let status_oid = StatusOid::try_from(input.to_owned()).unwrap();

    let test_result = status_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_status_oid_injection() {
    // Trying to inject: "john.<script>alert(1)</script>doe@example.net"
    let input = "john%2E%3Cscript%3Ealert%281%29%3C%2Fscript%3Edoe%40example%2Enet";

    let status_oid = StatusOid::try_from(input.to_owned()).unwrap();

    let test_result = status_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_user_status_with_invalid_data_negavite_number() {
    let input = "-1";

    let user_status = UserStatus::try_from(input.to_owned()).unwrap();

    let test_result = user_status.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_status_oid_with_invalid_data_zero() {
    let input = "0";

    let user_status = UserStatus::try_from(input.to_owned()).unwrap();

    let test_result = user_status.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_status_oid_with_invalid_data_greater_than_u32_max() {
    let input = format!("{}", i64::from(u32::MAX) + 1);

    let user_status = UserStatus::try_from(input.to_owned()).unwrap();

    let test_result = user_status.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn ok_on_validate_new_user() {
    let (oid_pool, database) = db_bootstrapping().await;
    let db_conn_pool = database.take();
    let it_department_oid = DepartmentsRepo::create(
        Some(&oid_pool),
        &db_conn_pool,
        &NewDepartment::new(DepartmentName::from("Department X".to_string())),
    )
    .await
    .unwrap();
    let admin_permission_oid = PermissionsRepo::create(
        Some(&oid_pool),
        &db_conn_pool,
        &NewPermission::new(PermissionName::from("Permission X".to_string())),
    )
    .await
    .unwrap();
    let deactivated_status_oid = StatusesRepo::create(
        Some(&oid_pool),
        &db_conn_pool,
        &NewStatus::new(StatusName::from("Status X".to_string())),
    )
    .await
    .unwrap();

    let new_user = NewUser::from((
        UserName::from("Administrator".to_string()),
        UserEmail::from("admin@example.net".to_string()),
        UserDepartment::from(it_department_oid),
        UserPermission::from(admin_permission_oid),
        UserStatus::from(deactivated_status_oid),
    ));

    let new_user_str = serde_json::to_string(&new_user).unwrap();

    let new_user: NewUser = serde_json::from_str(&new_user_str).unwrap();

    let test_result = new_user.validate().await;
    assert!(test_result.is_ok());
}

#[tokio::test]
async fn ok_on_validate_valid_department_oid() {
    let input = "21da7698ba2d8c7fe7c64c4bc40b1a5c734c2445ad094514496d087f958aa8ba";

    let department_oid = UserPermission::try_from(input.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_ok());
}

#[tokio::test]
async fn ok_on_validate_valid_email_address() {
    let input = "john.doe@example.net";

    let email_address = UserEmail::try_from(input.to_owned()).unwrap();

    let test_result = email_address.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_ok());
}

#[tokio::test]
async fn ok_on_validate_valid_status_oid() {
    let input = "f4809c173c42772803884dbc3d970c5560a3e44d27b0bad653e82044f266a623";

    let department_oid = StatusOid::try_from(input.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_ok());
}
