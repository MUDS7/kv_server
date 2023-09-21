use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum KvError {
    // 未找到表或者key
    #[error("Not found table:{0}, key:{1}")]
    NotFound(String, String),
    // 非法命令
    #[error("Cannot parse command: {0}")]
    InvalidCommand(String),
    // 插入失败
    #[error("insert error table:{0}, key:{1}")]
    InsertError(String, String),
}
