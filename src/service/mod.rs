use crate::command_request::RequestData;
use crate::kv_server::CommandRequest;
use crate::storage::memory::MemTable;
use crate::storage::Storage;
use crate::{CommandResponse, KvError};
use std::sync::Arc;

pub mod command_service;

/// 对 Command 的处理的抽象
pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

/// Service 内部数据结构
pub struct ServiceInner<Store> {
    store: Store,
}

/// 执行请求的结构体,默认为 MemTable
pub struct Service<Store = MemTable> {
    inner: Arc<ServiceInner<Store>>,
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }
    /// 执行客户端的命令
    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        println!("收到命令: {:?}", cmd);
        let res = dispatch(cmd, &self.inner.store);
        println!("执行结果为: {:?}", res);
        res
    }
}

/// 从 store中处理 request请求
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(para)) => para.execute(store),
        Some(RequestData::Hgetall(para)) => para.execute(store),
        Some(RequestData::Hset(para)) => para.execute(store),
        Some(RequestData::Hstop(para)) => para.execute(store),
        _ => KvError::InvalidCommand("非法请求!".to_string()).into(),
    }
}
