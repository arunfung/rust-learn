use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::prelude::*;
use kv::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9527";
    // 连接服务器
    let stream = TcpStream::connect(addr).await?;

    // 使用 AsyncProstStream 来处理 TCP Frame
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();

    // 生成一个 HSET 命令
    let cmd = CommandRequest::new_hset("table1", "hello", "world".to_string().into());
    // 发送 HSET 命令
    client.send(cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got HET response {:?}", data);
    }

    // 生成一个 HEXIST 命令
    let exist_cmd = CommandRequest::new_hexist("table1", "hello");
    // 发送 HEXIST 命令
    client.send(exist_cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got HEXIST response 1 {:?}", data);
    }

    // 生成一个 HGET 命令
    let get_cmd = CommandRequest::new_hget("table1", "hello");
    // 发送 HGET 命令
    client.send(get_cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got GET response 1 {:?}", data);
    }

    // 生成一个 HDEL 命令
    let del_cmd = CommandRequest::new_hdel("table1", "hello");
    // 发送 HDEL 命令
    client.send(del_cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got HDEL response {:?}", data);
    }

    // 生成一个 HGET 命令
    let get_cmd = CommandRequest::new_hget("table1", "hello");
    // 发送 HGET 命令
    client.send(get_cmd).await?;
    if let Some(Ok(data)) = client.next().await {
        info!("Got GET response 2 {:?}", data);
    }

    Ok(())
}
