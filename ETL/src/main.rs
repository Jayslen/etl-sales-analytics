use crate::conf::load_config;
use std::env;

mod conf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let config = load_config(path)?;
    println!("Config: {:?}", config.api_url);
    Ok(())
}
