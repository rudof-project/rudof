use serde::Serialize;

use super::Name;
use super::Value;

#[derive(Serialize, Debug, PartialEq, Default, Clone)]
pub enum ValueConstraint {
    #[default]
    Any,
    Datatype(Name),
    Ref(Name),
    ValueSet(Vec<Value>),
    None,
}

impl ValueConstraint {
    pub fn datatype(name: Name) -> ValueConstraint {
        ValueConstraint::Datatype(name)
    }
}
