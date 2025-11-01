//! # CE-Declara Database
//! Das ist die Datenbank von CE-Daclara
//! Hier werden die Funktionen bereitgestellt, die erforderlich sind um die Daten zu verarbeiten.

use std::vec::Vec;
use std::collections::HashMap;

//ToDo: Klärung erforderlich ob wir Tables noch brauchen.
pub enum Tables {
    TAdress(TAdress),
    TProduct(TProduct),
    TDeclaration(TDeclaration),
    TNaturalPerson(TNaturalPerson),
    TLegalPerson(TLegalPerson),
    TNotifiedPerson(TNotifiedPerson),
}

/// # Tabelle
/// Die Tabelle hier ist generisch.
/// Damit kann jeder Datentyp hier gespeichert werden.
/// Die Zuordnungen erfolgen über IDs um redundante Datenhaltung zu verhindern.
/// Um mit den IDs einfacher umzugehen enthält die struct ein entsprechendes Feld in der die nächste ID gehlaten wird.
/// Die ID des Objektes wird im Objekt selbst und in der HashMap als u32 gehalten.
pub struct Table<T> {
    pub next_id: u32,
    pub map: HashMap<u32, T>,
}

impl<T: Identifiable> Table<T> {
    pub fn new() -> Self {
        Self { 
            next_id: 0,
            map: HashMap::new(),
        }
    }

    pub fn load(&mut self){
        /*
        ToDo: Hier Daten laden noch umsetzen.
        In dem Bereich werden die Daten geladen.
        Vorher wird die HashMap jedoch gelöscht.
        Wenn die Daten geladen sind erzeugen wir die nächste id.
         */
        self.next_id = Self::create_next_id(&self.map);
    }

    pub fn add(&mut self, mut table_value: T) -> u32{
        let used_id = self.next_id;
        table_value.sync_id(self.next_id);
        self.map.insert(self.next_id, table_value);
        self.next_id = self.next_id + 1;
        used_id
    }

    fn create_next_id(map: &HashMap<u32, T>) -> u32 {
        map.keys().max().copied().unwrap_or(0) + 1
    }

}

/// # Database
/// This struct represents the database of CE-Declara
/// That means, that all objects are stored in these tables.
pub struct Database {
    list_adress: Table<TAdress>,
    list_legal_persons: Table<TLegalPerson>,
    list_natural_persons: Table<TNaturalPerson>,
    list_notified_persons: Table<TNotifiedPerson>,
    list_declarations: Table<TDeclaration>,
    list_products: Table<TProduct>,
}

impl Database {
    /// ## Konstruktor
    pub fn new() -> Self {
       Self {
        list_adress: Table::new(),
        list_legal_persons: Table::new(),
        list_natural_persons: Table::new(),
        list_notified_persons: Table::new(),
        list_declarations: Table::new(),
        list_products: Table::new(),
       } 
    }

    /// Die Funktion erstellt ein neues leeres Produkt mit einer eindeutigen ID.
    pub fn create_new_product(&mut self) -> TProduct {
        //Wir erstellen hier das neue Produkt um es in die Tabelle zu schreiben und am Ende auszugeben.
        let newproduct = TProduct::new();
        //Als erstes legen wir das Produkt in die Tabelle damit wir eine neue ID bekommen
        let id = self.list_products.add(newproduct);
        let createdproduct = self.list_products.map.get(&id).unwrap();
        createdproduct.clone() 
    }

    pub fn add(&mut self, data: Tables) {
        match data {
            Tables::TAdress(adress) => {
                self.list_adress.add(adress);
            }
            Tables::TDeclaration(declaration) => {
                self.list_declarations.add(declaration);
            }
            
            Tables::TLegalPerson(legalperson) => {
                
            }
            
            Tables::TNaturalPerson(naturalperson) => {
                
            }
        
            Tables::TNotifiedPerson(notifiedperson) => {
            
            }
    
            Tables::TProduct(product) => {
            
            }
        }
    }
}

// Table for adresses
pub struct TAdress {
    pub id_adress: Option<u32>,
    pub street: Option<String>,
    pub plz: Option<String>,
    pub location: Option<String>,
}

