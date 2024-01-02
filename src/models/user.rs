use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {

    //主键    
    pub id: i64,

    //用户名    
    pub username: String,

    //昵称    
    pub nickname: Option<String>,

    //密码    
    pub password: String,

    //盐值    
    pub salt: String,

    //手机号码    
    pub phone: Option<String>,

    //邮箱    
    pub email: Option<String>,

    //性别，0：女，1：男，默认1    
    pub gender: Option<i32>,

    //备注    
    pub remark: Option<String>,

    //状态，0：禁用，1：启用，2：锁定    
    pub state: i32,

    //创建人    
    pub create_user_id: i64,

    //创建时间    
    pub create_time: DateTime<Utc>,

    //修改人    
    pub update_user_id: i64,

    //修改时间
    pub update_time: DateTime<Utc>,

}


#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserParam {

    //主键
    pub id: i64,

    //用户名
    pub username: String,

    //昵称
    pub nickname: Option<String>,

    //密码
    pub password: String,

    //盐值
    pub salt: String,

    //手机号码
    pub phone: Option<String>,

    //邮箱
    pub email: Option<String>,

    //性别，0：女，1：男，默认1
    pub gender: Option<i32>,

    //备注
    pub remark: Option<String>,

    //状态，0：禁用，1：启用，2：锁定
    pub state: i32,

    //创建人
    pub create_user_id: i64,

    //创建时间
    pub create_time: DateTime<Utc>,

    //修改人
    pub update_user_id: i64,

    //修改时间
    pub update_time: DateTime<Utc>,


}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct UserQueryParam {

    //主键
    pub id: i64,

    //用户名
    pub username: String,

    //昵称
    pub nickname: Option<String>,

    //密码
    pub password: String,

    //盐值
    pub salt: String,

    //手机号码
    pub phone: Option<String>,

    //邮箱
    pub email: Option<String>,

    //性别，0：女，1：男，默认1
    pub gender: Option<i32>,

    //备注
    pub remark: Option<String>,

    //状态，0：禁用，1：启用，2：锁定
    pub state: i32,

    //创建人
    pub create_user_id: i64,

    //创建时间
    pub create_time: DateTime<Utc>,

    //修改人
    pub update_user_id: i64,

    //修改时间
    pub update_time: DateTime<Utc>,


}
