mod user;

use axum::{
    routing::{get},
    Router,
};
use axum::routing::{delete, post, put};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/user", user_routes())
}

fn user_routes() -> Router {
    Router::new()
        .route("/:id", get(user::select_one))
        .route("/page", get(user::select_page))
        .route("/", post(user::insert))
        // .route("/", put(user::update))
        .route("/:id", delete(user::delete))
}

pub async fn root() -> &'static str {
    "hello"
}
