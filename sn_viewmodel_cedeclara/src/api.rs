//! # CE-Declara API
//! 
//! This is the central api for CE-Declara.
//! All commands are realized here.
//! 
//! ## Introduction
//! 
//! This api provides a ce declaration of conformity.
//! The format of the ce delcation depends on two main issues:
//! 
//! - The selection of the applicable directives.
//! - The modules used within the directives.
//! 
//! ## Used Directives
//! 
//! In a declaration of conformity, all relevant directives can be listed to declare that the product complies with them.
//! Each directive has its own appendix, which defines what information must be declared.
//! By law, there is a minimum set of information that must be included in a declaration.
//! Most directives require only this minimum.
//! However, some, like the machine driective requiere additional information.
//! 
//! ## Modules used within the Directives
//! 
//! The law defines several modules, a company can follow to prove that the product complies with the applicable directives.
//! For the format of declaration, this means that depending on the module used, additional informations may be required.

use super::declaration::*;

pub fn getDeclaration() -> Declaration{
    let application_declaration = Declaration::new();
    application_declaration
}