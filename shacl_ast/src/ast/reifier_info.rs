use srdf::RDFNode;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct ReifierInfo {
    reification_required: bool,
    reifier_shape: Vec<RDFNode>,
}

impl ReifierInfo {
    pub fn new(reification_required: bool, reifier_shape: Vec<RDFNode>) -> Self {
        ReifierInfo {
            reification_required,
            reifier_shape,
        }
    }

    pub fn reification_required(&self) -> bool {
        self.reification_required
    }

    pub fn reifier_shape(&self) -> &Vec<RDFNode> {
        &self.reifier_shape
    }
}

impl Display for ReifierInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ReifierInfo {{ reificationRequired: {}, reifierShape: {} }}",
            self.reification_required,
            self.reifier_shape
                .iter()
                .map(|rdf_node| rdf_node.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
