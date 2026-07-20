use sqlx::{Pool, Postgres};
use types::engine::CancelOrderResponseData;
use uuid::Uuid;

use crate::utils::status_to_str;

pub async fn handle_delete_order(
    data: CancelOrderResponseData,
    conn: &Pool<Postgres>,
) -> Result<(), sqlx::Error> {
    println!("cancel order data received: {:?}", data);

    if data.order_id.is_empty() {
        println!("No order_id present, skipping DB writes: {}", data.msg);
        return Ok(());
    }

    let order_id =
        Uuid::parse_str(&data.order_id).map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

    let status_str = status_to_str(&data.status);

    let mut tx = conn.begin().await?;

    sqlx::query!(
        r#"
        UPDATE orders
        SET
            status = $2::text::order_status,
            updated_at = now()
        WHERE id = $1
        "#,
        order_id,
        status_str,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}