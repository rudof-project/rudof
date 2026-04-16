use crate::{Result, Rudof, api::materialize::implementations::materialize, formats::ResultDataFormat};
use std::io;

/// Operations for materializing RDF graphs from ShEx Map semantic-action state.
pub trait MaterializeOperations {
    /// Materializes an RDF graph from the currently loaded ShEx schema and MapState.
    ///
    /// The ShEx schema provides the structure, while the MapState supplies the
    /// concrete values obtained from ShEx validation with Map semantic actions.
    ///
    /// # Arguments
    ///
    /// * `initial_node_iri` - Optional IRI string used as the root subject of the
    ///   generated graph; a fresh blank node is minted when `None`
    /// * `result_format` - Optional RDF serialization format for the output
    ///   (defaults to Turtle when `None`)
    /// * `writer` - The destination to write the serialized RDF graph to
    ///
    /// # Errors
    ///
    /// Returns an error if no ShEx schema or MapState is loaded, if the IRI is
    /// invalid, if materialization fails, or if serialization fails.
    fn materialize<W: io::Write>(
        &self,
        initial_node_iri: Option<&str>,
        result_format: Option<&ResultDataFormat>,
        writer: &mut W,
    ) -> Result<()>;
}

impl MaterializeOperations for Rudof {
    fn materialize<W: io::Write>(
        &self,
        initial_node_iri: Option<&str>,
        result_format: Option<&ResultDataFormat>,
        writer: &mut W,
    ) -> Result<()> {
        materialize(self, initial_node_iri, result_format, writer)
    }
}
