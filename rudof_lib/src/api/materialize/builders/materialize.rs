use crate::{
    Result, Rudof,
    api::materialize::MaterializeOperations,
    formats::ResultDataFormat,
};
use std::io;

/// Builder for `materialize` operation.
///
/// Provides a fluent interface for configuring and executing ShEx Map
/// materialization, producing an RDF graph from the current ShEx schema
/// and the Map semantic-action state.
pub struct MaterializeBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    initial_node_iri: Option<&'a str>,
    result_format: Option<&'a ResultDataFormat>,
}

impl<'a, W: io::Write> MaterializeBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// Called internally by `Rudof::materialize()`; not intended for direct
    /// public construction.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            initial_node_iri: None,
            result_format: None,
        }
    }

    /// Sets the IRI of the initial RDF node used as the root subject.
    ///
    /// When omitted, a fresh blank node is generated as the root subject.
    ///
    /// # Arguments
    ///
    /// * `iri` - A valid IRI string identifying the root subject node
    pub fn with_initial_node_iri(mut self, iri: &'a str) -> Self {
        self.initial_node_iri = Some(iri);
        self
    }

    /// Sets the RDF serialization format for the materialized graph.
    ///
    /// Defaults to Turtle when not specified.
    ///
    /// # Arguments
    ///
    /// * `format` - The output format to use for serialization
    pub fn with_result_format(mut self, format: &'a ResultDataFormat) -> Self {
        self.result_format = Some(format);
        self
    }

    /// Executes the materialization with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as MaterializeOperations>::materialize(
            self.rudof,
            self.initial_node_iri,
            self.result_format,
            self.writer,
        )
    }
}
