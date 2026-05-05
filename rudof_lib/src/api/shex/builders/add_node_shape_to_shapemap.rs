use crate::{Result, Rudof, api::shex::ShExOperations, formats::IriNormalizationMode};

/// Builder for `add_node_shape_to_shapemap` operation.
///
/// Provides a fluent interface for adding a node/shape association
/// to the current shapemap (creating it if none is loaded).
pub struct AddNodeShapeToShapemapBuilder<'a> {
    rudof: &'a mut Rudof,
    node: &'a str,
    shape: Option<&'a str>,
    base_nodes: Option<&'a str>,
    base_shapes: Option<&'a str>,
    iri_mode: IriNormalizationMode,
}

impl<'a> AddNodeShapeToShapemapBuilder<'a> {
    pub(crate) fn new(rudof: &'a mut Rudof, node: &'a str) -> Self {
        Self {
            rudof,
            node,
            shape: None,
            base_nodes: None,
            base_shapes: None,
            iri_mode: IriNormalizationMode::default(),
        }
    }

    /// Sets the shape label to validate against (default: START).
    pub fn with_shape(mut self, shape: &'a str) -> Self {
        self.shape = Some(shape);
        self
    }

    /// Sets the base IRI for resolving node IRIs.
    pub fn with_base_nodes(mut self, base: &'a str) -> Self {
        self.base_nodes = Some(base);
        self
    }

    /// Sets the base IRI for resolving shape IRIs.
    pub fn with_base_shapes(mut self, base: &'a str) -> Self {
        self.base_shapes = Some(base);
        self
    }

    /// Sets the IRI normalization mode for node and shape selector strings.
    ///
    /// - [`IriNormalizationMode::Lax`] (default): bare `://` IRIs are auto-wrapped in `<>`.
    /// - [`IriNormalizationMode::Strict`]: no normalization; bare IRIs produce a parse error.
    pub fn with_iri_mode(mut self, mode: IriNormalizationMode) -> Self {
        self.iri_mode = mode;
        self
    }

    /// Executes the operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::add_node_shape_to_shapemap(
            self.rudof,
            self.node,
            self.shape,
            self.base_nodes,
            self.base_shapes,
            self.iri_mode,
        )
    }
}
