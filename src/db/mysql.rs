use dotenv::dotenv;
use sqlx::{Pool, MySql, MySqlPool};
use std::env;
use std::sync::Arc;
use sqlx::mysql::MySqlConnectOptions;

pub type Db = Arc<Pool<MySql>>;

pub async fn db_connect() -> Pool<MySql> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let opts: MySqlConnectOptions = db_url.parse().unwrap();
    MySqlPool::connect_with(opts).await.unwrap()
}
