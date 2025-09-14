use std::fmt::Display;

use iri_s::IriS;
use srdf::IriOrBlankNode;

use crate::{EntailmentProfile, EntailmentRegime, GraphDescription};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct NamedGraphDescription {
    id: Option<IriOrBlankNode>,
    name: IriS,
    graphs: Vec<GraphDescription>,
    supported_entailment_profile: Option<EntailmentProfile>,
    entailment_regime: Option<EntailmentRegime>,
}

impl NamedGraphDescription {
    pub fn new(id: Option<IriOrBlankNode>, name: IriS) -> Self {
        NamedGraphDescription {
            id,
            name,
            graphs: Vec::new(),
            supported_entailment_profile: None,
            entailment_regime: None,
        }
    }

    pub fn with_graphs(mut self, graphs: Vec<GraphDescription>) -> Self {
        self.graphs = graphs;
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
            &self.id.as_ref().map(|n| n.to_string()).unwrap_or_default()
        )?;
        writeln!(f, " name: {}", self.name)?;
        if !self.graphs.is_empty() {
            writeln!(
                f,
                " graphs: [{}]",
                self.graphs
                    .iter()
                    .map(|g| g.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}
