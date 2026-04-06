use crate::{
    Result,
    api::rdf_config::implementations::{load_rdf_config, reset_rdf_config, serialize_rdf_config},
    formats::{InputSpec, RdfConfigFormat, ResultRdfConfigFormat},
};
use std::io;

/// Operations for RDF-config.
///
/// RDF-config is a tool to generate SPARQL queries, schema diagrams, and files
/// required for Grasp, TogoStanza and ShEx validator from simple YAML-based
/// configuration files.
pub trait RdfConfigOperations {
    /// Loads an RDF-config specification from an input source.
    ///
    /// # Arguments
    ///
    /// * `rdf_config` - Input specification defining the RDF-config source
    /// * `rdf_config_format` - Optional RDF-config format (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the RDF-config specification cannot be parsed or loaded.
    fn load_rdf_config(&mut self, rdf_config: &InputSpec, rdf_config_format: Option<&RdfConfigFormat>) -> Result<()>;

    /// Serializes the current RDF-config model to a writer.
    ///
    /// # Arguments
    ///
    /// * `result_rdf_config_format` - Optional output format for the RDF-config model (uses default if None)
    /// * `writer` - The destination to write the serialized RDF-config model to
    ///
    /// # Errors
    ///
    /// Returns an error if no RDF-config model is loaded or serialization fails.
    fn serialize_rdf_config<W: io::Write>(
        &self,
        result_rdf_config_format: Option<&ResultRdfConfigFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current RDF-config model.
    fn reset_rdf_config(&mut self);
}

impl RdfConfigOperations for crate::Rudof {
    fn load_rdf_config(&mut self, rdf_config: &InputSpec, rdf_config_format: Option<&RdfConfigFormat>) -> Result<()> {
        load_rdf_config(self, rdf_config, rdf_config_format)
    }

    fn serialize_rdf_config<W: io::Write>(
        &self,
        result_rdf_config_format: Option<&ResultRdfConfigFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_rdf_config(self, result_rdf_config_format, writer)
    }

    fn reset_rdf_config(&mut self) {
        reset_rdf_config(self)
    }
}
