pub mod api;
mod declaration;
mod person;
mod product;
mod project;
mod view_productident;

use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{to_value, from_value};

use crate::{declaration::Declaration, product::Product, view_productident::ViewProductIdent};


thread_local! {
    static DECLARA: RefCell<Option<Declara>> = RefCell::new(None);
}

#[wasm_bindgen]
pub struct Declara {
    datamodel: Declaration,
    view_productident: ViewProductIdent,
}

#[wasm_bindgen]
impl Declara {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Declara {
        let datamodel = Declaration::new();
        let product = Rc::clone(&datamodel.product);
        let view_productident = ViewProductIdent::new(product);
        Declara { 
            datamodel, 
            view_productident 
        }
    }
    
    #[wasm_bindgen]
    pub fn get_viewmodel_productident(&self){
        self.view_productident.visual_projectspecific_productdata();
    }

    #[wasm_bindgen]
    pub fn get_product_ident(&self) -> JsValue {
        let product = self.datamodel.product.borrow();
        to_value(&*product).unwrap()     
    }

    #[wasm_bindgen]
    pub fn set_product_ident(&self, js_value: JsValue) {
        let product: Product = from_value(js_value).unwrap();
        let mut prod_ref = self.datamodel.product.borrow_mut();
        *prod_ref = Some(product);
    }
    
}

    #[wasm_bindgen]
    pub fn hello_console() {
        let msg:JsValue = JsValue::from_str("Hello from Rust!");
        web_sys::console::log_1(&msg); 
    }
    
    #[wasm_bindgen]
    pub fn set_element_for_fun(){
        let window = web_sys::window().expect("Oh no, no window!");
        let document = window.document().expect("Oh no, no document");
        if let Some(element) = document.get_element_by_id(&"test"){
            element.set_inner_html("Hello from Rust");
        }
    }




/*
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
*/

#[cfg(test)]
mod tests {
    use crate::declaration::Declaration;

    use super::*;

    #[test]
    fn it_works() {
        let declaration: Declaration = api::getDeclaration();
        //let result = add(2, 2);
        //assert_eq!(declaration.manufacturer, Option::None);
    }
}
