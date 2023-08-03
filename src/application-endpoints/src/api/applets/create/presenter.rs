use super::CreateApplet;
use super::{outcome::ModelToViewMessage, view::AppletCreated};
use application_models::applets::NewApplet;
use async_trait::async_trait;
use axum::response::Json;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, CreateApplet, Sqlite, NewApplet, ModelToViewMessage, Json<AppletCreated>>
    for CreateApplet
{
}
