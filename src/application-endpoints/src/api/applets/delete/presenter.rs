use super::outcome::ModelToViewMessage;
use super::view::AppletDeleted;
use super::DeleteApplet;
use application_models::applets::model::AppletOid;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, DeleteApplet, Sqlite, AppletOid, ModelToViewMessage, Json<AppletDeleted>>
    for DeleteApplet
{
}
