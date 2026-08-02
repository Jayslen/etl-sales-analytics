use crate::conf::{CsvResource, Customers, Order, Products};
use crate::extraction::{Entity, Response};
use csv;
use serde::de::DeserializeOwned;

pub fn extract(entity: &str, file_path: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let resource = CsvResource {
        entity: entity.to_string(),
        path: file_path.to_string(),
    };

    extract_resource(&resource)
}

pub fn extract_resource(resource: &CsvResource) -> Result<Response, Box<dyn std::error::Error>> {
    match resource.entity.parse::<Entity>()? {
        Entity::Customers => Ok(Response::Customers(extract_csv::<Customers>(
            &resource.path,
        )?)),
        Entity::Products => Ok(Response::Products(extract_csv::<Products>(&resource.path)?)),
        Entity::Orders => Ok(Response::Orders(extract_csv::<Order>(&resource.path)?)),
    }
}

pub fn extract_csv<T>(file_path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let mut rdr = csv::Reader::from_path(file_path)?;
    let data = rdr.deserialize().collect::<Result<Vec<T>, csv::Error>>()?;
    Ok(data)
}
