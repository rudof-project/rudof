use crate::{
    Result, Rudof,
    api::shex::ShExOperations,
    formats::{InputSpec, ShapeMapFormat},
};

/// Builder for `load_shapemap` operation.
///
/// Provides a fluent interface for configuring and executing shape map loading
/// operations with optional parameters.
pub struct LoadShapemapBuilder<'a> {
    rudof: &'a mut Rudof,
    shapemap: &'a InputSpec,
    shapemap_format: Option<&'a ShapeMapFormat>,
    base_nodes: Option<&'a str>,
    base_shapes: Option<&'a str>,
}

impl<'a> LoadShapemapBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::load_shapemap()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, shapemap: &'a InputSpec) -> Self {
        Self {
            rudof,
            shapemap,
            shapemap_format: None,
            base_nodes: None,
            base_shapes: None,
        }
    }

    /// Sets the shape map format.
    ///
    /// # Arguments
    ///
    /// * `shapemap_format` - The format to use when loading the shape map
    pub fn with_shapemap_format(mut self, shapemap_format: &'a ShapeMapFormat) -> Self {
        self.shapemap_format = Some(shapemap_format);
        self
    }

    /// Sets the base IRI for resolving node IRIs.
    ///
    /// # Arguments
    ///
    /// * `base_nodes` - The base IRI for node resolution
    pub fn with_base_nodes(mut self, base_nodes: &'a str) -> Self {
        self.base_nodes = Some(base_nodes);
        self
    }

    /// Sets the base IRI for resolving shape IRIs.
    ///
    /// # Arguments
    ///
    /// * `base_shapes` - The base IRI for shape resolution
    pub fn with_base_shapes(mut self, base_shapes: &'a str) -> Self {
        self.base_shapes = Some(base_shapes);
        self
    }

    /// Executes the shape map loading operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::load_shapemap(
            self.rudof,
            self.shapemap,
            self.shapemap_format,
            self.base_nodes,
            self.base_shapes,
        )
    }
}
