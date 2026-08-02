use crate::{
    conf::{Customers, Products},
    extraction::Response,
};

use serde::de::DeserializeOwned;

pub async fn extract(
    endpoint: &str,
    api_url: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
    match endpoint {
        "customers" => {
            let data = extract_paginated::<Customers>(endpoint, api_url).await?;
            Ok(Response::Customers(data))
        }
        "products" => {
            let data = extract_paginated::<Products>(endpoint, api_url).await?;
            Ok(Response::Products(data))
        }
        _ => Err(format!("Unknown endpoint: {}", endpoint).into()),
    }
}

async fn extract_paginated<T>(
    endpoint: &str,
    api_url: &str,
) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let mut results = Vec::new();
    let limit = 100;
    let mut offset = 0;

    loop {
        let url = format!("{}/{}?limit={}&offset={}", api_url, endpoint, limit, offset);

        let res = reqwest::get(&url).await?;

        if !res.status().is_success() {
            let text = res.text().await?;
            return Err(format!("API error: {}", text).into());
        }

        let batch: Vec<T> = res.json().await?;

        if batch.is_empty() {
            break;
        }

        results.extend(batch);
        offset += limit;
    }

    Ok(results)
}
