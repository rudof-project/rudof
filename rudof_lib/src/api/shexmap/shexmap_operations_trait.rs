use crate::{
    Result, Rudof,
    api::shexmap::implementations::{
        shexmap_bind, shexmap_check, shexmap_load_bindings, shexmap_materialize, shexmap_serialize_bindings,
    },
    formats::{InputSpec, ResultDataFormat},
};
use shex_ast::Schema as ShExSchema;
use shex_ast::shexmap::{Accept, AnalysisReport, MapTriple};
use std::collections::HashMap;
use std::io;

/// What a materialization produced, beyond the graph written.
#[derive(Debug, Clone)]
pub struct ShexmapMaterialization {
    /// The materialization chosen: its triples, the bindings it read, and the provenance of
    /// each triple.
    pub chosen: Accept,
    /// How many distinct materializations the bindings allowed.
    pub alternatives: usize,
    /// When updating a graph in place: the triples added and removed.
    pub added: Vec<MapTriple>,
    pub removed: Vec<MapTriple>,
}

/// Operations for mapping RDF between ShEx schemas with `%Map:{ ... %}` semantic actions.
pub trait ShExMapOperations {
    /// Collects the ShExMap bindings of `focus` in the loaded RDF data against the loaded ShEx
    /// schema (the input schema), and keeps them as the current bindings.
    ///
    /// * `shape`: the shape label to start from; the schema's start when `None`
    /// * `base_nodes`, `base_shapes`: bases for relative IRIs in `focus` and `shape`
    /// * `strict`: fail when the input conforms in several ways that bind differently
    /// * `validate`: check conformance with the ShEx validator first, for its diagnostics
    #[allow(clippy::too_many_arguments)]
    fn shexmap_bind(
        &mut self,
        focus: &str,
        shape: Option<&str>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
        strict: bool,
        validate: bool,
    ) -> Result<()>;

    /// Reads bindings JSON (shex.js's and PyShEx's format) and keeps it as the current bindings.
    fn shexmap_load_bindings<R: io::Read>(&mut self, reader: R) -> Result<()>;

    /// Writes the current bindings as JSON.
    fn shexmap_serialize_bindings<W: io::Write>(&self, writer: &mut W, pretty: bool) -> Result<()>;

    /// Materializes the loaded ShEx schema (the output schema) from the current bindings and
    /// writes the graph to `writer`.
    ///
    /// * `root`: the node to build; a blank node when `None`
    /// * `shape`: the shape label to build; the schema's start when `None`
    /// * `static_vars`: extra variable values available everywhere, as terms (`"literal"`,
    ///   `<iri>`, `_:label`)
    /// * `into`: an existing graph to update in place: what the output schema currently holds
    ///   at `root` is replaced, the rest kept, and the whole graph written
    #[allow(clippy::too_many_arguments)]
    fn shexmap_materialize<W: io::Write>(
        &mut self,
        root: Option<&str>,
        shape: Option<&str>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
        static_vars: &HashMap<String, String>,
        into: Option<&InputSpec>,
        strict: bool,
        result_format: Option<&ResultDataFormat>,
        writer: &mut W,
    ) -> Result<ShexmapMaterialization>;

    /// Checks that the loaded ShEx schema (the output schema) can be materialized coherently
    /// from what `input_schema` binds: every output repetition has a list to iterate, every
    /// variable read is bound where it can be read from.
    fn shexmap_check(
        &self,
        input_schema: &ShExSchema,
        input_shape: Option<&str>,
        output_shape: Option<&str>,
        base_shapes: Option<&str>,
        static_vars: &[String],
    ) -> Result<AnalysisReport>;
}

impl ShExMapOperations for Rudof {
    fn shexmap_bind(
        &mut self,
        focus: &str,
        shape: Option<&str>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
        strict: bool,
        validate: bool,
    ) -> Result<()> {
        shexmap_bind(self, focus, shape, base_nodes, base_shapes, strict, validate)
    }

    fn shexmap_load_bindings<R: io::Read>(&mut self, reader: R) -> Result<()> {
        shexmap_load_bindings(self, reader)
    }

    fn shexmap_serialize_bindings<W: io::Write>(&self, writer: &mut W, pretty: bool) -> Result<()> {
        shexmap_serialize_bindings(self, writer, pretty)
    }

    fn shexmap_materialize<W: io::Write>(
        &mut self,
        root: Option<&str>,
        shape: Option<&str>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
        static_vars: &HashMap<String, String>,
        into: Option<&InputSpec>,
        strict: bool,
        result_format: Option<&ResultDataFormat>,
        writer: &mut W,
    ) -> Result<ShexmapMaterialization> {
        shexmap_materialize(
            self,
            root,
            shape,
            base_nodes,
            base_shapes,
            static_vars,
            into,
            strict,
            result_format,
            writer,
        )
    }

    fn shexmap_check(
        &self,
        input_schema: &ShExSchema,
        input_shape: Option<&str>,
        output_shape: Option<&str>,
        base_shapes: Option<&str>,
        static_vars: &[String],
    ) -> Result<AnalysisReport> {
        shexmap_check(self, input_schema, input_shape, output_shape, base_shapes, static_vars)
    }
}
