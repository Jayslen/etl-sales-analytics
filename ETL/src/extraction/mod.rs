use crate::conf::{Customers, Order, Order_details, Products};

#[derive(Debug)]
pub enum Response {
    Products(Vec<Products>),
    Customers(Vec<Customers>),
    Orders(Vec<Order>),
    Order_details(Vec<Order_details>),
}

pub mod api;
pub mod csv;
pub mod database;
