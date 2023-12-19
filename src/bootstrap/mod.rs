use std::sync::Arc;
use axum::{Extension, Router};

use crate::repositories::{create_repositories};
use crate::router::router;

pub async fn create_app() -> Router {
    let repositories = Arc::new(create_repositories().await);
    router().layer(Extension(repositories))
}