use sqlx::types::chrono::NaiveDateTime;
use sqlx::SqlitePool;

use crate::applets::{
    model::{AppletSize, AppletUpdatedAt},
    AppletToUpdate,
};

// CRUD - UpdateOne
pub(super) async fn update_applet(
    db_conn_pool: &SqlitePool,
    applet_to_update: &AppletToUpdate,
) -> design_scaffold::Result<()> {
    tracing::debug!("AppletToUpdate: {:?}", applet_to_update);
    let applet_oid = match applet_to_update.get_oid() {
        Some(data) => data,
        None => {
            return Err(design_scaffold::Error::ImpossibleState(
                "Can't get applet_oid from Option".into(),
            ))
        }
    };
    let applet_oid_str = &*(*applet_oid);
    let mut total_rows_affected = 0;

    let updated_at: NaiveDateTime = AppletUpdatedAt::now().into();

    // * AppletToUpdate.filename
    if let Some(applet_filename) = &applet_to_update.filename {
        let applet_filename_str = &*(*applet_filename);
        let query_result = sqlx::query!(
            r#"
                UPDATE `applets`
                SET filename = ?2, updated_at = ?3
                WHERE oid = ?1
             ;
            "#,
            applet_oid_str,
            *applet_filename_str,
            updated_at,
        )
        .execute(db_conn_pool)
        .await?;
        total_rows_affected += query_result.rows_affected();
    }
    // * AppletToUpdate.code
    if let Some(applet_code) = &applet_to_update.code {
        let applet_code_str = &*(*applet_code);
        let size: i64 = AppletSize::from(applet_code_str.len()).into();
        let query_result = sqlx::query!(
            r#"
                UPDATE `applets`
                SET code = ?2, size = ?3, updated_at = ?4
                WHERE oid = ?1
             ;
            "#,
            applet_oid_str,
            *applet_code_str,
            size,
            updated_at
        )
        .execute(db_conn_pool)
        .await?;
        total_rows_affected += query_result.rows_affected();
    }

    tracing::debug!("AppletToUpdate: Total rows affected {}", total_rows_affected);

    Ok(())
}
