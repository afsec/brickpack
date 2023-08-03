use crate::{departments::model::DepartmentOid, DatabaseConnection};
use crate::{
    departments::{model::DepartmentName, NewDepartment},
    DatabaseConnection,
};
use design_scaffold::oid::{ObjectIdReactor, OidGenerator};
use design_scaffold::oid::{ObjectIdReactor, OidGenerator};
use sqlx::SqlitePool;

#[tokio::test]
async fn err_on_try_to_delete_nonexistent_department() {
    let oid_reactor = ObjectIdReactor::new().await.unwrap().take();

    let database = DatabaseConnection::new(&oid_reactor, ":memory:").await.unwrap();
    // let database = DatabaseConnection::new(":memory:").await.unwrap();
    let db_conn_pool = database.take();
    let option_oid_pool: Option<&OidReactorSynced> = Some(&oid_reactor);
    let department_oid = DepartmentOid::new(option_oid_pool).await.unwrap();

    let result = super::delete_department(&db_conn_pool, &department_oid).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn ok_on_try_to_delete_three_departments() {
    use crate::departments::model::DepartmentOid;
    let oid_reactor = ObjectIdReactor::new().await.unwrap().take();
    let database = DatabaseConnection::new(&oid_reactor, ":memory:").await.unwrap();
    let db_conn_pool = database.take();
    if let Err(error) = populate_with_sample_data(&db_conn_pool).await {
        dbg!(&error); // `cargo test -- --nocapture`
    }

    let mut detected_errors = 0;
    for _ in 1..=3 {
        let option_oid_pool: Option<&OidReactorSynced> = Some(&oid_reactor);
        let department_oid = DepartmentOid::new(option_oid_pool).await.unwrap();

        if let Err(error) = super::delete_department(&db_conn_pool, &department_oid).await {
            dbg!(&error); // `cargo test -- --nocapture`
            detected_errors += 1;
        }
    }

    assert!(detected_errors == 0);
}
async fn populate_with_sample_data(db_conn_pool: &SqlitePool) -> design_scaffold::Result<()> {
    // TODO: Should use other repos intead of build a query
    sqlx::query(
        r#"
            INSERT INTO `departments` ("id","name","is_deleted") VALUES (1,'IT',0);
            INSERT INTO `departments` ("id","name","is_deleted") VALUES (2,'Accounting',0);
            INSERT INTO `departments` ("id","name","is_deleted") VALUES (3,'Marketing',0);
            INSERT INTO `permissions` ("id","name","is_deleted") VALUES (1,'Administrator',0);
            INSERT INTO `permissions` ("id","name","is_deleted") VALUES (2,'Technical',0);
            INSERT INTO `permissions` ("id","name","is_deleted") VALUES (3,'Department',0);  
            INSERT INTO `statuses` ("id","name","is_deleted") VALUES (1,'Enabled',0);
            INSERT INTO `statuses` ("id","name","is_deleted") VALUES (2,'Disabled',0);
            INSERT INTO `statuses` ("id","name","is_deleted") VALUES (3,'Blocked',0);
            INSERT INTO `departments` ("email","name","department","permission","status","is_deleted")
                VALUES ('root@example.net','Charlie Root',1,1,1,0);
            INSERT INTO `departments` ("email","name","department","permission","status","is_deleted")
                VALUES ('admin@example.net','Administrator',1,2,1,0);
            INSERT INTO `departments` ("email","name","department","permission","status","is_deleted")
                VALUES ('staff@example.net','Staff',1,3,1,0); 
        "#,
    )
    .execute(db_conn_pool)
    .await?;
    Ok(())
}

#[tokio::test]
async fn ok_on_try_to_create_department() {
    // let database = DatabaseConnection::new(":memory:").await.unwrap();
    let oid_reactor = ObjectIdReactor::new().await.unwrap().take();

    let database = DatabaseConnection::new(&oid_reactor, ":memory:").await.unwrap();
    let db_conn_pool = database.take();

    let new_department = NewDepartment {
        // TODO: Check DepartmentEmail validation
        name: DepartmentName::from("OK".to_owned()),
    };
    let option_oid_pool: Option<&OidReactorSynced> = Some(&oid_reactor);
    let result = super::create_department(oid_generator, &db_conn_pool, &new_department).await;
    if let Err(error) = &result {
        dbg!(error); // `cargo test -- --nocapture`
    }
    assert!(result.is_ok());
}
