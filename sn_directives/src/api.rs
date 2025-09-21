use crate::directive::BasicDirective;

pub fn get_directive() -> BasicDirective{
    let name:String = String::from("MRL");
    let desc:String = String::from("Maschinenrichtlinie");
    BasicDirective::new(name, desc)
}