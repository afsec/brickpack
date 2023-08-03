use super::outcome::ModelToViewMessage;
use super::view::AppletIdUpdated;
use super::UpdateApplet;
use application_models::applets::AppletToUpdate;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<
        'endpoint,
        UpdateApplet,
        Sqlite,
        AppletToUpdate,
        ModelToViewMessage,
        Json<AppletIdUpdated>,
    > for UpdateApplet
{
}
