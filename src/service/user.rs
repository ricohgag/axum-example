use crate::error::Result;
use crate::models::user::{ User, UserParam};
use crate::repositories::{user::UserRepo, Repositories};
use std::sync::Arc;


pub async fn select_pages<R: Repositories>(repo: Arc<R>,
    user_param: &UserParam,
) -> Result<Vec<User>> {
    let users = repo.user().find_all(user_param).await?;
    Ok(users)
}

pub async fn select_one<R: Repositories>(repo: Arc<R>, user_id: i32) -> Result<User> {
    let user = repo.user().find_by_id(user_id).await?;
    Ok(user)
}

pub async fn insert<R: Repositories>(repo: Arc<R>, user: User) -> Result<()> {
    repo.user().insert(user).await?;
    Ok(())
}

pub async fn update<R: Repositories>(repo: Arc<R>, user: User) -> Result<()> {
    repo.user().update(user).await?;
    Ok(())
}

pub async fn delete<R: Repositories>(repo: Arc<R>, user_id: i32) -> Result<()> {
    repo.user().delete(user_id).await?;
    Ok(())
}
