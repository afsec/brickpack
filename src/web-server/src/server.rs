use design_scaffold::{
    oid::{ObjectIdReactor, OidPool},
    AppResult,
};

use crate::config::WebServerConfig;

pub async fn run(config: WebServerConfig) -> AppResult<()> {
    dbg!(config);
    println!("[{}:{}] Hello, world!", file!(), line!());

    // * OID Reactor
    let oid_pool: OidPool = ObjectIdReactor::new().await?;

    let mut oid_reactor = oid_pool.write().await;
    let new_oid = oid_reactor.generate().await?;

    println!("[{}:{}] new_oid: [{new_oid}]", file!(), line!());

    Ok(())
}
