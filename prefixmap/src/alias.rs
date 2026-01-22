#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Alias {
    pub id: String,
}

impl Alias {
    pub fn new<S: Into<String>>(str: S) -> Self {
        Self { id: str.into() }
    }
}
