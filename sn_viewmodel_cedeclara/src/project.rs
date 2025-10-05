//! # Project
//! Some directives require deep information to the product.
//! To avoid any missunderstandings some project information can be inculded.

/// # Project
/// This struct is not representing the whole project.
/// It contains only information like projectnumber and ordernumber.

pub struct Project {
    pub project_number: String,
    pub order_number: String,
}

impl Project {
    pub fn new() -> Self {
        Self {
            project_number: String::from("---"),
            order_number: String::from("---"),
        }
    }
}