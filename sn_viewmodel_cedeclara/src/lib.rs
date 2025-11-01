//! # Model - View Model CE-Declara
//! This is the wasm implementation for the model and view model of CE-Daclara.
//! It is holding the data and the data structure to view the data.
//!
//! ## Thread_local
//! Mit diesem Code stellen wir sich, dass nur eine Instanz von Declara erzeugt werden kann.
//! In der Webassembly als auch im JS.
//! 
//! ```rust,ignore
//! thread_local! {
//!    static DECLARA: RefCell<Option<Declara>> = RefCell::new(None);
//! }
//! ```
//! 
//! ### Technischer Hintergrund
//! Mit thread_local wird sichergestellt, dass jeder thread seine eigene Kopie von Declara erhält.
//! In Webanwendung gibt es nur einen Thread. Damit kann es nur eine Instanz geben.
//! RefCell wird benötigt, damit die Instanz auch geändert werden kann.
//!
//! ### Vorteil
//! DECLARA steht nun global zur Verfügung und es ist sichergestellt, dass nur eine einzige Instanz vorhanen ist.
//!
//! # Makros
//! Um den Zugriff auf das zentrale DECLARA zu vereinfach kommen zwei Makros zum einsatz.
//! Da wir das hier zum erstenmal machen soll auch hier das Makro kurz erläutert werden.
//! 
//! ## Aufbau eines Makros
//! Um ein Makro zu verwenden kann folgendes Template verwendet werden:
//! ```rust, ignore
//! #[macro_export]
//! macro_rules! macroname {
//!     ($parameter1:ident, $parameter2:block) => {
//!         let $paramter1 = 4;
//!         $paramter2;
//!     };
//! };
//! ```
//! Im Grunde ist das Makro nichts anderes als eine anonyme Funktion.
//! Die Parameter die wir oben Übergeben sind vom Typ ident und block.
//! 
//! |Parameter: | Beschreibung: |
//! |-----------|--------------|
//! | ident: | Die Variable kann man innerhalb des Makros verwenden. |
//! | block: | Wird verwendet um ganze Codezeilen hinzuzufügen. |
//!
mod database;
mod viewmodel;
mod viewbaseproduct;


use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{to_value, from_value};

use crate::{database::{Database, TAdress, Tables}, viewmodel::Viewmodel};

// Siehe Dokumentation oben.
thread_local! {
    static DECLARA: RefCell<Option<Declara>> = RefCell::new(None);
}

/// # Declara 
/// Die Struct enthält Datenbank und Viewmodel.
/// Die Felder, besonders das Viewmodel können nicht als public deklariert und JS übergeben werden.
/// Aus diesem Grund muss dies über eine gesonderte Funktion laufen.
#[wasm_bindgen]
pub struct Declara {
    database: Database,
    viewmodel: Viewmodel,
}

#[wasm_bindgen]
impl Declara {
    /// # Konstruktor
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsValue {
        let database:Database = Database::new();
        let viewmodel:Viewmodel = Viewmodel::new();
        let declara:Declara= Declara { database, viewmodel};
        DECLARA.with(|cell| {
            *cell.borrow_mut() = Some(declara);
        });
        DECLARA.with(|cell| {
            let declara = cell.borrow();
            let viewmodel = &declara.as_ref().unwrap().viewmodel;
            to_value(viewmodel).unwrap()
        })
    }
}

/// # Mutable Declara
/// Das Makro sorgt für mutablen Zugriff auf Declara.
/// Wichtig ist, dass die Verwendung von DECLARA innerhalb des Makros stattfinden muss.
/// 
/// > **Hinweis:**
/// >
/// > $declara enthält die Struct DECLARA mit ihren Implementierungen.
/// > Diese kann dann in dem Code der mit $body verwendet wird genutzt werden.
#[macro_export]
macro_rules! with_declara_mut {
    ($declara:ident, $body:block) => {
        crate::DECLARA.with(|cell| {
            let mut borrow = cell.borrow_mut();
            let mut $declara = borrow.as_mut().expect("DECLARA wurde nicht initialisiert!");
            $body
        })
    };
}

#[macro_export]
macro_rules! with_declara {
    ($declara:ident, $body:block) => {
        crate::DECLARA.with(|cell| {
            let borrow = cell.borrow();
            let $declara = borrow.as_ref().expect("DECLARA wurde nicht initialisiert!");
            $body
        })
    };
}

///ToDo: Die Funktion hier soll demnächst entfernt werden.
#[wasm_bindgen]
pub fn hello_console() {
    let msg:JsValue = JsValue::from_str("Hello from Rust!");
    web_sys::console::log_1(&msg); 
}

///ToDo: Die Funktion hier soll demnächst entfernt werden.
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
    use std::string;
    use crate::database::TProduct;

    use super::*;
    use wasm_bindgen_test::*;
    use wasm_bindgen::JsValue;
    use web_sys::console;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn it_works() {
        let decla: JsValue = Declara::new();
        with_declara_mut!(declara, {
            declara.database.create_new_product();
            let prod:TProduct = declara.database.create_new_product();
            console::log_1(&JsValue::from_f64(prod.id_product.unwrap() as f64));
        });
        //let result = add(2, 2);
        //assert_eq!(declaration.manufacturer, Option::None);
    }
}
