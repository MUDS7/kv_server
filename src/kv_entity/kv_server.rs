/// 客户端命令集合
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandRequest {
    #[prost(
        oneof = "command_request::RequestData",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10"
    )]
    pub request_data: ::core::option::Option<command_request::RequestData>,
}
/// Nested message and enum types in `CommandRequest`.
pub mod command_request {
    #[derive(PartialOrd)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestData {
        #[prost(message, tag = "1")]
        Hget(super::Hget),
        #[prost(message, tag = "2")]
        Hgetall(super::Hgetall),
        #[prost(message, tag = "3")]
        Hmget(super::Hmget),
        #[prost(message, tag = "4")]
        Hset(super::Hset),
        #[prost(message, tag = "5")]
        Hmset(super::Hmset),
        #[prost(message, tag = "6")]
        Hdel(super::Hdel),
        #[prost(message, tag = "7")]
        Hmdel(super::Hmdel),
        #[prost(message, tag = "8")]
        Hexist(super::Hexist),
        #[prost(message, tag = "9")]
        Hmexist(super::Hmexist),
        #[prost(message, tag = "10")]
        Hstop(super::Hstop),
    }
}
/// 服务器响应
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    /// 状态码
    #[prost(uint32, tag = "1")]
    pub status: u32,
    /// 若访问失败，返回错误信息
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// 返回成功的数据
    #[prost(message, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<Value>,
    /// 返回成功的 kv数据
    #[prost(message, repeated, tag = "4")]
    pub kvpairs: ::prost::alloc::vec::Vec<Kvpair>,
}
/// 单条kv数据
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Kvpair {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
}
/// value 的几种类型
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(PartialOrd)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "1")]
        String(::prost::alloc::string::String),
        #[prost(bytes, tag = "2")]
        Binary(::prost::bytes::Bytes),
        #[prost(int64, tag = "3")]
        Integer(i64),
        #[prost(double, tag = "4")]
        Float(f64),
        #[prost(bool, tag = "5")]
        Bool(bool),
    }
}
/// 根据单个key 获取某个value
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hget {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// 获取所有的kv数据
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hgetall {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
}
/// 通过多个key 获取多个对应的value
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmget {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 设置kv
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hset {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pair: ::core::option::Option<Kvpair>,
}
/// 设置多条kv
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmset {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::prost::alloc::vec::Vec<Kvpair>,
}
/// 删除对应的key 并返回他的value
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hdel {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// 删除多个key 并返回他的value
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmdel {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 查看该key是否存在
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hexist {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// 查询多个key是否存在
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmexist {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// 关闭进程
#[derive(PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hstop {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
