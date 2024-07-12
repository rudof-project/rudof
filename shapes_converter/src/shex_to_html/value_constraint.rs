use super::Name;

#[derive(Debug, PartialEq, Default, Clone)]
pub enum ValueConstraint {
    #[default]
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
