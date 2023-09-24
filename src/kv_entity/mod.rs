use crate::kv_entity::kv_server::command_request::RequestData;
use crate::kv_entity::kv_server::{value, CommandRequest, Hset, Kvpair};

pub mod kv_server;

pub use kv_server::*;

impl Kvpair {
    /// 创建一个新的 kv 键值对
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
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
