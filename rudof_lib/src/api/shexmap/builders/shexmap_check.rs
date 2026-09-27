use crate::{Result, Rudof, api::shexmap::ShExMapOperations};
use shex_ast::Schema as ShExSchema;
use shex_ast::shexmap::AnalysisReport;

/// Builder for the `shexmap_check` operation: check that the loaded ShEx schema (the output
/// schema) can be materialized coherently from what an input schema binds.
pub struct ShexmapCheckBuilder<'a> {
    rudof: &'a Rudof,
    input_schema: &'a ShExSchema,
    input_shape: Option<&'a str>,
    output_shape: Option<&'a str>,
    base_shapes: Option<&'a str>,
    static_vars: Vec<String>,
}

impl<'a> ShexmapCheckBuilder<'a> {
    pub(crate) fn new(rudof: &'a Rudof, input_schema: &'a ShExSchema) -> Self {
        Self {
            rudof,
            input_schema,
            input_shape: None,
            output_shape: None,
            base_shapes: None,
            static_vars: Vec::new(),
        }
    }

    /// The input schema's shape label to start from (default: its start).
    pub fn with_input_shape(mut self, shape: &'a str) -> Self {
        self.input_shape = Some(shape);
        self
    }

    /// The output schema's shape label to build (default: its start).
    pub fn with_output_shape(mut self, shape: &'a str) -> Self {
        self.output_shape = Some(shape);
        self
    }

    pub fn with_base_shapes(mut self, base: &'a str) -> Self {
        self.base_shapes = Some(base);
        self
    }

    /// Variable IRIs that will be supplied as static variables.
    pub fn with_static_vars(mut self, static_vars: Vec<String>) -> Self {
        self.static_vars = static_vars;
        self
    }

    pub fn execute(self) -> Result<AnalysisReport> {
        <Rudof as ShExMapOperations>::shexmap_check(
            self.rudof,
            self.input_schema,
            self.input_shape,
            self.output_shape,
            self.base_shapes,
            &self.static_vars,
        )
    }
}
