pub struct Converter {
    pub name: String,
}

impl Converter {
    pub fn new(name: String) -> Converter {
        Converter { name: name }
    }

    pub fn greet(&self) {
        println!("Hello, {}", self.name)
    }
}
