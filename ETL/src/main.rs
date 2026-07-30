use crate::conf::load_config;
use std::env;

mod conf;
mod extraction;

use extraction::{api, csv};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let config = load_config(path)?;
    //let d_api = api::extract(&config.endpoints[0], &config.api_url).await?;

    let d = csv::extract("orders", &config.csv.orders);
    println!("Total customers: {:?}", d);
    // if let Some(res) = d.get(0) {
    //     if let ApiResponse::Customers(customers) = res {
    //         println!("Total customers: {}", customers.len());

    //         // access 5th customer
    //         if let Some(customer) = customers.get(4) {
    //             println!("{:?}", customer);
    //         }
    //     }
    // }

    Ok(())
}
