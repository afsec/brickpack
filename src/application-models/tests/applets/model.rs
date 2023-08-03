#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_negavite_number() {
    use super::AppletSize;
    use design_scaffold::validators::DataValidator;
    let applet_oid_str = "-1";

    let applet_oid = AppletSize::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_zero() {
    use super::AppletSize;
    use design_scaffold::validators::DataValidator;
    let applet_oid_str = "0";

    let applet_oid = AppletSize::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_greater_than_u32_max() {
    use super::AppletSize;
    use design_scaffold::validators::DataValidator;

    let applet_oid_str = format!("{}", i64::from(u32::MAX) + 1);

    let applet_oid = AppletSize::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn ok_on_validate_valid_applet_oid() {
    use super::AppletOid;
    use design_scaffold::validators::DataValidator;

    let applet_oid_str = "john.doe@example.net";

    let applet_oid = AppletOid::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_ok());
}

#[tokio::test]
async fn err_on_validate_invalid_applet_oid_user_part() {
    use super::AppletOid;
    use design_scaffold::validators::DataValidator;

    let applet_oid_str = "http://localhost/#john.doe@example.net";

    let applet_oid = AppletOid::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_applet_oid_domain_part() {
    use super::AppletOid;
    use design_scaffold::validators::DataValidator;

    let applet_oid_str = "john.doe@example.net:http://localhost/";

    let applet_oid = AppletOid::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_applet_oid_more_than_one_at_char() {
    use super::AppletOid;
    use design_scaffold::validators::DataValidator;

    let applet_oid_str = "john.doe@example.net@http://localhost/";

    let applet_oid = AppletOid::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_applet_oid_empty() {
    use super::AppletOid;
    use design_scaffold::validators::DataValidator;

    let applet_oid_str = "";

    let applet_oid = AppletOid::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_invalid_applet_oid_injection() {
    use super::AppletOid;
    use design_scaffold::validators::DataValidator;

    // Trying to inject: "john.<script>alert(1)</script>doe@example.net"
    let applet_oid_str = "john%2E%3Cscript%3Ealert%281%29%3C%2Fscript%3Edoe%40example%2Enet";

    let applet_oid = AppletOid::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_negavite_number() {
    use super::AppletId;
    use design_scaffold::validators::DataValidator;
    let applet_oid_str = "-1";

    let applet_oid = AppletId::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_zero() {
    use super::AppletId;
    use design_scaffold::validators::DataValidator;
    let applet_oid_str = "0";

    let applet_oid = AppletId::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_greater_than_u32_max() {
    use super::AppletId;
    use design_scaffold::validators::DataValidator;

    let applet_oid_str = format!("{}", i64::from(u32::MAX) + 1);

    let applet_oid = AppletId::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_filename_with_invalid_chars() {
    use super::AppletFilename;
    use design_scaffold::validators::DataValidator;

    let status_oid_str = "John%00Doe";

    let status_oid = AppletFilename::try_from(status_oid_str.to_owned()).unwrap();

    let test_result = status_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn ok_on_validate_applet_filenames_with_one_valid_entry() {
    use super::AppletFilename;
    use design_scaffold::validators::DataValidator;
    // TODO: Implement Fuzzing
    let filenames = [
        "0file_name.lua",
        "fi0e name.lua",
        "file_namelua",
        "file_name.txt",
        "file_name1.lua", // valid
        "file_name.ldua",
        " file_name.ldua",
        " file.name.ldua",
        "_file_name.ldua",
        " file.name_lua",
        " file.name..lua",
        "file_name..lua",
        "file_name...lua",
        "file_name.lu0a",
    ];

    let mut vec_ok_ocurrences: Vec<&str> = vec![];

    for filename in filenames {
        println!("Testing: [{}]", filename);
        let applet_filename = AppletFilename::from(filename.to_owned());
        match applet_filename.validate().await {
            Ok(_) => vec_ok_ocurrences.push(filename),
            Err(error) => println!("Error: [{:?}]", error),
        }
    }
    println!("Test finished!");
    if !vec_ok_ocurrences.is_empty() {
        println!("Ok ocurrences found: {:?}", vec_ok_ocurrences);
    }
    assert_eq!(vec_ok_ocurrences.len(), 1);
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_negavite_number() {
    use super::AppletCreatedAt;
    use design_scaffold::validators::DataValidator;
    let applet_oid_str = "-1";

    let applet_oid = AppletCreatedAt::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_zero() {
    use super::AppletCreatedAt;
    use design_scaffold::validators::DataValidator;
    let applet_oid_str = "0";

    let applet_oid = AppletCreatedAt::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_greater_than_u32_max() {
    use super::AppletCreatedAt;
    use design_scaffold::validators::DataValidator;

    let applet_oid_str = format!("{}", i64::from(u32::MAX) + 1);

    let applet_oid = AppletCreatedAt::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_code_with_invalid_chars() {
    use super::AppletCode;
    use design_scaffold::validators::DataValidator;

    let status_oid_str = "John%00Doe";

    let status_oid = AppletCode::try_from(status_oid_str.to_owned()).unwrap();

    let test_result = status_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_negavite_number() {
    use super::AppletUpdatedAt;
    use design_scaffold::validators::DataValidator;
    let applet_oid_str = "-1";

    let applet_oid = AppletUpdatedAt::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;
    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_zero() {
    use super::AppletUpdatedAt;
    use design_scaffold::validators::DataValidator;
    let applet_oid_str = "0";

    let applet_oid = AppletUpdatedAt::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    assert!(test_result.is_err());
}

#[tokio::test]
async fn err_on_validate_applet_oid_with_invalid_data_greater_than_u32_max() {
    use super::AppletUpdatedAt;
    use design_scaffold::validators::DataValidator;

    let applet_oid_str = format!("{}", i64::from(u32::MAX) + 1);

    let applet_oid = AppletUpdatedAt::try_from(applet_oid_str.to_owned()).unwrap();

    let test_result = applet_oid.validate().await;

    if let Err(error) = &test_result {
        dbg!(error); // `cargo test -- --nocapture`
    }

    assert!(test_result.is_err());
}
