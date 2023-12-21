mod users;
mod user;

use axum::{
    routing::{get},
    Router,
};
use axum::routing::{delete, post, put};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/users", users_routes())
        .nest("/user", user_routes())
}

fn user_routes() -> Router {
    Router::new()
        .route("/:id", get(user::select_one))
        .route("/page", get(user::page))
        .route("/", post(user::insert))
        .route("/", put(user::update))
        .route("/:id", delete(user::delete))
}

fn users_routes() -> Router {
    Router::new()
        .route("/", get(users::index))
        .route("/find_all", get(users::list))
        .route("/find_one/:user_id", get(users::find_by_id))
}

pub async fn root() -> &'static str {
    "hello"
}
