//! # Viewmodel for Produkidentifikation
//! This modul represents the visualisation of the productinformation.
//! The functions are diredctly coupled to a web-component in js.
//! The plan is to bind the field-data from here to the UI.

use std::rc::Rc;
use std::cell::RefCell;
use super::product::*;

/// # ViewProductIdent
/// This strcut represents the wasm-viewmodel for the Produktidentifikation view. 
pub struct ViewProductIdent {
    pub product: Rc<RefCell<Option<Product>>>,
}

impl ViewProductIdent {

    pub fn new (product: Rc<RefCell<Option<Product>>>) -> Self {
        ViewProductIdent {
            product,
        }
    }

    /// # visual_projectspecific_productdata
    /// Creates a web-component, that shows the specific informations of the product.
    /// 
    /// ## Parameter
    /// The function needs to know where to create the web-assembly
    pub fn visual_projectspecific_productdata(&self) {
        let borrowed = self.product.borrow();
        if let Some(prod) = borrowed.as_ref() {
            web_sys::console::log_1(&prod.serial_number.clone().into());
        }
    }
    
    pub fn visual_common_productdata(&self) {
        
    }
}