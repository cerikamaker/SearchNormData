pub struct BasicDirective {
    pub name: String,
    pub description: String,
}

impl BasicDirective{
    pub fn new(name: String, desc: String) -> Self {
        Self { name, description: desc }
    }
}

impl BasicDirective {
   pub fn change_name(&mut self, name: String) {
       self.name = name;
   } 
}
