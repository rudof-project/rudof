use std::fmt::Display;

use iri_s::IriS;

#[derive(Debug, PartialEq, Clone)]
pub struct Var {
    name: String,
}

impl Var {
    pub fn new(name: &str) -> Var {
        Var { name: name.to_string() }
    }

    pub fn new_from_iri(_iri: &IriS, var_builder: &mut VarBuilder) -> Var {
        var_builder.generate()
    }
}

impl Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "?{}", self.name)
    }
}

pub struct VarBuilder {
    counter: usize,
    pattern: String,
}

impl VarBuilder {
    pub fn new() -> VarBuilder {
        VarBuilder {
            counter: 0,
            pattern: "var".to_string(),
        }
    }

    pub fn generate(&mut self) -> Var {
        self.counter += 1;
        let name = format!("{}{}", self.pattern, self.counter);
        Var::new(name.as_str())
    }
}

impl Default for VarBuilder {
    fn default() -> Self {
        Self::new()
    }
}
