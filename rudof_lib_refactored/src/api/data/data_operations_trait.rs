use crate::{
    Rudof, Result, 
    formats::{DataFormat, ResultDataFormat, InputSpec, DataReaderMode, NodeInspectionMode, ResultServiceFormat},
    api::data::implementations::{
        load_data, serialize_data, reset_data, load_service_description, serialize_service_description,
        reset_service_description, show_node_info
    }
};
use std::io;

/// Operations for managing RDF data.
pub trait DataOperations {
    /// Loads RDF data from one or more input sources.
    ///
    /// # Arguments
    ///
    /// * `data` - Array of input specifications defining data sources
    /// * `data_format` - The RDF format of the input data (uses default if None)
    /// * `base` - Optional base IRI for resolving relative IRIs (uses default if None)
    /// * `endpoint` - Optional SPARQL endpoint URL to load data from. If stablished it overrides data (uses None by default)
    /// * `reader_mode` - The parsing mode (uses default if None) 
    ///
    /// # Errors
    ///
    /// Returns an error if the data cannot be parsed or loaded.
    fn load_data(
        &mut self,
        data: &[InputSpec],
        data_format: Option<&DataFormat>,
        base: Option<&str>,
        endpoint: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()>;

    /// Serializes the current RDF data to a writer.
    ///
    /// # Arguments
    ///
    /// * `format` - Optional output format (uses default if None)
    /// * `writer` - The destination to write the serialized data to
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails.
    fn serialize_data<W: io::Write>(
        &self, 
        format: Option<&ResultDataFormat>, 
        writer: &mut W
    ) -> Result<()>;

    /// Resets the current data to an empty state.
    fn reset_data(&mut self);

    /// Loads a SPARQL service description from an input specification.
    ///
    /// # Arguments
    ///
    /// * `service` - Input specification defining the service description source
    /// * `format` - Optional format (uses default if None)
    /// * `reader_mode` - Optional parsing mode (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the service description cannot be parsed or loaded.
    fn load_service_description(
        &mut self,
        service: &InputSpec,
        format: Option<&DataFormat>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()>;

    /// Serializes the current service description to a writer.
    ///
    /// # Arguments
    ///
    /// * `format` - Optional output format for the service description (uses default if None)
    /// * `writer` - The destination to write the serialized service description to
    ///
    /// # Errors
    ///
    /// Returns an error if no service description is loaded or serialization fails.
    fn serialize_service_description<W: io::Write>(
        &self,
        format: Option<&ResultServiceFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current service description.
    fn reset_service_description(&mut self);

    /// Shows detailed information about a node in the current RDF data.
    ///
    /// # Arguments
    ///
    /// * `node` - Node identifier (IRI or prefixed name) to inspect
    /// * `predicates` - Optional list of predicates used to filter displayed relations
    /// * `show_node_mode` - Optional inspection mode controlling the level of detail (uses default if None)
    /// * `depth` - Optional maximum traversal depth when expanding related nodes (uses 1 by default)
    /// * `show_hyperlinks` - Whether hyperlinks should be included in the output (uses false by default)
    /// * `writer` - The destination to write the node information to
    ///
    /// # Errors
    ///
    /// Returns an error if the node information cannot be retrieved or serialized.
    fn show_node_info<W: io::Write>(
        &self,
        node: &str,
        predicates: Option<&[String]>,
        show_node_mode: Option<&NodeInspectionMode>,
        depth: Option<usize>,
        show_hyperlinks: Option<bool>,
        writer: &mut W,
    ) -> Result<()>;
}

impl DataOperations for Rudof {
    fn load_data(
        &mut self,
        data: &[InputSpec],
        data_format: Option<&DataFormat>,
        base: Option<&str>,
        endpoint: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()> {
        load_data(self, data, data_format, base, endpoint, reader_mode)
    }

    fn serialize_data<W: io::Write>(
        &self, 
        format: Option<&ResultDataFormat>, 
        writer: &mut W
    ) -> Result<()> {
        serialize_data(self, format, writer)
    }

    fn reset_data(&mut self) {
        reset_data(self)
    }

    fn load_service_description(
        &mut self,
        service: &InputSpec,
        format: Option<&DataFormat>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()> {
        load_service_description(self, service, format, reader_mode)
    }

    fn serialize_service_description<W: io::Write>(
        &self,
        format: Option<&ResultServiceFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_service_description(self, format, writer)
    }

    fn reset_service_description(&mut self) {
        reset_service_description(self)
    }

    fn show_node_info<W: io::Write>(
        &self,
        node: &str,
        predicates: Option<&[String]>,
        show_node_mode: Option<&NodeInspectionMode>,
        depth: Option<usize>,
        show_hyperlinks: Option<bool>,
        writer: &mut W,
    ) -> Result<()> {
        show_node_info(self, node, predicates, show_node_mode, depth, show_hyperlinks, writer)
    }
}