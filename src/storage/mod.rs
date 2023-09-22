pub mod memory;

use crate::kv_server::Kvpair;
use crate::{KvError, Value};

/// kv_server的存储和查询操作
pub trait Storage {
    // 获取指定table中对应key的value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    // 插入值
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    // table中是否包含某个key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    // 删除某个值
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    // 获取某个table中所有的键值对
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    // 返回table中所有键值对的迭代器
    fn get_all_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}
