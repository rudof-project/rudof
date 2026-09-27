use crate::{
    Result, Rudof,
    api::shexmap::{ShExMapOperations, ShexmapMaterialization},
    formats::{InputSpec, ResultDataFormat},
};
use std::collections::HashMap;
use std::io;

/// Builder for the `shexmap_materialize` operation: build a graph conforming to the loaded
/// ShEx schema from the current ShExMap bindings.
pub struct ShexmapMaterializeBuilder<'a, W: io::Write> {
    rudof: &'a mut Rudof,
    writer: &'a mut W,
    root: Option<&'a str>,
    shape: Option<&'a str>,
    base_nodes: Option<&'a str>,
    base_shapes: Option<&'a str>,
    static_vars: HashMap<String, String>,
    into: Option<&'a InputSpec>,
    strict: bool,
    result_format: Option<&'a ResultDataFormat>,
}

impl<'a, W: io::Write> ShexmapMaterializeBuilder<'a, W> {
    pub(crate) fn new(rudof: &'a mut Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            root: None,
            shape: None,
            base_nodes: None,
            base_shapes: None,
            static_vars: HashMap::new(),
            into: None,
            strict: false,
            result_format: None,
        }
    }

    /// The node to build (default: a blank node).
    pub fn with_root(mut self, root: &'a str) -> Self {
        self.root = Some(root);
        self
    }

    /// The shape label to build (default: the schema's start).
    pub fn with_shape(mut self, shape: &'a str) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn with_base_nodes(mut self, base: &'a str) -> Self {
        self.base_nodes = Some(base);
        self
    }

    pub fn with_base_shapes(mut self, base: &'a str) -> Self {
        self.base_shapes = Some(base);
        self
    }

    /// Extra variable values available everywhere, as terms (`"literal"`, `<iri>`, `_:label`).
    pub fn with_static_vars(mut self, static_vars: HashMap<String, String>) -> Self {
        self.static_vars = static_vars;
        self
    }

    /// An existing graph to update in place: what the output schema currently holds at the
    /// root is replaced, the rest kept, and the whole graph written.
    pub fn with_into(mut self, into: &'a InputSpec) -> Self {
        self.into = Some(into);
        self
    }

    /// Fail when the bindings fit the output schema in more than one way.
    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    /// The RDF serialization of the graph written (default: Turtle).
    pub fn with_result_format(mut self, format: &'a ResultDataFormat) -> Self {
        self.result_format = Some(format);
        self
    }

    pub fn execute(self) -> Result<ShexmapMaterialization> {
        <Rudof as ShExMapOperations>::shexmap_materialize(
            self.rudof,
            self.root,
            self.shape,
            self.base_nodes,
            self.base_shapes,
            &self.static_vars,
            self.into,
            self.strict,
            self.result_format,
            self.writer,
        )
    }
}
