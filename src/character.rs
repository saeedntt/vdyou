pub struct Character {
    pub name: String
}

impl Character {
    fn new(name: String) -> Self {
        Character {
            name
        }
    }
}