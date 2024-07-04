use super::Name;

#[derive(Debug, PartialEq)]
pub enum ValueConstraint {
    Any,
    Datatype(Name),
    Ref(Name),
    None,
}

impl ValueConstraint {
    pub fn datatype(name: Name) -> ValueConstraint {
        ValueConstraint::Datatype(name)
    }
}

impl Default for ValueConstraint {
    fn default() -> Self {
        ValueConstraint::Any
    }
}
