use design_scaffold::oid::OidPool;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::SqlitePool;

use crate::applets::{
    model::{AppletCreatedAt, AppletOid, AppletSize},
    NewApplet,
};

// CRUD - CreateOne
pub(super) async fn create_applet(
    option_oid_pool: Option<&OidPool>,
    db_conn_pool: &SqlitePool,
    new_applet: &NewApplet,
) -> design_scaffold::Result<AppletOid> {
    tracing::debug!("NewApplet: {:?}", new_applet);
    let new_applet_oid = AppletOid::new(option_oid_pool).await?;
    let filename = &(*new_applet.filename);
    let code = &(*new_applet.code);
    let size: i64 = AppletSize::from(code.len()).into();
    let created_at: NaiveDateTime = AppletCreatedAt::now().into();
    let updated_at = &created_at;
    let _ = sqlx::query!(
        r#"
            INSERT INTO `applets` ("oid","filename","code","size","created_at","updated_at","is_deleted")
            VALUES (?1,?2,?3,?4,?5,?6,0);
        "#,
        *new_applet_oid,
        filename,
        code,
        size,
        created_at,
        updated_at
    )
    .execute(db_conn_pool)
    .await?
    ;

    Ok(AppletOid::from(new_applet_oid.take()))
}
