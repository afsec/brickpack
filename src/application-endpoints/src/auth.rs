use axum::{extract::Path, http::StatusCode, response::Json};

pub async fn check_auth(
    Path(endpoint): Path<String>,
) -> Result<Json<String>, (StatusCode, &'static str)> {
    tracing::debug!("Endpoint Found: /auth/{}", &endpoint);
    let body = "{}";
    Ok(Json(body.to_string()))
}

// async fn get_user(Path(user_oid) : Path<Uuid>) -> Json<User> {
//     // let user = find_user(user_oid).await;
//     let user = "{}";
//     Json(user)
// }

// async fn find_user(user_oid: Uuid) -> User {
//     // ...
// }
