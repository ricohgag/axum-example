use std::sync::Arc;
use axum::Router;
use crate::db::mysql;
use crate::repositories::RepoImpls;
use crate::repositories::user::UserRepoImpl;
use crate::router::router;

pub async fn create_app() -> Router {
    router()
}

pub async fn create_repositories() -> RepoImpls {
    let db_pool = Arc::new(mysql::db_connect().await);
    RepoImpls::new(
        UserRepoImpl::new(db_pool.clone()),
    )
}