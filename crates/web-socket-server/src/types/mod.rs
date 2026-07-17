use std::{collections::HashMap, sync::Arc};

use futures_util::stream::SplitSink;
use serde::{Deserialize, Serialize};
use tokio::{
    net::TcpStream,
    sync::{Mutex, RwLock},
};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};
use types::engine::Depth;

pub struct ClientSub {
    pub id: String,
    pub market: String,
    pub write: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
}

pub type DepthStore = Arc<RwLock<HashMap<String, Depth>>>;
pub type ClientRegistry = Arc<Mutex<Vec<ClientSub>>>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "method")]
pub enum ClientMessage {
    SUBSCRIBE { params: [String; 1] },
    UNSUBSCRIBE,
}
// {"method":"SUBSCRIBE","params":["trade.SOL_USDC","bookTicker.SOL_USDC"],"id":"1"}
