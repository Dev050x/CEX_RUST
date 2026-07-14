use sqlx::{Pool, Postgres};
use types::engine::CreateOrderResponseData;
use uuid::Uuid;

use crate::utils::{side_to_str, status_to_str, type_to_str};

pub async fn handle_create_order(
    data: CreateOrderResponseData,
    conn: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    println!("data that we've received: {:?}", data);

    let order_id_str = match &data.order_id {
        Some(id) => id,
        None => {
            println!("No order_id present, skipping DB writes: {}", data.msg);
            return Ok(());
        }
    };

    let order_id = Uuid::parse_str(order_id_str).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let user_id = Uuid::parse_str(&data.user_id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

    let mut tx = conn.begin().await?;

    let side_str = side_to_str(&data.order.side);
    let type_str = type_to_str(&data.order.r#type);
    let status_str = status_to_str(&data.status);
    let price_str = data.order.price.clone().unwrap_or_else(|| "0".to_string());

    sqlx::query!(
        r#"
        INSERT INTO orders (id, quantity, price, side, type, status, user_id, market)
        VALUES ($1, $2, $3, $4::text::side, $5::text::type, $6::text::order_status, $7, $8)
        "#,
        order_id,
        data.order.qty,
        price_str,
        side_str,
        type_str,
        status_str,
        user_id,
        data.order.market,
    )
    .execute(&mut *tx)
    .await?;

    for trade in &data.trades {
        let maker_user_id =
            Uuid::parse_str(&trade.maker_user_id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let taker_user_id =
            Uuid::parse_str(&trade.taker_user_id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let maker_order_id =
            Uuid::parse_str(&trade.maker_order_id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        let taker_order_id =
            Uuid::parse_str(&trade.taker_order_id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

        sqlx::query!(
            r#"
        INSERT INTO fills
            (maker_user_id, taker_user_id, maker_order_id, taker_order_id, price, quantity, market)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
            maker_user_id,
            taker_user_id,
            maker_order_id,
            taker_order_id,
            trade.price.to_string(),
            trade.fill_qty.to_string(),
            data.order.market,
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            r#"
        UPDATE orders
        SET
            status = CASE
                WHEN (
                    SELECT COALESCE(SUM(quantity::numeric), 0)
                    FROM fills
                    WHERE maker_order_id = $1
                ) >= quantity::numeric THEN 'filled'::order_status
                ELSE 'partially_filled'::order_status
            END,
            updated_at = now()
        WHERE id = $1
        "#,
            maker_order_id,
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
