use iri_s::IriS;
use serde::{Deserialize, Serialize};
use srdf::{IriOrBlankNode, numeric_literal::NumericLiteral};
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Default, Debug, Hash, Serialize, Deserialize)]
pub struct PropertyPartition {
    id: Option<IriOrBlankNode>,
    property: IriS,
    #[serde(skip_serializing_if = "Option::is_none")]
    triples: Option<NumericLiteral>,
}

impl PropertyPartition {
    pub fn new(property: &IriS) -> Self {
        PropertyPartition {
            id: None,
            property: property.clone(),
            triples: None,
        }
    }

    pub fn with_id(mut self, id: &IriOrBlankNode) -> Self {
        self.id = Some(id.clone());
        self
    }

    pub fn with_triples(mut self, triples: Option<NumericLiteral>) -> Self {
        self.triples = triples;
        self
    }

    pub fn property(&self) -> &IriS {
        &self.property
    }

    pub fn triples(&self) -> Option<NumericLiteral> {
        self.triples.clone()
    }
}

impl Display for PropertyPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Property partition:  property: {}{})",
            self.property,
            self.triples
                .as_ref()
                .map(|n| format!(", triples: {n}"))
                .unwrap_or_default()
        )
    }
}
