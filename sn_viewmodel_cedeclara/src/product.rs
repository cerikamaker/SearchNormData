//! # Product
//! Most of all directives if not all require to identify the specific product as an item.
//! That means the data on the declaratio must exactly describe the product.
//! So that there is no missunderstanding.

use serde::{Deserialize, Serialize};

/// # Product
/// This structure represents the product.
#[derive(Serialize, Clone, Deserialize)]
pub struct Product {
    pub name: String,
    pub model: String,
    pub typ: String,
    pub function: String,
    pub additional_information: String,
    pub build_year: String,
    pub charge_number: String,
    pub serial_number: String,
    pub order_number: String,
    pub project_number: String,
    pub image_base64: String,
}

impl Product {
    pub fn new() -> Self {
        Self {
            name: String::from("---"),
            model: String::from("---"),
            typ: String::from("---"),
            function: String::from("---"),
            additional_information: String::from("---"),
            build_year: String::from("---"),
            charge_number: String::from("---"),
            serial_number: String::from("---"),
            order_number: String::from("---"),
            project_number: String::from("---"),
            image_base64: String::from("---"),
        }
    }
}