//! # The Declaration of Conformity
//!
//! This module handles the complete declaration of conformity.
//! The api is commuicating between application and this module.

use super::person::*;
use super::product::*;
use super::project::*;

use std::rc::Rc;
use std::cell::RefCell;

/// # DirectiveModul
/// In this enum all modules that are existing are defined here. 
pub enum DirectiveModul {
    InternalControl,
    TypeExaminationProcedure,
    ComprehensiveQualityAssurance,
}

/// # CE Declaration of Conformity
///
/// This struct defines all fields a Declaration of Conformity could have.
/// All fields that are optional als provide the enum Option.
pub struct Declaration {
    pub manufacturer: Option<Person>,
    pub authorized_person_to_sign: Option<Person>,
    pub authorized_representative: Option<Person>,
    pub authorized_person_documentation: Option<Person>,
    pub notified_body: Option<Person>,
    pub product: Rc<RefCell<Option<Product>>>,
    pub project: Option<Project>,
    pub used_directives: Option<Vec<String>>,
    pub used_harmonised_standards: Option<Vec<String>>,
    pub used_nonharmonised_standards: Option<Vec<String>>,
    pub used_module: Option<DirectiveModul>,
}

impl Declaration {
    pub fn new() -> Self {
        Self {
            manufacturer: None,
            authorized_person_to_sign: None, 
            authorized_representative: None, 
            authorized_person_documentation: None, 
            notified_body: None, 
            product: Rc::new(RefCell::new(Some(Product::new()))), 
            project: None, 
            used_directives: None, 
            used_harmonised_standards: None, 
            used_nonharmonised_standards: None, 
            used_module: None, 
        }
    }
}
