use super::outcome::ModelToViewMessage;
use super::RunApplet;
use applet_runner::AppletRequestData;
use async_trait::async_trait;
use axum::response::Html;
use design_scaffold::endpoint::Presenter;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint>
    Presenter<'endpoint, RunApplet, Sqlite, AppletRequestData, ModelToViewMessage, Html<String>>
    for RunApplet
{
}
