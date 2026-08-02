use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub extract: ExtractConfig,
    pub load: LoadConfig,
}

#[derive(Deserialize, Debug)]
pub struct ExtractConfig {
    pub api: ApiConfig,
    pub csv: CsvConfig,
}

#[derive(Deserialize, Debug)]
pub struct ApiConfig {
    pub base_url: String,
    pub default_limit: usize,
    pub resources: Vec<ApiResource>,
}

#[derive(Deserialize, Debug)]
pub struct ApiResource {
    pub entity: String,
    pub endpoint: String,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct CsvConfig {
    pub resources: Vec<CsvResource>,
}

#[derive(Deserialize, Debug)]
pub struct LoadConfig {
    pub postgres_url: String,
}

#[derive(Deserialize, Debug)]
pub struct CsvResource {
    pub entity: String,
    pub path: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"))?;

    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

#[derive(Debug, Deserialize)]
pub struct Products {
    #[serde(rename = "ProductID", alias = "product_id")]
    pub product_id: usize,

    #[serde(rename = "ProductName", alias = "product_name")]
    pub product_name: String,

    #[serde(rename = "Category", alias = "category")]
    pub category: String,

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

    // CSV usually gives City, API can provide city/city_id
    #[serde(rename = "City", alias = "city", alias = "city_id")]
    pub city: Option<String>,

    #[serde(rename = "Country", alias = "country", default)]
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
