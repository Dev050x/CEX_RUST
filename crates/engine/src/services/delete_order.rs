use types::engine::DeleteOrderData;

use crate::messages::{
    TxChannelsBalance,
    types::{DeleteOrder, Request},
};

pub async fn handle_delete_order(
    correlation_id: String,
    data: DeleteOrderData,
    channels: &TxChannelsBalance,
) {
    let tx = match data.market.as_str() {
        "BTC" => &channels.btc,
        "SOL" => &channels.sol,
        "ETH" => &channels.eth,
        _ => {
            println!("no matching asset");
            return;
        }
    };
    if let Err(e) = tx
        .send(Request::DeleteOrderData(DeleteOrder {
            correlation_id,
            market: data.market,
            order_id: data.order_id,
        }))
        .await
    {
        println!(
            "there is some error in sending to orderbook via channel {:?}",
            e
        );
    }
}
