use axum::extract::{Path, Query};
use axum::Extension;
use crate::models::user::{User, UserParam};
use crate::models::result::{R, ToResult};
use crate::repositories::RepoExt;
use crate::service;

pub async fn index(Query(param): Query<UserParam>) -> R<UserParam> {
    // let users = usecases::users::search(repo.clone(), &conditions).await?;
    println!("{:?}", param);
    R::ok(param)
}

pub async fn list(Query(user_param): Query<UserParam>, Extension(repo): RepoExt) -> R<Vec<User>> {
    let users_result = service::users::search(repo.clone(), &user_param).await;
    users_result.to_result()
}

pub async fn find_by_id(Path(id): Path<i32>, Extension(repo): RepoExt) -> R<User> {
    service::users::view(repo.clone(), id).await.to_result()
}