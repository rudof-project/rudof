#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Alias {
    pub id: String,
}

impl Alias {
    pub fn new(str: &str) -> Self {
        Self { id: str.to_owned() }
    }
}
