use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    sync::broadcast::{self, Sender},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::types::{ClientMessage, ClientRegistry, ClientSub};

pub fn hadle_websocket_connection(
    stream: TcpStream,
    depth_tx: Sender<String>,
    client_registery: ClientRegistry,
) {
    let mut push_task: Option<tokio::task::JoinHandle<()>> = None;
    let connection_id = uuid::Uuid::new_v4().to_string();

    tokio::spawn(async move {
        let ws_stream = accept_async(stream).await.unwrap();
        let (write, mut read) = ws_stream.split();
        let write = std::sync::Arc::new(tokio::sync::Mutex::new(write));
        println!("websocket connection established");
        while let Some(Ok(msg)) = read.next().await {
            let text = match msg {
                Message::Text(t) => t,
                Message::Close(_) => break,
                _ => continue,
            };

            let client_msg: ClientMessage = match serde_json::from_str(&text) {
                Ok(m) => m,
                Err(_) => continue,
            };
            println!("got the client msg: {:?}", client_msg);

            match client_msg {
                ClientMessage::SUBSCRIBE { params } => {
                    if let Some(handle) = push_task.take() {
                        handle.abort();
                    }
                    let mut rx = depth_tx.subscribe();
                    println!("subscribe to broadcaseter");
                    let data_params = params[0].clone();
                    let parts:Vec<&str> = data_params.split(".").collect();
                    let market = parts[1];

                    let w = write.clone();
                    let mut registery = client_registery.lock().await;
                    registery.push(ClientSub {
                        id: connection_id.clone(),
                        market: market.to_string().clone(),
                        write: write.clone(),
                    });

                    push_task = Some(tokio::spawn(async move {
                        loop {
                            match rx.recv().await {
                                Ok(payload) => {
                                    let mut writer = w.lock().await;
                                    println!("sending data to websocket client");
                                    if writer.send(Message::Text(payload.into())).await.is_err() {
                                        break;
                                    }
                                }
                                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                                Err(broadcast::error::RecvError::Closed) => break,
                            }
                        }
                    }))
                }
                ClientMessage::UNSUBSCRIBE => {
                    if let Some(handle) = push_task.take() {
                        let mut clients = client_registery.lock().await;
                        clients.retain(|c| c.id != connection_id);
                        handle.abort();
                        println!("client unsubscribed");
                    }
                }
            }
        }

        if let Some(handle) = push_task.take() {
            let mut clients = client_registery.lock().await;
            clients.retain(|c| c.id != connection_id);
            handle.abort();
        }
        println!("connection closed");
    });
}
