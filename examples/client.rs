use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv_server::kv_server::{CommandRequest, CommandResponse};
use std::io::Error;
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建 tcp 连接
    let addr = "127.0.0.1:9000";
    let stream = TcpStream::connect(addr).await?;
    // 创建 client
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();
    // 使用命令
    let command = CommandRequest::new_hset("table 1", "hello", "world".into());
    // 发送命令
    client.send(command).await?;
    if let Some(response) = client.next().await {
        // 处理请求
        match response {
            Ok(data) => {
                dbg!(&data);
            }
            Err(e) => {
                dbg!(format!("{}", e).as_str());
            }
        }
    }
    Ok(())
}
