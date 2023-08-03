mod model;
mod outcome;
mod presenter;
mod view;

use axum::{
    extract::Extension,
    headers::{HeaderMap, HeaderValue},
    http::StatusCode,
};
use design_scaffold::endpoint::Endpoint;

use sqlx::SqlitePool;
pub(crate) struct CountUsers;
impl Endpoint for CountUsers {}

use crate::api::X_TOTAL_COUNT;

pub(super) async fn handler(
    Extension(ref sqlite_pool): Extension<SqlitePool>,
) -> Result<(HeaderMap, ()), (StatusCode, String)> {
    use design_scaffold::endpoint::Presenter;
    tracing::info!("Endpoint Found: CountUsers - [HEAD /api/users]");
    let users_length =
        CountUsers::presenter(&CountUsers, None, &sqlite_pool, ()).await?.take().to_string();

    let mut headers = HeaderMap::new();

    let header_value = match HeaderValue::from_str(users_length.as_str()) {
        Ok(header_value) => header_value,
        Err(error) => {
            // TODO: Implement Error matching
            tracing::warn!("Handler error: {}", error.to_string());
            return Err((
                StatusCode::CONFLICT,
                "Error on serialization counted Users.".to_string(),
            ));
        }
    };
    headers.insert(X_TOTAL_COUNT, header_value);

    Ok((headers, ()))
}
