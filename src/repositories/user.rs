use anyhow::Context;
use crate::db::mysql::Db;
use crate::error::Result;
use crate::models::user::{ User, UserParam};
use axum::async_trait;

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
    async fn find_all(&self, user_param: &UserParam) -> Result<Vec<User>>;
    // async fn add(&self, user_data: &NewUser) -> Result<UserId>;
    async fn find_by_id(&self, user_id: i32) -> Result<User>;
}

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn find_all(&self, user_param: &UserParam) -> Result<Vec<User>> {
        let mut query = sqlx::query_as::<_, User>("select id, username, email as msg, status as age from user");
        if let Some(name) = &user_param.name {
            query = sqlx::query_as::<_, User>("select id, username, email as msg, status as age from user where username LIKE ?")
                .bind(format!("%{}%", name));
        }
        let result = query
            .fetch_all(&*self.pool)
            .await.with_context(||"查询错误")?;
        Ok(result)
    }

    // async fn add(&self, user_data: &NewUser) -> Result<UserId> {
    //     let row = sqlx::query_as::<_, UserId>(
    //         r#"
    //         INSERT INTO users (name, msg, age)
    //         VALUES ($1, $2, $3)
    //         RETURNING id
    //         "#,
    //     )
    //     .bind(&user_data.name)
    //     .bind(&user_data.msg)
    //     .bind(&user_data.age)
    //     .fetch_one(&*self.pool)
    //     .await
    //     .context("DB ERROR (create user)")?;
    //     Ok(row)
    // }

    async fn find_by_id(&self, user_id: i32) -> Result<User> {
        let row = sqlx::query_as::<_, User>("select ida, username, email as msg, status as age from user where id = ?")
            .bind(user_id)
            .fetch_one(&*self.pool)
            .await
            .with_context(|| "DB find_by_id ERROR ()")?;
        Ok(row)
    }
}
