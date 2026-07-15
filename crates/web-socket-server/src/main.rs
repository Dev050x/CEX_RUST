use futures_util::{SinkExt, StreamExt};
use redis::streams::StreamReadReply;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use types::engine::EngineResponse;

use crate::store::RedisManager;

mod store;

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("web socket server is listening on port: 9001");
    tokio::spawn(read_redis_stream_data());
    while let Ok((stream, addr)) = listner.accept().await {
        let _ = hadle_websocket_connection(stream);
    }
}

async fn read_redis_stream_data() {
    let manager = RedisManager::get_instance().await;
    let mut last_id = manager
        .get_last_stream_id("to-backend")
        .await
        .unwrap_or("0".to_string());

    loop {
        let Ok(result) = manager.read_message(&last_id).await else {
            println!("no data received in websocket server");
            continue;
        };

        let reply: StreamReadReply = redis::from_redis_value(result).unwrap();
        println!("websocket server received the data: {:?}",reply);

        for stream in reply.keys {
            for entry in stream.ids {
                last_id = entry.id.clone();

                if let Some(msg) = entry.map.get("message") {
                    let json_str = match msg {
                        redis::Value::BulkString(b) => std::str::from_utf8(b).unwrap(),
                        _ => continue,
                    };

                    if let Ok(engine_response) = serde_json::from_str::<EngineResponse>(json_str) {
                        println!("received engine response: {:?}", engine_response);
                    }
                }
            }
        }
    }
}

async fn hadle_websocket_connection(stream: TcpStream) {
    tokio::spawn(async move {
        let ws_stream = accept_async(stream).await.unwrap();
        let (mut write, mut read) = ws_stream.split();
        while let Some(Ok(msg)) = read.next().await {
            if msg.is_text() {
                println!("i received the msg: {:?}", msg);
                write.send(msg).await;
            }
        }
    });
}
