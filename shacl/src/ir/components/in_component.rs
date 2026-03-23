use rudof_rdf::rdf_core::term::Object;
use std::fmt::{Display, Formatter};

/// sh:in specifies the condition that each value node is a member of a provided
/// SHACL list.
///
/// https://www.w3.org/TR/shacl/#InConstraintComponent
#[derive(Debug, Clone)]
pub(crate) struct In {
    values: Vec<Object>,
}

impl In {
    pub fn new(values: Vec<Object>) -> Self {
        In { values }
    }

    pub fn values(&self) -> &Vec<Object> {
        &self.values
    }
}

impl Display for In {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let values = self.values()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "In[{values}]")
    }
}
