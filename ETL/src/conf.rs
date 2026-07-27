use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_url: String,
    pub endpoints: Vec<String>,
    pub csv: Vec<String>,
}

pub fn load_config(file_path: &String) -> Result<Config, Box<dyn std::error::Error>> {
    let content: String = fs::read_to_string(file_path).unwrap();

    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

pub mod endpoints {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Products {
        product_id: usize,
        product_name: String,
        category_id: usize,
        price: f64,
        stock: usize,
    }

    #[derive(Deserialize, Debug)]
    pub struct Customers {
        customer_id: usize,
        first_name: String,
        last_name: String,
        email: String,
        phone: String,
        city_id: usize,
    }
}
