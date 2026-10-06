use crate::{Result, Rudof, api::shexmap::ShExMapOperations};

/// Builder for the `shexmap_bind` operation: collect ShExMap bindings from the loaded RDF
/// data against the loaded ShEx schema.
pub struct ShexmapBindBuilder<'a> {
    rudof: &'a mut Rudof,
    focus: &'a str,
    shape: Option<&'a str>,
    base_nodes: Option<&'a str>,
    base_shapes: Option<&'a str>,
    strict: bool,
    validate: bool,
}

impl<'a> ShexmapBindBuilder<'a> {
    pub(crate) fn new(rudof: &'a mut Rudof, focus: &'a str) -> Self {
        Self {
            rudof,
            focus,
            shape: None,
            base_nodes: None,
            base_shapes: None,
            strict: false,
            validate: true,
        }
    }

    /// The shape label to start from (default: the schema's start).
    pub fn with_shape(mut self, shape: &'a str) -> Self {
        self.shape = Some(shape);
        self
    }

    /// The base for relative IRIs in the focus node.
    pub fn with_base_nodes(mut self, base: &'a str) -> Self {
        self.base_nodes = Some(base);
        self
    }

    /// The base for relative IRIs in the shape label.
    pub fn with_base_shapes(mut self, base: &'a str) -> Self {
        self.base_shapes = Some(base);
        self
    }

    /// Fail when the input conforms in several ways that bind differently.
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// Check conformance with the ShEx validator first, for its diagnostics (default: true).
    pub fn with_validate(mut self, validate: bool) -> Self {
        self.validate = validate;
        self
    }

    pub fn execute(self) -> Result<()> {
        <Rudof as ShExMapOperations>::shexmap_bind(
            self.rudof,
            self.focus,
            self.shape,
            self.base_nodes,
            self.base_shapes,
            self.strict,
            self.validate,
        )
    }
}
