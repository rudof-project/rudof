use crate::{EntailmentProfile, EntailmentRegime, GraphDescription};
use iri_s::IriS;
use rudof_rdf::rdf_core::term::IriOrBlankNode;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, Serialize, Deserialize)]
pub struct NamedGraphDescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<IriOrBlankNode>,
    name: IriS,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    graphs: Vec<GraphDescription>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supported_entailment_profile: Option<EntailmentProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn name(&self) -> &IriS {
        &self.name
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
                " graphs: {}",
                self.graphs.iter().map(|g| g.to_string()).collect::<Vec<_>>().join("\n")
            )?;
        }
        Ok(())
    }
}