impl TAdress {
    pub fn new(street: String, plz: String, location: String) -> Self {
        Self { 
            id_adress: Option::None,
            street: Option::Some(street),
            plz: Option::Some(plz),
            location: Option::Some(location),
        } 
    }
}

// Juristische Personen
pub struct TLegalPerson {
    pub id_legal_person: Option<u32>,
    pub name: Option<String>,
    pub id_adress: Option<u32>,
}

impl TLegalPerson {
    pub fn new(name: String, id_adress: u32) -> Self{
        Self{
            id_legal_person: Option::None,
            name: Option::Some(name),
            id_adress: Option::Some(id_adress),
        }
    }
}

// Natürliche Personen
pub struct TNaturalPerson {
    pub id_natural_person: Option<u32>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub function: Option<String>,
    pub id_adress: Option<u32>,
}

impl TNaturalPerson{
    pub fn new(first_name:String, last_name:String, function:String, id_adress:u32) -> Self{
        Self{
            id_natural_person: Option::None,
            first_name: Option::Some(first_name),
            last_name: Option::Some(last_name),
            function: Option::Some(function),
            id_adress: Option::Some(id_adress),
        }
    }
}

// Benannte Stellen
pub struct TNotifiedPerson {
    pub id_notified_person: Option<u32>,
    pub name: Option<String>,
    pub id_adress: Option<u32>,
}

impl TNotifiedPerson{
    pub fn new(name: String, id_adress: u32) -> Self{
        Self{
            id_notified_person: Option::None,
            name: Option::Some(name),
            id_adress: Option::Some(id_adress),
        }
    }
}

// Produkte
#[derive(Clone)]
pub struct TProduct {
    pub id_product: Option<u32>,
    pub name: Option<String>,
    pub model: Option<String>,
    pub typ: Option<String>,
    pub function: Option<String>,
    pub serial_number: Option<String>,
    pub charge_number: Option<String>,
    pub build_year: Option<String>,
    pub remarks: Option<String>,
}

impl TProduct {
    pub fn new() -> Self{
        Self{
            id_product: Option::None,
            name: Option::None,
            model: Option::None,
            typ: Option::None,
            function: Option::None,
            serial_number: Option::None,
            charge_number: Option::None,
            build_year: Option::None,
            remarks: Option::None,
        }
    }
}

// Deklarationen
pub struct TDeclaration {
    pub id_declaration: Option<u32>,
    pub id_manufacturer: Option<u32>,
    pub id_authorized_representative: Option<u32>,
    pub id_notified_person: Option<u32>,
    pub id_product: Option<u32>,
    pub id_authorized_person_sign: Option<Vec<u32>>,
    pub id_authorized_person_documentation: Option<Vec<u32>>,
    pub representative_type: Option<String>,
}

impl TDeclaration{
    pub fn new() -> Self{
        Self{
            id_declaration: Option::None,
            id_manufacturer: Option::None,
            id_authorized_representative: Option::None,
            id_notified_person: Option::None,
            id_product: Option::None,
            id_authorized_person_sign: Option::None,
            id_authorized_person_documentation: Option::None,
            representative_type: Option::None,
        }
    }
}

pub trait Identifiable {
    fn sync_id(&mut self, id:u32);
}

impl Identifiable for TAdress {
    fn sync_id(&mut self, id:u32) {
        self.id_adress = Some(id);
    } 
}

impl Identifiable for TDeclaration {
    fn sync_id(&mut self, id:u32) {
        self.id_declaration = Some(id);
    } 
}

impl Identifiable for TLegalPerson {
    fn sync_id(&mut self, id:u32) {
        self.id_legal_person = Some(id);
    } 
}

impl Identifiable for TNaturalPerson {
    fn sync_id(&mut self, id:u32) {
        self.id_natural_person = Some(id);
    }
}

impl Identifiable for TNotifiedPerson {
    fn sync_id(&mut self, id:u32) {
        self.id_notified_person = Some(id);
    }
}

impl Identifiable for TProduct {
    fn sync_id(&mut self, id:u32) {
        self.id_product = Some(id);
    }
}