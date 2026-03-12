use super::Name;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum ValueConstraint {
    #[default]
    Any,
    ValueSet(Vec<Name>),
    Datatype(Name),
    Facet(Vec<Name>),
    Ref(Name),
    Kind(Name),
    None,
    And {
        values: Vec<ValueConstraint>,
    },
    Or {
        values: Vec<ValueConstraint>,
    },
    Not {
        value: Box<ValueConstraint>,
    },
}

impl ValueConstraint {
    pub fn datatype(name: Name) -> ValueConstraint {
        ValueConstraint::Datatype(name)
    }
}
