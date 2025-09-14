use crate::PropertyPartition;
use iri_s::IriS;
use srdf::IriOrBlankNode;
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Default, Debug, Hash)]
pub struct ClassPartition {
    id: Option<IriOrBlankNode>,
    class: IriS,
    property_partition: Vec<PropertyPartition>,
}

impl ClassPartition {
    pub fn new(class: &IriS) -> Self {
        ClassPartition {
            id: None,
            class: class.clone(),
            property_partition: Vec::new(),
        }
    }

    pub fn with_id(mut self, id: &IriOrBlankNode) -> Self {
        self.id = Some(id.clone());
        self
    }

    pub fn with_property_partition(mut self, property_partition: Vec<PropertyPartition>) -> Self {
        self.property_partition = property_partition;
        self
    }

    pub fn class(&self) -> &IriS {
        &self.class
    }

    pub fn property_partition(&self) -> &Vec<PropertyPartition> {
        &self.property_partition
    }
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
