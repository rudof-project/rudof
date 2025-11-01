use std::fmt::Display;

use either::Either;

use crate::{
    edge::Edge, evidence::Evidence, formal_base_type::FormalBaseType,
    label_property_spec::LabelPropertySpec, pgs::PropertyGraphSchema, pgs_error::PgsError,
};

#[derive(Debug, Clone)]
pub struct EdgeType {
    pub source: LabelPropertySpec,
    pub edge: LabelPropertySpec,
    pub target: LabelPropertySpec,
}
impl EdgeType {
    pub fn new(
        source: LabelPropertySpec,
        edge: LabelPropertySpec,
        target: LabelPropertySpec,
    ) -> Self {
        EdgeType {
            source,
            edge,
            target,
        }
    }

    pub fn semantics(&self, schema: &PropertyGraphSchema) -> Result<EdgeSemantics, PgsError> {
        // Placeholder for actual implementation
        Ok(EdgeSemantics {
            source: FormalBaseType::new(),
            edge: FormalBaseType::new(),
            target: FormalBaseType::new(),
        })
    }
}

impl Display for EdgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EdgeType(({})-[{}]->({}))",
            self.source, self.edge, self.target
        )
    }
}

pub struct EdgeSemantics {
    pub source: FormalBaseType,
    pub edge: FormalBaseType,
    pub target: FormalBaseType,
}

impl EdgeSemantics {
    pub fn conforms_edge(
        &self,
        type_name: &str,
        edge: &Edge,
    ) -> Either<Vec<PgsError>, Vec<Evidence>> {
        // Placeholder for actual implementation
        // This would check if the edge conforms to the semantics defined by this EdgeType
        Either::Right(vec![]) // Return empty evidence for now
    }
}
