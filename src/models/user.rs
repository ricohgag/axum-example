use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: Option<String>,
    pub nickname: Option<String>,

}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserParam {
    pub name: Option<String>,
}