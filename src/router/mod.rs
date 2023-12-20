mod users;
// mod user;

use axum::{
    routing::{get},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/users", user_routes())
}

fn user_routes() -> Router {
    Router::new()
        .route("/", get(users::index))
        .route("/find_all", get(users::list))
        .route("/find_one/:user_id", get(users::find_by_id))
}

pub async fn root() -> &'static str {
    "hello"
}
