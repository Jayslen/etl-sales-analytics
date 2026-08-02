use crate::conf::{Customers, Order, Products};
use crate::extraction::Response;
use csv;
use serde::de::DeserializeOwned;

pub fn extract(entity: &str, file_path: &str) -> Result<Response, Box<dyn std::error::Error>> {
    match entity {
        "customers" => Ok(Response::Customers(extract_csv::<Customers>(file_path)?)),
        "products" => Ok(Response::Products(extract_csv::<Products>(file_path)?)),
        "orders" => Ok(Response::Orders(extract_csv::<Order>(file_path)?)),
        _ => Err(format!("Unknown CSV entity: {}", entity).into()),
    }
}

pub fn extract_csv<T>(file_path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut data = Vec::new();

    for result in rdr.deserialize() {
        let record: T = result?;
        data.push(record);
    }

    Ok(data)
}
