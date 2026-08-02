use crate::{
    conf::{ApiResource, Customers, Products},
    extraction::{Entity, Response},
};

use serde::de::DeserializeOwned;
use std::fmt;

#[derive(Debug)]
struct ApiError {
    entity: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Entity '{}' is not supported for API extraction",
            self.entity
        )
    }
}

impl std::error::Error for ApiError {}

pub async fn extract(
    endpoint: &str,
    api_url: &str,
) -> Result<Response, Box<dyn std::error::Error>> {
    let resource = ApiResource {
        entity: endpoint.to_string(),
        endpoint: endpoint.to_string(),
        limit: Some(100),
    };

    extract_resource(api_url, &resource, 100).await
}

pub async fn extract_resource(
    api_url: &str,
    resource: &ApiResource,
    default_limit: usize,
) -> Result<Response, Box<dyn std::error::Error>> {
    let entity = resource.entity.parse::<Entity>()?;
    let limit = resource.limit.unwrap_or(default_limit);
    let client = reqwest::Client::new();

    match entity {
        Entity::Customers => Ok(Response::Customers(
            extract_paginated::<Customers>(&client, api_url, &resource.endpoint, limit).await?,
        )),
        Entity::Products => Ok(Response::Products(
            extract_paginated::<Products>(&client, api_url, &resource.endpoint, limit).await?,
        )),
        Entity::Orders => Err(ApiError {
            entity: resource.entity.clone(),
        }
        .into()),
    }
}

pub async fn extract_paginated<T>(
    client: &reqwest::Client,
    api_url: &str,
    endpoint: &str,
    limit: usize,
) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let mut results = Vec::new();
    let mut offset = 0;

    loop {
        let url = format!("{}/{}?limit={}&offset={}", api_url, endpoint, limit, offset);

        let res = client.get(&url).send().await?;

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
