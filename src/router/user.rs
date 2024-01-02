use axum::extract::{Path, Query};
use axum::{Extension, Json};
use crate::models::user::{User, UserParam, UserQueryParam};
use crate::error::Result;
use crate::repositories::RepoExt;
use crate::service;
use crate::models::result::{R, ToResult};


pub async fn select_page(Extension(repo): RepoExt,
    Query(user_query_param): Query<UserQueryParam>
) -> R<Vec<User>> {
    let users_res = service::user::select_page(repo.clone(), &user_query_param).await;
    users_res.to_result()
}

pub async fn select_one(Extension(repo): RepoExt, Path(id): Path<i32>) -> R<User> {
    let user_res = service::user::select_one(repo.clone(), id).await;
    user_res.to_result()
}

pub async fn insert(Extension(repo): RepoExt, Json(user_param): Json<UserParam>) -> R<()> {
    let res = service::user::insert(repo.clone(), user_param).await;
    res.to_result()
}



pub async fn delete(Extension(repo): RepoExt, Path(id): Path<i32>) -> R<()> {
    let res = service::user::delete(repo.clone(), id).await;
    res.to_result()
}
