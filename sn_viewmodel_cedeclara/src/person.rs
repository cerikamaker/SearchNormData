//! # Definition of all Persons
//! A directive is defining a couple of roles also called persons here.
//! A person can be a natural one like a human oder legal on like a company or a notified body.


struct Address {
    street: Option<String>,
    plz: Option<String>,
    loacation: Option<String>,
}

/// # Natural Person
/// A natural person is always a human, the can have different roles in a company.
pub struct NaturalPerson {
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub function: Option<String>,
    pub address: Option<Address>,
}

pub struct LegalPerson {
    pub name: Option<String>,
    pub address: Option<Address>,
}

pub struct NotifiedBody {
    pub identifaction_number: Option<String>,
    pub name: Option<String>,
    pub address: Option<Address>,
}

/// # Person
/// The enum includes all possible types of persons.
pub enum Person {
    Natural(NaturalPerson),
    Legal(LegalPerson),
    NotifiedBody(NotifiedBody),
}

impl NaturalPerson {
    pub fn new() -> Self {
        Self {
            title: None,
            first_name: None,
            last_name: None,
            function: None,
            address: None,
        }
    }
}

impl LegalPerson {
    pub fn new() -> Self {
        Self {
            name: None,
            address: None,
        }
    }
}

impl NotifiedBody {
    pub fn new() -> Self {
        Self {
            identifaction_number: None,
            name: None,
            address: None,
        }
    }
}