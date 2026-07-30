#[derive(Debug)]
pub enum Response {
    Products(Vec<Products>),
    Customers(Vec<Customers>),
    Orders(Vec<Order>),
    // Customers_csv(Vec<Customers_csv>),
}

use crate::conf::{Customers, Order, Products};

pub mod api {
    use crate::{
        conf::{Customers, Products},
        extraction::Response,
    };

    pub async fn extract(
        endpoint: &str,
        api_url: &str,
    ) -> Result<Vec<Response>, Box<dyn std::error::Error>> {
        let mut results = Vec::new();

        let limit = 100;
        let mut offset = 0;

        match endpoint {
            "customers" => {
                let mut all_customers: Vec<Customers> = Vec::new();

                loop {
                    let api = format!("{}/{}?limit={}&offset={}", api_url, endpoint, limit, offset);

                    let res = reqwest::get(&api).await?;

                    if !res.status().is_success() {
                        let text = res.text().await?;
                        return Err(format!("API error: {}", text).into());
                    }

                    let response: Vec<Customers> = res.json().await?;

                    if response.is_empty() {
                        break;
                    }

                    all_customers.extend(response);

                    offset += limit;
                }

                results.push(Response::Customers(all_customers));
            }

            "products" => {
                let mut all_products: Vec<Products> = Vec::new();

                loop {
                    let api = format!("{}/{}?limit={}&offset={}", api_url, endpoint, limit, offset);

                    let res = reqwest::get(&api).await?;

                    if !res.status().is_success() {
                        let text = res.text().await?;
                        return Err(format!("API error: {}", text).into());
                    }

                    let response: Vec<Products> = res.json().await?;

                    if response.is_empty() {
                        break;
                    }

                    all_products.extend(response);

                    offset += limit;
                }

                results.push(Response::Products(all_products));
            }

            _ => {
                return Err(format!("Unknown endpoint: {}", endpoint).into());
            }
        }

        Ok(results)
    }
}
pub mod csv {
    use csv;

    use crate::conf::{Customers, Order, Products};
    use crate::extraction::Response;

    pub fn extract(entity: &str, file_path: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let mut rdr = csv::Reader::from_path(file_path)?;

        match entity {
            "customers" => {
                let mut data = Vec::new();
                for result in rdr.deserialize() {
                    let record: Customers = result?;
                    data.push(record);
                }
                Ok(Response::Customers(data))
            }

            "products" => {
                let mut data = Vec::new();
                for result in rdr.deserialize() {
                    let record: Products = result?;
                    data.push(record);
                }
                Ok(Response::Products(data))
            }

            "orders" => {
                let mut data = Vec::new();
                for result in rdr.deserialize() {
                    let record: Order = result?;
                    data.push(record);
                }
                Ok(Response::Orders(data))
            }

            _ => Err(format!("Unknown CSV entity: {}", entity).into()),
        }
    }
}
