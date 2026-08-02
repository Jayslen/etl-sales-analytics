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
