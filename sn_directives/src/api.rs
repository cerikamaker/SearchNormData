use super::directives::*;

pub fn get_directive() -> Directive{
    let name: String = String::from("NRL");
    let desc: String = String::from("Maschinenrichtlinie");
    Directive::Base(BaseDirective::new(name, desc))   
}
