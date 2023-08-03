use crate::{
    applets::{
        model::{AppletCode, AppletFilename},
        NewApplet,
    },
    DatabaseConnection,
};
use design_scaffold::oid::{ObjectIdReactor, OidGenerator};

#[tokio::test]
async fn ok_on_try_to_create_applet() {
    let oid_reactor = ObjectIdReactor::new().await.unwrap().take();

    let database = DatabaseConnection::new(&oid_reactor, ":memory:").await.unwrap();
    let db_conn_pool = database.take();

    let new_applet = NewApplet {
        // TODO: Check AppletEmail validation
        filename: AppletFilename::from("OK".to_owned()),
        code: AppletCode::from("OK".to_owned()),
    };
    let option_oid_pool: Option<&OidReactorSynced> = Some(&oid_reactor);
    let result = super::create_applet(oid_generator, &db_conn_pool, &new_applet).await;
    if let Err(error) = &result {
        dbg!(error); // `cargo test -- --nocapture`
    }
    assert!(result.is_ok());
}
