// * Security Considerations
// Clients need to exercise care when reporting health information. Malicious actors
// could use this information for orchestrating attacks. In some cases the health
// check endpoints may need to be **authenticated** and institute role-based **access control**.
//
// * Reference: https://tools.ietf.org/id/draft-inadarei-api-health-check-01.html#rfc.section.1
//
use std::time::Duration;

use axum::{
    headers::{HeaderMap, HeaderValue},
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        StatusCode,
    },
};
use serde_json::json;

pub async fn health_check() -> Result<(StatusCode, HeaderMap, String), (StatusCode, String)> {
    tracing::info!("Endpoint Found: HealthCheck - [GET /health]");
    // * RESPONSE HEADERS
    let mut headers = HeaderMap::new();

    // Content-Type: application/health+json
    let content_type = match HeaderValue::from_str("application/health+json") {
        Ok(header_value) => header_value,
        Err(error) => {
            tracing::warn!("Unable to define HeaderValue. Reason: {}", error.to_string());
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to define HeaderValue.".to_string(),
            ));
        }
    };
    headers.insert(CONTENT_TYPE, content_type);

    // Cache-Control: max-age=3600
    let max_age = Duration::from_secs(3600);
    let header_value = format!("max-age={}", max_age.as_secs());
    let cache_control = match HeaderValue::from_str(header_value.as_str()) {
        Ok(header_value) => header_value,
        Err(error) => {
            tracing::warn!("Unable to define HeaderValue. Reason: {}", error.to_string());
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to define HeaderValue.".to_string(),
            ));
        }
    };
    headers.insert(CACHE_CONTROL, cache_control);

    // * RESPONSE BODY
    let body = json!({
            "status": "pass",
            "version": "1",
    })
    .to_string();

    Ok((StatusCode::OK, headers, body))
}
