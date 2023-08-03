mod create;
mod delete;
mod read;
mod read_all;
mod read_count;
mod update;

use axum::Router;

pub(crate) async fn routes() -> Router {
    use axum::routing::get;

    Router::new()
        .route("/users", get(read_all::handler).post(create::handler).head(read_count::handler))
        .route("/users/:id", get(read::handler).patch(update::handler).delete(delete::handler))
}
