use crate::kv_entity::kv_server::command_request::RequestData;
use crate::kv_entity::kv_server::{value, CommandRequest, Hset, Kvpair, Value};

pub mod kv_server;

impl Kvpair {
    /// 创建一个新的 kv 键值对
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl CommandRequest {
    /// hset 命令
    // 尝试使用泛型来限定 table 和 key
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            value: Some(value::Value::String(value)),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            value: Some(value::Value::String(value.into())),
        }
    }
}
