use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Var {
    name: String,
}

impl Var {
    pub fn new(name: &str) -> Var {
        Var {
            name: name.to_string(),
        }
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.name)
    }
}
