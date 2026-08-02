use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub api_url: String,
    pub api: Api,
    pub csv: Csv,
}

#[derive(Deserialize)]
pub struct Api {
    pub customers: String,
    pub products: String,
}

#[derive(Deserialize)]
pub struct Csv {
    pub customers: String,
    pub products: String,
    pub orders: String,
}

pub fn load_config(file_path: &String) -> Result<Config, Box<dyn std::error::Error>> {
    let content: String = fs::read_to_string(file_path).unwrap();

    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

#[derive(Debug, Deserialize)]
pub struct Products {
    #[serde(rename = "ProductID", alias = "product_id")]
    pub product_id: usize,

    #[serde(rename = "ProductName", alias = "product_name")]
    pub product_name: String,

    #[serde(rename = "CategoryID", alias = "category_id")]
    pub category_id: usize,

    #[serde(rename = "Price", alias = "price")]
    pub price: f64,

    #[serde(rename = "Stock", alias = "stock")]
    pub stock: usize,
}

#[derive(Debug, Deserialize)]
pub struct Customers {
    #[serde(rename = "CustomerID", alias = "customer_id")]
    pub customer_id: usize,

    #[serde(rename = "FirstName", alias = "first_name")]
    pub first_name: String,

    #[serde(rename = "LastName", alias = "last_name")]
    pub last_name: String,

    #[serde(rename = "Email", alias = "email")]
    pub email: String,

    #[serde(rename = "Phone", alias = "phone")]
    pub phone: String,

    // API gives city_id, CSV gives city name
    #[serde(alias = "city_id")]
    pub city: Option<String>,

    #[serde(default)]
    pub country: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    #[serde(rename = "OrderID")]
    pub order_id: usize,

    #[serde(rename = "CustomerID")]
    pub customer_id: usize,

    pub order_date: String,
    pub status: String,
}

#[derive(Debug)]
pub struct Order_details {
    pub order_id: i32,
    pub product_name: String,
    pub quantity: i32,
    pub price: f64,
    pub customer: String,
}
