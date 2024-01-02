use std::error::Error;

use chrono::{DateTime, Local, TimeZone};
use serde::{self, Deserialize, Deserializer, Serializer};
use serde::de::{Unexpected, Visitor};
use serde_json::ser::Formatter;

pub fn serialize<S>(
    date: &DateTime<Local>,
    serializer: S,
) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
{
    serializer.serialize_i64(date.timestamp_millis())//序列化直接转为时间戳
}


pub fn deserialize<'de, D>(
    deserializer: D,
) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
{
    let timestamp = deserializer.deserialize_any(I64)?;//定义了I64访问者来处理格式转换
    Ok(Local.timestamp_millis_opt(timestamp)?)
}

struct I64;

impl<'de> Visitor<'de> for I64 {
    type Value = i64;
    fn expecting<'a>(&self, formatter: &mut Formatter<'a>) -> std::fmt::Result {
        write!(formatter, "is an integer")
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E> where
        E: Error, {//这个方法在这个业务里没有实现必要，只是为了例子解释
        println!("v {}", v);
        Ok(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> where
        E: Error, {
        println!("v {}", v);
        Ok(v)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> where
        E: Error, {//这个例子如果不实现这个方法，就会报错
        println!("v {}", v);
        Ok(v as i64)
    }
}