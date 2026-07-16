mod utils;
mod types;

use std::{collections::HashMap, sync::Arc};

use crate::{types::{ClientRegistry, DepthStore}, utils::{
    depth_push_interval::depth_push_interval, redis_handler::read_redis_stream_data, websocket_handler::hadle_websocket_connection,
}};
use tokio::{
    net::TcpListener, sync::{Mutex, RwLock, broadcast::{self}},
};

mod store;

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("web socket server is listening on port: 9001");

    let depth_store: DepthStore = Arc::new(RwLock::new(HashMap::new()));
    let client_registery: ClientRegistry = Arc::new(Mutex::new(Vec::new()));

    let (depth_tx, _) = broadcast::channel::<String>(1000);

    tokio::spawn(read_redis_stream_data(depth_tx.clone(), depth_store.clone()));
    tokio::spawn(depth_push_interval(depth_store, client_registery.clone()));

    while let Ok((stream, _addr)) = listner.accept().await {
        let _ = hadle_websocket_connection(stream, depth_tx.clone(), client_registery.clone());
    }
}
