use axum::extract::{Path, Query};
use axum::{Extension, Json};
use crate::models::user::{User, UserParam};
use crate::error::Result;
use crate::repositories::RepoExt;
use crate::service;

pub async fn index(Query(param): Query<UserParam>) -> Result<Json<User>> {
    // let users = usecases::users::search(repo.clone(), &conditions).await?;
    println!("{:?}", param);
    let user = User{
        id: 1,
        name: param.name,
        age: param.age,
        msg: None,
    };
    
    Ok(Json(user))
}

pub async fn list(Query(user_param): Query<UserParam>, Extension(repo): RepoExt) -> Result<Json<Vec<User>>> {
    let user = service::users::search(repo.clone(), &user_param).await?;
    Ok(Json(user))
}

pub async fn find_by_id(Path(id): Path<i32>, Extension(repo): RepoExt) -> Result<Json<User>> {
    let user = service::users::view(repo.clone(), id).await?;
    Ok(Json(user))
}