mod utils;

use crate::utils::{
    redis_handler::read_redis_stream_data, websocket_handler::hadle_websocket_connection,
};
use tokio::{
    net::TcpListener,
    sync::broadcast::{self},
};

mod store;

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("web socket server is listening on port: 9001");
    let (depth_tx, _) = broadcast::channel::<String>(1000);
    tokio::spawn(read_redis_stream_data(depth_tx.clone()));
    while let Ok((stream, _addr)) = listner.accept().await {
        let _ = hadle_websocket_connection(stream, depth_tx.clone());
    }
}
