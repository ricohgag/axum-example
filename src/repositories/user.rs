use crate::error::Result;
use crate::models::user::{User, UserParam, UserQueryParam};
use crate::repositories::Repositories;
use crate::db::mysql::Db;
use axum::async_trait;
use anyhow::Context;


pub struct UserRepoImpl {
    pool: Db,
}

impl UserRepoImpl {
    pub fn new(pool: Db) -> Self {
        Self { pool: pool }
    }
}

// #[automock]
#[async_trait]
pub trait UserRepo {
    async fn find_all(&self, user_query_param: &UserQueryParam) -> Result<Vec<User>>;

    async fn find_by_id(&self, user_id: i32) -> Result<User>;

    async fn insert(&self, user_param: &UserParam) -> Result<()>;

    async fn update_by_id(&self, user_param: &UserParam) -> Result<()>;

    async fn delete_by_id(&self, id: i32) -> Result<()>;

}


#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn find_all(&self, user_query_param: &UserQueryParam) -> Result<Vec<User>> {
        let mut query = sqlx::query_as::<_, User>("select * from user");
        let result = query
            .fetch_all(&*self.pool)
            .await
            .context("DB ERROR (find all users)")?;
        Ok(result)
    }

    async fn find_by_id(&self, user_id: i32) -> Result<User> {
        let row = sqlx::query_as::<_, User>("select * from sys_user where id = $1")
            .bind(user_id)
            .fetch_one(&*self.pool)
            .await
            .context("DB ERROR (find user by id)")?;
        Ok(row)
    }

    async fn insert(&self, user_param: &UserParam) -> Result<()> {
        todo!()
    }

    async fn update_by_id(&self, user_param: &UserParam) -> Result<()> {
        todo!()
    }

    async fn delete_by_id(&self, id: i32) -> Result<()> {
        todo!()
    }
}
