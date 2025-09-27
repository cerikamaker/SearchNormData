
pub enum Directive {
    Base(BaseDirective),
    MRL(DirectiveMRL),
}


pub struct BaseDirective {
    pub name: String,
    pub description: String,
}

impl BaseDirective{
    pub fn new(name: String, desc: String) -> Self {
        Self { name, description: desc }
    }
}

impl BaseDirective {
   pub fn change_name(&mut self, name: String) {
       self.name = name;
   } 
}


pub struct DirectiveMRL{
    pub directive: BaseDirective,
}

impl DirectiveMRL {
    pub fn new(name: String, desc: String) -> Self{
        Self {
            directive: BaseDirective::new(name, desc),
        } 
    }
}