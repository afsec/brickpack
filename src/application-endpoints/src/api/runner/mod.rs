mod run_applet_get;
mod run_applet_post;

use axum::Router;

pub(crate) async fn routes() -> Router {
    use axum::routing::get;

    Router::new().route("/runner/:id", get(run_applet_get::handler).post(run_applet_post::handler))
}
