use types::engine::{OrderStatus, Side, TypeOfOrder};

pub fn side_to_str(side: &Side) -> &'static str {
    match side {
        Side::BUY => "buy",
        Side::SELL => "sell",
    }
}

pub fn type_to_str(t: &TypeOfOrder) -> &'static str {
    match t {
        TypeOfOrder::LIMIT => "limit",
        TypeOfOrder::MARKET => "market",
    }
}

pub fn status_to_str(status: &OrderStatus) -> &'static str {
    match status {
        OrderStatus::OPEN => "open",
        OrderStatus::PartialyFilled => "partially_filled",
        OrderStatus::FILLED => "filled",
        OrderStatus::CANCEL => "cancelled",
    }
}
