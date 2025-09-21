mod directive;
pub mod api;

use crate::directive::BasicDirective;

pub fn get_directive() -> BasicDirective{
    let name: String = String::from("MRL");
    let desc: String = String::from("Maschinenrichtlinie");
    BasicDirective::new(name, desc)   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = get_directive();
        assert_eq!(result.name, "MRL");
        println!("Beschreibung: {}", result.description);
    }
}
