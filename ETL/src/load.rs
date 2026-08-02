use crate::conf::{Customers, Order, Products};
use crate::extraction::Response;
use chrono::Datelike;
use tokio_postgres::Client;

pub async fn load_response(
    client: &Client,
    response: Response,
) -> Result<(), Box<dyn std::error::Error>> {
    match response {
        Response::Customers(customers) => load_customers(client, customers).await?,
        Response::Products(products) => load_products(client, products).await?,
        Response::Orders(orders) => load_orders(client, orders).await?,
        Response::Order_details(_) => {}
    }

    Ok(())
}

async fn load_customers(
    client: &Client,
    customers: Vec<Customers>,
) -> Result<(), Box<dyn std::error::Error>> {
    for customer in customers {
        let city_name = normalized_name(customer.city, "Unknown");
        let country_name = normalized_name(customer.country, "Unknown");
        let country_id = resolve_country_id(client, &country_name).await?;
        let city_id = resolve_city_id(client, &city_name, &country_name).await?;

        client
            .execute(
                "INSERT INTO dim_location (city_id, city_name, country_id, country_name)
                 VALUES ($1, $2, $3, $4)
                 ON CONFLICT (city_id) DO NOTHING",
                &[&city_id, &city_name, &country_id, &country_name],
            )
            .await?;

        let customer_id = customer.customer_id as i32;
        client
            .execute(
                "INSERT INTO dim_customer (customer_id, first_name, last_name, city_id, country_id)
                 VALUES ($1, $2, $3, $4, $5)
                 ON CONFLICT (customer_id) DO UPDATE
                 SET first_name = EXCLUDED.first_name,
                     last_name = EXCLUDED.last_name,
                     city_id = EXCLUDED.city_id,
                     country_id = EXCLUDED.country_id",
                &[
                    &customer_id,
                    &customer.first_name,
                    &customer.last_name,
                    &city_id,
                    &country_id,
                ],
            )
            .await?;
    }

    Ok(())
}

async fn load_products(
    client: &Client,
    products: Vec<Products>,
) -> Result<(), Box<dyn std::error::Error>> {
    for product in products {
        let category_name = normalized_name(Some(product.category.clone()), "Unknown");
        let category_id = resolve_category_id(client, &category_name).await?;

        client
            .execute(
                "INSERT INTO dim_category (category_id, category_name)
                 VALUES ($1, $2)
                 ON CONFLICT (category_id) DO NOTHING",
                &[&category_id, &category_name],
            )
            .await?;

        let product_id = product.product_id as i32;
        client
            .execute(
                "INSERT INTO dim_product (product_id, product_name, category_id, category_name)
                 VALUES ($1, $2, $3, $4)
                 ON CONFLICT (product_id) DO UPDATE
                 SET product_name = EXCLUDED.product_name,
                     category_id = EXCLUDED.category_id,
                     category_name = EXCLUDED.category_name",
                &[&product_id, &product.product_name, &category_id, &category_name],
            )
            .await?;
    }

    Ok(())
}

async fn load_orders(
    client: &Client,
    orders: Vec<Order>,
) -> Result<(), Box<dyn std::error::Error>> {
    for order in orders {
        let status_name = normalized_name(Some(order.status.clone()), "Unknown");
        let status_id = resolve_status_id(client, &status_name).await?;

        client
            .execute(
                "INSERT INTO dim_status (status_id, status_name)
                 VALUES ($1, $2)
                 ON CONFLICT (status_id) DO NOTHING",
                &[&status_id, &status_name],
            )
            .await?;

        let date = chrono::NaiveDate::parse_from_str(&order.order_date, "%Y-%m-%d")?;
        let date_id = date.year() * 10000 + (date.month() as i32) * 100 + date.day() as i32;
        let quarter = ((date.month0() / 3) + 1) as i32;
        let month_name = date.format("%B").to_string();
        let day_name = date.format("%A").to_string();
        let day = date.day() as i32;
        let month = date.month() as i32;
        let year = date.year();

        client
            .execute(
                "INSERT INTO dim_date (date_id, full_date, day, month, year, quarter, month_name, day_name)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                 ON CONFLICT (date_id) DO NOTHING",
                &[&date_id, &date, &day, &month, &year, &quarter, &month_name, &day_name],
            )
            .await?;
    }

    Ok(())
}

fn normalized_name(value: Option<String>, fallback: &str) -> String {
    let trimmed = value.unwrap_or_else(|| fallback.to_string()).trim().to_string();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed
    }
}

async fn next_dim_id(client: &Client, table: &str, id_column: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let query = format!("SELECT COALESCE(MAX({id_column}), 0) + 1 FROM {table}");
    let row = client.query_one(&query, &[]).await?;
    Ok(row.get::<_, i32>(0))
}

async fn resolve_country_id(
    client: &Client,
    country_name: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    if let Some(row) = client
        .query_opt(
            "SELECT country_id FROM dim_location WHERE country_name = $1 ORDER BY country_id LIMIT 1",
            &[&country_name],
        )
        .await?
    {
        return Ok(row.get("country_id"));
    }

    next_dim_id(client, "dim_location", "country_id").await
}

async fn resolve_city_id(
    client: &Client,
    city_name: &str,
    country_name: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    if let Some(row) = client
        .query_opt(
            "SELECT city_id
             FROM dim_location
             WHERE city_name = $1 AND country_name = $2
             LIMIT 1",
            &[&city_name, &country_name],
        )
        .await?
    {
        return Ok(row.get("city_id"));
    }

    next_dim_id(client, "dim_location", "city_id").await
}

async fn resolve_category_id(
    client: &Client,
    category_name: &str,
) -> Result<i32, Box<dyn std::error::Error>> {
    if let Some(row) = client
        .query_opt(
            "SELECT category_id FROM dim_category WHERE category_name = $1 LIMIT 1",
            &[&category_name],
        )
        .await?
    {
        return Ok(row.get("category_id"));
    }

    next_dim_id(client, "dim_category", "category_id").await
}

async fn resolve_status_id(client: &Client, status_name: &str) -> Result<i32, Box<dyn std::error::Error>> {
    if let Some(row) = client
        .query_opt(
            "SELECT status_id FROM dim_status WHERE status_name = $1 LIMIT 1",
            &[&status_name],
        )
        .await?
    {
        return Ok(row.get("status_id"));
    }

    next_dim_id(client, "dim_status", "status_id").await
}
