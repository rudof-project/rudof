use crate::{ClassPartition, PropertyPartition};
use serde::{Deserialize, Serialize};
use rdf::rdf_core::term::{IriOrBlankNode, literal::NumericLiteral};
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct GraphDescription {
    id: IriOrBlankNode,
    #[serde(skip_serializing_if = "Option::is_none")]
    triples: Option<NumericLiteral>,
    #[serde(skip_serializing_if = "Option::is_none")]
    classes: Option<NumericLiteral>,
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<NumericLiteral>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<NumericLiteral>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents: Option<NumericLiteral>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    property_partition: Vec<PropertyPartition>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    class_partition: Vec<ClassPartition>,
}

impl GraphDescription {
    pub fn new(id: &IriOrBlankNode) -> Self {
        GraphDescription {
            id: id.clone(),
            triples: None,
            class_partition: Vec::new(),
            property_partition: Vec::new(),
            classes: None,
            properties: None,
            entities: None,
            documents: None,
        }
    }

    pub fn with_triples(mut self, triples: Option<NumericLiteral>) -> Self {
        self.triples = triples;
        self
    }

    pub fn with_classes(mut self, classes: Option<NumericLiteral>) -> Self {
        self.classes = classes;
        self
    }

    pub fn with_properties(mut self, properties: Option<NumericLiteral>) -> Self {
        self.properties = properties;
        self
    }

    pub fn with_entities(mut self, entities: Option<NumericLiteral>) -> Self {
        self.entities = entities;
        self
    }

    pub fn with_documents(mut self, documents: Option<NumericLiteral>) -> Self {
        self.documents = documents;
        self
    }

    pub fn with_property_partition(mut self, property_partition: Vec<PropertyPartition>) -> Self {
        self.property_partition = property_partition;
        self
    }

    pub fn with_class_partition(mut self, class_partition: Vec<ClassPartition>) -> Self {
        self.class_partition = class_partition;
        self
    }
}

impl Display for GraphDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, " Graph {}", self.id)?;
        if let Some(triples) = &self.triples {
            writeln!(f, "  triples: {triples}")?;
        }
        if let Some(classes) = &self.classes {
            writeln!(f, "  classes: {classes}")?;
        }
        if let Some(properties) = &self.properties {
            writeln!(f, "  properties: {properties}")?;
        }
        if let Some(entities) = &self.entities {
            writeln!(f, "  entities: {entities}")?;
        }
        if let Some(documents) = &self.documents {
            writeln!(f, "  documents: {documents}")?;
        }
        let mut class_partition = self.class_partition.iter().peekable();
        if class_partition.peek().is_some() {
            writeln!(
                f,
                "  class_partition: {}",
                class_partition.map(|c| c.to_string()).collect::<Vec<_>>().join(", ")
            )?;
        }
        let mut property_partition = self.property_partition.iter().peekable();
        if property_partition.peek().is_some() {
            writeln!(
                f,
                "  property_partition: {}",
                property_partition.map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
            )?;
        }
        Ok(())
    }
}
