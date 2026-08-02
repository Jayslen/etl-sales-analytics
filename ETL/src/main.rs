use crate::conf::load_config;

mod conf;
mod extraction;
mod load;

use extraction::{api, csv};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    let (client, connection) = tokio_postgres::connect(&config.load.postgres_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(err) = connection.await {
            eprintln!("PostgreSQL connection error: {err}");
        }
    });

    for resource in &config.extract.api.resources {
        let data = api::extract_resource(
            &config.extract.api.base_url,
            resource,
            config.extract.api.default_limit,
        )
        .await?;
        load::load_response(&client, data).await?;
        println!("Loaded API resource: {}", resource.entity);
    }

    for resource in &config.extract.csv.resources {
        let data = csv::extract_resource(resource)?;
        load::load_response(&client, data).await?;
        println!("Loaded CSV resource: {}", resource.entity);
    }

    Ok(())
}
