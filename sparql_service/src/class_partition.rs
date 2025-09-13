use crate::PropertyPartition;
use iri_s::IriS;
use itertools::Itertools;
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Default, Debug, Hash)]
pub struct ClassPartition {
    class: IriS,
    property_partition: Vec<PropertyPartition>,
}

impl Display for ClassPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let props = self
            .property_partition
            .iter()
            .map(|pp| pp.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "ClassPartition(class: {}, properties: [{}])",
            self.class, props
        )
    }
}
