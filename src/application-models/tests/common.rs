use application_models::DatabaseConnection;
use design_scaffold::oid::{ObjectIdReactor, OidPool};

pub(crate) async fn db_bootstrapping() -> (OidPool, DatabaseConnection) {
    // * OID Reactor
    let oid_pool: OidPool = ObjectIdReactor::new().await.unwrap();

    // * Database connection
    let sqlite_pool = DatabaseConnection::new(Some(&oid_pool), ":memory:").await.unwrap();

    (oid_pool, sqlite_pool)
}
