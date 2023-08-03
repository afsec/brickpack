use super::outcome::ModelToViewMessage;
use super::ReadApplet;
use application_models::applets::{model::AppletOid, Applet};
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, ReadApplet, Sqlite, AppletOid, ModelToViewMessage, Json<Applet>>
    for ReadApplet
{
}
