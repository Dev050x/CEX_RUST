use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::{
    net::{TcpStream},
    sync::broadcast::{self, Sender},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum ClientMessage {
    SUBSCRIBE { market: String },
    UNSUBSCRIBE,
}

pub fn hadle_websocket_connection(stream: TcpStream, depth_tx: Sender<String>) {
    let mut push_task: Option<tokio::task::JoinHandle<()>> = None;

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
                ClientMessage::SUBSCRIBE { market: _  } => {
                    if let Some(handle) = push_task.take() {
                        handle.abort();
                    }
                    let mut rx = depth_tx.subscribe();
                    println!("subscribe to broadcaseter");
                    let w = write.clone();
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
                        handle.abort();
                        println!("client unsubscribed");
                    }
                }
            }
        }

        if let Some(handle) = push_task.take() {
            handle.abort();
        }
        println!("connection closed");
    });
}