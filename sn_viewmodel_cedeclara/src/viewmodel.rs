//! # Viewmodel
//! Hier sind alle Viewmodels enthalten auf die JS zugreifen kann.
//! Damit sind auch alle wasm-bindgen Funktionen dort enthalten.
//! 
//! ## Kommunikation mit der Datenbank
//! DECLARA ist nun global definiert und es kann von Überall darauf zugegriffen werden.
//! Der Datenbank Zugriff erfolgt daher immer über das spezielle Viewmodel, welches direkt mit JS kommuniziert.

use serde::Serialize;
use crate::viewbaseproduct::*;

#[derive(Serialize)]
pub struct Viewmodel {
    baseproduct: ViewBaseProduct, 
}

impl Viewmodel {
    pub fn new() -> Self {
        Self { 
            baseproduct: ViewBaseProduct::new(),
         }
    }
}