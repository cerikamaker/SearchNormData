use serde::Serialize;
use wasm_bindgen::prelude::*;
use crate::with_declara;
use crate::with_declara_mut;


/// # ViewBaseProduct
#[derive(Serialize)]
#[wasm_bindgen]
pub struct ViewBaseProduct {
    productlist: Vec<ViewItemProduct>,
}

#[wasm_bindgen]
impl ViewBaseProduct {
    pub fn new() -> Self {
        Self{productlist: Vec::new()}
    }

    #[wasm_bindgen]
    pub fn list_all_products(&self) {
        with_declara!(declara, {
            let test = &declara.database;
        })
    }

    #[wasm_bindgen]
    pub fn write_product(&mut self) {

    }

    #[wasm_bindgen]
    pub fn read_product(&self) {

    }
}

#[derive(Serialize)]
#[wasm_bindgen]
pub struct ViewItemProduct {

}