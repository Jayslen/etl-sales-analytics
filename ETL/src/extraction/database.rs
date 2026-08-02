use tokio_postgres::{self, NoTls};

use crate::{conf::Order_details, extraction::Response};
pub async fn extract() -> Result<Response, Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=admin dbname=sales",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    let rows = client.query("SELECT * FROM orders_record", &[]).await?;
    let orders: Vec<Order_details> = rows
        .iter()
        .map(|row| Order_details {
            order_id: row.get("order_id"),
            product_name: row.get("product_name"),
            quantity: row.get("quantity"),
            price: row.get("price"),
            customer: row.get("customer"),
        })
        .collect();
    Ok(Response::Order_details(orders))
}
