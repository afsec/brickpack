mod model;
mod outcome;
mod presenter;
mod view;

use std::collections::HashMap;

use crate::extractors::PathInput;
use applet_runner::{AppletCookies, AppletQueryString};
use application_models::applets::model::AppletOid;
use axum::{
    extract::{Extension, RawQuery, TypedHeader},
    headers::Cookie,
    http::{Method, StatusCode},
    response::Html,
};
use design_scaffold::endpoint::{Endpoint, Presenter};
use sqlx::SqlitePool;

struct RunApplet;
impl Endpoint for RunApplet {}

pub(super) async fn handler(
    method: Method,
    Extension(ref sqlite_pool): Extension<SqlitePool>,
    PathInput(applet_oid): PathInput<AppletOid>,
    RawQuery(query_input): RawQuery,
    cookies_input: Option<TypedHeader<Cookie>>,
) -> Result<Html<String>, (StatusCode, String)> {
    use applet_runner::AppletRequestData;
    tracing::info!("Endpoint Found: RunApplet - [GET /api/runner/:applet_oid]");
    let applet_oid_str = applet_oid.take();
    let mut request_data = AppletRequestData::new(applet_oid_str, method);

    if let Some(query_string) = query_input {
        tracing::debug!("Query_string: {query_string:?}");
        if let Ok(hashmap) = serde_qs::from_str::<HashMap<String, String>>(&query_string) {
            request_data.query_string(AppletQueryString::from(hashmap));
        }
    }

    if let Some(cookies) = cookies_input {
        tracing::debug!("Cookies: {cookies:?}");
        let hashmap: HashMap<String, String> =
            cookies.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        request_data.cookies(AppletCookies::from(hashmap));
    }

    RunApplet::presenter(&RunApplet, None, sqlite_pool, request_data).await
}
