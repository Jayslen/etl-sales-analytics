use crate::conf::{Customers, Order, Order_details, Products};
use std::fmt;

#[derive(Debug)]
pub enum Response {
    Products(Vec<Products>),
    Customers(Vec<Customers>),
    Orders(Vec<Order>),
    Order_details(Vec<Order_details>),
}

#[derive(Debug, Clone, Copy)]
pub enum Entity {
    Customers,
    Products,
    Orders,
}

impl Entity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Customers => "customers",
            Self::Products => "products",
            Self::Orders => "orders",
        }
    }
}

impl std::str::FromStr for Entity {
    type Err = ExtractError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "customers" => Ok(Self::Customers),
            "products" => Ok(Self::Products),
            "orders" => Ok(Self::Orders),
            _ => Err(ExtractError::UnknownEntity(value.to_string())),
        }
    }
}

#[derive(Debug)]
pub enum ExtractError {
    UnknownEntity(String),
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownEntity(entity) => write!(f, "Unknown entity: {entity}"),
        }
    }
}

impl std::error::Error for ExtractError {}

pub mod api;
pub mod csv;
pub mod database;
