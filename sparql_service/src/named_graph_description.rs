use std::fmt::Display;

use iri_s::IriS;
use srdf::IriOrBlankNode;

use crate::{EntailmentProfile, EntailmentRegime, GraphDescription};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct NamedGraphDescription {
    id: Option<IriOrBlankNode>,
    name: IriS,
    graph: Option<GraphDescription>,
    supported_entailment_profile: Option<EntailmentProfile>,
    entailment_regime: Option<EntailmentRegime>,
}

impl NamedGraphDescription {
    pub fn new(id: Option<IriOrBlankNode>, name: IriS) -> Self {
        NamedGraphDescription {
            id,
            name,
            graph: None,
            supported_entailment_profile: None,
            entailment_regime: None,
        }
    }

    pub fn with_graph(mut self, graph: Option<GraphDescription>) -> Self {
        self.graph = graph;
        self
    }

    pub fn id(&self) -> &Option<IriOrBlankNode> {
        &self.id
    }
}

impl Display for NamedGraphDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            " NamedGraph {}",
            &self
                .id
                .as_ref()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "".to_string())
        )?;
        writeln!(f, " name: {}", self.name)?;
        if let Some(graph) = &self.graph {
            writeln!(f, " graph: {}", graph)?;
        }
        Ok(())
    }
}
