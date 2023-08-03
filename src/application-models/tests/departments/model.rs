#[tokio::test]
async fn err_on_validate_department_oid_with_invalid_data_negavite_number() {
    use super::DepartmentId;
    use design_scaffold::validators::DataValidator;
    let department_oid_str = "-1";

    let department_oid = DepartmentId::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_department_oid_with_invalid_data_zero() {
    use super::DepartmentId;
    use design_scaffold::validators::DataValidator;
    let department_oid_str = "0";

    let department_oid = DepartmentId::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_department_oid_with_invalid_data_greater_than_u32_max() {
    use super::DepartmentId;
    use design_scaffold::validators::DataValidator;

    let department_oid_str = format!("{}", i64::from(u32::MAX) + 1);

    let department_oid = DepartmentId::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn ok_on_validate_valid_department_oid() {
    use super::DepartmentOid;
    use design_scaffold::validators::DataValidator;

    let department_oid_str = "john.doe@example.net";

    let department_oid = DepartmentOid::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_ok());
}

#[tokio::test]
async fn err_on_validate_invalid_department_oid_user_part() {
    use super::DepartmentOid;
    use design_scaffold::validators::DataValidator;

    let department_oid_str = "http://localhost/#john.doe@example.net";

    let department_oid = DepartmentOid::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_department_oid_domain_part() {
    use super::DepartmentOid;
    use design_scaffold::validators::DataValidator;

    let department_oid_str = "john.doe@example.net:http://localhost/";

    let department_oid = DepartmentOid::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_department_oid_more_than_one_at_char() {
    use super::DepartmentOid;
    use design_scaffold::validators::DataValidator;

    let department_oid_str = "john.doe@example.net@http://localhost/";

    let department_oid = DepartmentOid::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_department_oid_empty() {
    use super::DepartmentOid;
    use design_scaffold::validators::DataValidator;

    let department_oid_str = "";

    let department_oid = DepartmentOid::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_department_oid_injection() {
    use super::DepartmentOid;
    use design_scaffold::validators::DataValidator;

    // Trying to inject: "john.<script>alert(1)</script>doe@example.net"
    let department_oid_str = "john%2E%3Cscript%3Ealert%281%29%3C%2Fscript%3Edoe%40example%2Enet";

    let department_oid = DepartmentOid::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_department_name_with_invalid_chars() {
    use super::DepartmentName;
    use design_scaffold::validators::DataValidator;

    let status_oid_str = "John%00Doe";

    let status_oid = DepartmentName::try_from(status_oid_str.to_owned()).unwrap();

    let test_result = status_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_department_oid_with_invalid_data_negavite_number() {
    use super::DepartmentUpdatedAt;
    use design_scaffold::validators::DataValidator;
    let department_oid_str = "-1";

    let department_oid = DepartmentUpdatedAt::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_department_oid_with_invalid_data_zero() {
    use super::DepartmentUpdatedAt;
    use design_scaffold::validators::DataValidator;
    let department_oid_str = "0";

    let department_oid = DepartmentUpdatedAt::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_department_oid_with_invalid_data_greater_than_u32_max() {
    use super::DepartmentUpdatedAt;
    use design_scaffold::validators::DataValidator;

    let department_oid_str = format!("{}", i64::from(u32::MAX) + 1);

    let department_oid = DepartmentUpdatedAt::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_department_oid_with_invalid_data_negavite_number() {
    use super::DepartmentCreatedAt;
    use design_scaffold::validators::DataValidator;
    let department_oid_str = "-1";

    let department_oid = DepartmentCreatedAt::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_department_oid_with_invalid_data_zero() {
    use super::DepartmentCreatedAt;
    use design_scaffold::validators::DataValidator;
    let department_oid_str = "0";

    let department_oid = DepartmentCreatedAt::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_department_oid_with_invalid_data_greater_than_u32_max() {
    use super::DepartmentCreatedAt;
    use design_scaffold::validators::DataValidator;

    let department_oid_str = format!("{}", i64::from(u32::MAX) + 1);

    let department_oid = DepartmentCreatedAt::try_from(department_oid_str.to_owned()).unwrap();

    let test_result = department_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}
#[tokio::test]
async fn ok_on_validate_new_department() {
    use super::NewDepartment;
    use design_scaffold::validators::DataValidator;

    let new_department_str = r#"
{
    "name": "Marketing"
}
"#;
    let new_department: NewDepartment = serde_json::from_str(new_department_str).unwrap();

    let test_result = new_department.validate().await;
    assert!(test_result.is_ok());
}

#[tokio::test]
async fn err_on_validate_new_department_with_wrong_name() {
    use super::NewDepartment;
    use design_scaffold::validators::DataValidator;

    let new_department_str = r#"
{
    "name": "Marke9ting"
}
"#;
    let new_department: NewDepartment = serde_json::from_str(new_department_str).unwrap();

    let test_result = new_department.validate().await;
    assert!(test_result.is_err());
}
