use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv_server::kv_server::{CommandRequest, CommandResponse};
use kv_server::storage::memory::MemTable;
use kv_server::Service;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建监听端口
    let addr = "127.0.0.1:9000";
    let listener = TcpListener::bind(addr).await?;
    let table = MemTable::new();
    let service = Service::new(table);
    loop {
        let service_copy = service.clone();
        // 与客户端建立连接
        let (stream, addr) = listener.accept().await?;
        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            // 处理请求
            while let Some(response) = stream.next().await {
                match response {
                    Ok(message) => {
                        // todo 对client request做处理
                        dbg!(&message);
                        service_copy.execute(message);
                        // 返回client数据
                        let send_response = CommandResponse {
                            status: 200,
                            message: "hello".to_string(),
                            values: vec![],
                            kvpairs: vec![],
                        };
                        stream.send(send_response).await.unwrap();
                    }
                    Err(e) => {
                        dbg!(&e.to_string());
                    }
                }
            }
        });
    }
}
