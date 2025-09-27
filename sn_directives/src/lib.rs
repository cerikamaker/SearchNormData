pub mod api;
mod directives;

#[cfg(test)]
mod tests {
    use crate::directives::Directive;

    use super::*;

    #[test]
    fn it_works() {
        let mut result = api::get_directive();
        match result {
            Directive::Base(ref mut base) =>{
                assert_eq!(base.name, "NRL");

            } 
            Directive::MRL(ref mut mrl) =>{

            }  
        }
        //result.change_name(String::from("NRL"));
        //assert_eq!(result.name, "NRL");
        //println!("Beschreibung: {}", result.description);
    }
}
