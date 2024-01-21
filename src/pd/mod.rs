use crate::command_request::RequestData;
use crate::kv_server::{CommandRequest, Hset, Kvpair};
use crate::{CommandResponse, Hget, Hgetall, Hstop, KvError, Value};
use http::StatusCode;
use prost::Message;

impl CommandRequest {
    // 包装 hget 的 new方法
    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }
    // 包装 get_all 的 new方法
    pub fn new_getall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }
    // 包装 hset 的 new方法
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }

    pub fn stop() -> Self {
        Self {
            request_data: Some(RequestData::Hstop(Hstop {
                key: "stop".to_string(),
            })),
        }
    }
}

/// 将 Value 转为 CommandResponse
impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![value],
            ..Default::default()
        }
    }
}

impl From<String> for CommandResponse {
    fn from(value: String) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as u32,
            message: value,
            ..Default::default()
        }
    }
}

/// 将 Vec<Kvpair> 转为 CommandResponse
impl From<Vec<Kvpair>> for CommandResponse {
    fn from(value: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            kvpairs: value,
            ..Default::default()
        }
    }
}

/// 将 KvError 转为 CommandResponse
impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: e.to_string(),
            values: vec![],
            kvpairs: vec![],
        };
        // todo 优化错误代码
        match e {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            _ => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
        }
        result
    }
}
