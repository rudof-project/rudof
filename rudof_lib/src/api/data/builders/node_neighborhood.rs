use crate::{
    Result, Rudof,
    api::data::DataOperations,
    formats::{IriNormalizationMode, NodeInspectionMode},
    types::NodeNeighborhood,
};

/// Builder for `node_neighborhood` operation.
pub struct NodeNeighborhoodBuilder<'a> {
    rudof: &'a Rudof,
    node: &'a str,
    predicates: Option<&'a [String]>,
    mode: Option<&'a NodeInspectionMode>,
    depth: Option<usize>,
    iri_mode: IriNormalizationMode,
}

impl<'a> NodeNeighborhoodBuilder<'a> {
    pub(crate) fn new(rudof: &'a Rudof, node: &'a str) -> Self {
        Self {
            rudof,
            node,
            predicates: None,
            mode: None,
            depth: None,
            iri_mode: IriNormalizationMode::default(),
        }
    }

    pub fn with_predicates(mut self, predicates: &'a [String]) -> Self {
        self.predicates = Some(predicates);
        self
    }

    pub fn with_mode(mut self, mode: &'a NodeInspectionMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = Some(depth);
        self
    }

    pub fn with_iri_mode(mut self, iri_mode: IriNormalizationMode) -> Self {
        self.iri_mode = iri_mode;
        self
    }

    pub fn execute(self) -> Result<NodeNeighborhood<'a>> {
        <Rudof as DataOperations>::node_neighborhood(
            self.rudof,
            self.node,
            self.predicates,
            self.mode,
            self.depth,
            self.iri_mode,
        )
    }
}
