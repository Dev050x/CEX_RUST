use types::engine::GetDepthData;

use crate::messages::{
    TxChannelsBalance,
    types::{Depth, Request},
};

pub async fn handle_get_depth(correlation_id: String, data: GetDepthData, channels: &TxChannelsBalance) {
    let tx = match data.market.as_str() {
        "BTC" => &channels.btc,
        "SOL" => &channels.sol,
        "ETH" => &channels.eth,
        _ => {
            println!("no matching asset");
            return;
        }
    };
    if let Err(e) = tx.send(Request::DepthData(Depth {
            correlation_id,
        })).await {
        println!(
            "there is some error in sending to orderbook via channel {:?}",
            e
        );
    }
}
    