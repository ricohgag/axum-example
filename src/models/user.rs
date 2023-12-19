use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub msg: Option<String>,
    pub age: Option<i16>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserParam {
    pub name: Option<String>,
    pub age: Option<i16>,
}