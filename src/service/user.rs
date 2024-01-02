use crate::error::Result;
use crate::models::user::{User, UserParam, UserQueryParam};
use crate::repositories::{user::UserRepo, Repositories};
use std::sync::Arc;


pub async fn select_page<R: Repositories>(repo: Arc<R>,
    user_query_param: &UserQueryParam,
) -> Result<Vec<User>> {
    let users = repo.user().find_all(user_query_param).await?;
    Ok(users)
}

pub async fn select_one<R: Repositories>(repo: Arc<R>, user_id: i32) -> Result<User> {
    let user = repo.user().find_by_id(user_id).await?;
    Ok(user)
}

pub async fn insert<R: Repositories>(repo: Arc<R>, user_param: UserParam) -> Result<()> {
    repo.user().insert(&user_param).await?;
    Ok(())
}

pub async fn update<R: Repositories>(repo: Arc<R>, user_param: UserParam) -> Result<()> {
    repo.user().update_by_id(&user_param).await?;
    Ok(())
}

pub async fn delete<R: Repositories>(repo: Arc<R>, user_id: i32) -> Result<()> {
    repo.user().delete_by_id(user_id).await?;
    Ok(())
}
