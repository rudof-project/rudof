
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Alias { 
    pub id: String
}

impl Alias {
    pub fn from_str(str: &str) -> Alias { 
        Alias { id: str.to_owned() }
    }
}