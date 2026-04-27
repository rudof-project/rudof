#![allow(unsafe_op_in_unsafe_fn)]
//! Python bindings for the Rudof RDF validation and manipulation library.
//!
//! This module provides Python wrappers for working with RDF data, ShEx and SHACL schemas,
//! SPARQL queries, and related semantic web technologies.

use crate::PyRudofConfig;
use pyo3::{PyErr, PyResult, Python, exceptions::PyValueError, pyclass, pymethods};
use rudof_lib::{
    Rudof,
    errors::{InputSpecError, RudofError},
    formats::{
        ComparisonFormat, ComparisonMode, ConversionFormat, ConversionMode, DCTapFormat, DataFormat, DataReaderMode,
        InputSpec, NodeInspectionMode, QueryType, ResultConversionFormat, ResultConversionMode, ResultDCTapFormat,
        ResultDataFormat, ResultQueryFormat, ResultServiceFormat, ResultShExValidationFormat,
        ResultShaclValidationFormat, ShExFormat, ShExValidationSortByMode, ShaclFormat, ShaclValidationMode,
        ShaclValidationSortByMode, ShapeMapFormat,
    },
};
use std::{io::BufWriter, path::Path, str::FromStr};

/// Main interface for working with Semantic Web operations.
///
/// Provides a unified interface for:
///
/// - Loading and serializing **RDF** and **PG** data.
/// - Loading, checking, serializing and validating **ShEx** schemas.
/// - Loading, serializing and validating **SHACL** shapes.
/// - Loading, serializing and validating **PGSchemas**.
/// - Loading, running and serializing **SPARQL** queries and query results.
/// - Converting and comparing schemas between supported formats.
/// - Loading and serializing **DCTAP** and **Service Descriptions**.
/// - Generating synthetic data from schemas.
#[pyclass(name = "Rudof")]
pub struct PyRudof {
    inner: Rudof,
}

#[pymethods]
impl PyRudof {
    /// Creates a new Rudof instance with the specified configuration.
    ///
    /// Args:
    ///     config (RudofConfig): Configuration object with settings for the Rudof instance.
    ///
    /// Returns:
    ///     Rudof: A new configured Rudof instance ready for use.
    ///
    /// Raises:
    ///     RudofError: If initialization fails due to invalid configuration.
    #[new]
    pub fn __init__(config: &PyRudofConfig) -> PyResult<Self> {
        let rudof = Rudof::new(config.inner.clone());
        Ok(Self { inner: rudof })
    }

    /// Updates the configuration of this Rudof instance.
    ///
    /// Args:
    ///     config (RudofConfig): New configuration to apply.
    ///
    /// Note:
    ///     This does not affect already-loaded data or schemas, only future operations.
    pub fn update_config(&mut self, config: &PyRudofConfig) {
        self.inner.update_config(config.inner.clone()).execute();
    }

    /// Clears the current RDF data graph.
    ///
    /// Removes all RDF triples from memory. Does not affect loaded schemas or other state.
    pub fn reset_data(&mut self) {
        self.inner.reset_data().execute();
    }

    /// Clears the current ShEx schema.
    ///
    /// Unloads the ShEx schema from memory. Does not affect RDF data or other state.
    pub fn reset_shex(&mut self) {
        self.inner.reset_shex().execute();
    }

    /// Clears the current SHACL shapes graph.
    ///
    /// Unloads the SHACL schema from memory. Does not affect RDF data or other state.
    pub fn reset_shacl(&mut self) {
        self.inner.reset_shacl_shapes().execute();
    }

    /// Clears the current ShapeMap.
    ///
    /// Removes the ShapeMap used for ShEx validation.
    pub fn reset_shapemap(&mut self) {
        self.inner.reset_shapemap().execute();
    }

    /// Clears the current SPARQL query.
    ///
    /// Removes the stored query from memory.
    pub fn reset_query(&mut self) {
        self.inner.reset_query().execute();
    }

    /// Resets all current state (data, schemas, queries, validation results).
    ///
    /// This is equivalent to calling all individual reset methods. Use this to
    /// completely clean the Rudof instance.
    pub fn reset_all(&mut self) {
        self.inner.reset_all().execute();
    }

    /// Retrieves detailed information about a specific node in the RDF graph.
    ///
    /// Provides a neighborhood view of a node, including its properties, outgoing
    /// and incoming edges, and connected nodes up to a specified depth.
    ///
    /// Args:
    ///     node_selector (str): Node identifier. Can be:
    ///         - Full IRI: ``<http://example.org/alice>``
    ///         - Prefixed name: ``:alice``
    ///         - Blank node: ``_:b1``
    ///     predicates (List[str], optional): Filter by specific predicates. Empty list
    ///         means all predicates. Defaults to ``[]``.
    ///     mode (str, optional): Node inspection mode. Can be:
    ///         - ``"outgoing"``: Show only outgoing edges
    ///         - ``"incoming"``: Show only incoming edges
    ///         - ``"both"``: Show both outgoing and incoming edges
    ///         Defaults to ``"both"``.
    ///     show_colors (bool, optional): Use ANSI terminal colors in output.
    ///         Defaults to ``True``.
    ///     depth (int, optional): Neighborhood distance (1=direct neighbors, 2=neighbors
    ///         of neighbors, etc.). Defaults to ``1``.
    ///
    /// Returns:
    ///     str: Formatted string with node information and neighborhood graph.
    ///
    /// Raises:
    ///     RudofError: If node selector is invalid or node doesn't exist in the graph.
    ///
    /// Note:
    ///     Colors require a terminal with ANSI escape sequence support.
    #[pyo3(signature = (node_selector, predicates=None, mode=None, show_colors=None, depth=None))]
    pub fn node_info(
        &mut self,
        node_selector: &str,
        predicates: Option<Vec<String>>,
        mode: Option<&str>,
        show_colors: Option<bool>,
        depth: Option<usize>,
    ) -> PyResult<String> {
        let mut writer = BufWriter::new(Vec::new());

        let mut show_node_info = self.inner.show_node_info(node_selector, &mut writer);
        if let Some(predicates) = predicates.as_deref() {
            show_node_info = show_node_info.with_predicates(predicates);
        }
        let aux_mode;
        if let Some(mode) = mode {
            aux_mode = NodeInspectionMode::from_str(mode).map_err(|e| cnv_err(e.into()))?;
            show_node_info = show_node_info.with_show_node_mode(&aux_mode);
        }
        if let Some(show_colors) = show_colors {
            show_node_info = show_node_info.with_show_colors(show_colors);
        }
        if let Some(depth) = depth {
            show_node_info = show_node_info.with_depth(depth);
        }
        show_node_info.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Loads RDF data from a string, file path or URL. If a SPARQL endpoint is specified, it loads data from the endpoint instead.
    ///
    /// Args:
    ///     input (str): String, file path or URL to the RDF data. Defaults to ``None``.
    ///         Examples: ``"data.ttl"``, ``"http://example.org/data.rdf"``
    ///     format (RDFFormat, optional): Serialization format. Defaults to ``RDFFormat.Turtle``.
    ///         Available: Turtle, NTriples, RdfXml, TriG, N3, NQuads, JsonLd
    ///     base (str, optional): Base IRI for resolving relative IRIs. Defaults to ``None``.
    ///     reader_mode (&ReaderMode, optional): Error handling strategy. Defaults to ``ReaderMode.Lax``.
    ///         - ``Lax``: Continue on errors (recommended for real-world data)
    ///         - ``Strict``: Fail on first error
    ///     merge (bool, optional): If ``True``, merge with existing data; if ``False``,
    ///         replace current data. Defaults to ``False``.
    ///    endpoint (str, optional): SPARQL endpoint URL to load data from. If provided, it overrides the `input` parameter. Defaults to ``None``.
    ///
    /// Raises:
    ///     RudofError: If String/file/URL cannot be read or data is malformed (in Strict mode).
    #[pyo3(signature = (input = None, format=None, base=None, reader_mode=None, merge=None, endpoint=None))]
    pub fn read_data(
        &mut self,
        input: Option<&str>,
        format: Option<&PyRDFFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
        merge: Option<bool>,
        endpoint: Option<&str>,
    ) -> PyResult<()> {
        let reader_mode = cnv_reader_mode(reader_mode);
        let format = cnv_rdf_format(format);

        let mut parsed_input = None;
        if let Some(input) = input {
            parsed_input = Some(vec![
                InputSpec::parse_from_str(input, true)
                    .map_err(|e| InputSpecError::InvalidInput {
                        error: { e.to_string() },
                    })
                    .map_err(|e| cnv_err(e.into()))?,
            ]);
        }

        let mut load_data = self.inner.load_data();
        if let Some(input) = &parsed_input {
            load_data = load_data.with_data(input);
        }
        if let Some(format) = format {
            load_data = load_data.with_data_format(format);
        }
        if let Some(base) = base {
            load_data = load_data.with_base(base);
        }
        if let Some(reader_mode) = reader_mode {
            load_data = load_data.with_reader_mode(reader_mode);
        }
        if let Some(merge) = merge {
            load_data = load_data.with_merge(merge);
        }
        if let Some(endpoint) = endpoint {
            load_data = load_data.with_endpoint(endpoint);
        }
        load_data.execute().map_err(cnv_err)?;

        Ok(())
    }

    /// Serializes the current RDF data to a string.
    ///
    /// Args:
    ///     format (ResultDataFormat, optional): Output format. Defaults to ``ResultDataFormat.Compact``.
    ///
    /// Returns:
    ///     str: Serialized RDF data.
    ///
    /// Raises:
    ///     RudofError: If serialization fails.
    #[pyo3(signature = (format=None))]
    pub fn serialize_data(&mut self, format: Option<&PyResultDataFormat>) -> PyResult<String> {
        let mut writer = BufWriter::new(Vec::new());
        let format = cnv_result_data_format(format);

        let mut serialize_data = self.inner.serialize_data(&mut writer);
        if let Some(format) = format {
            serialize_data = serialize_data.with_result_data_format(format);
        }
        serialize_data.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Loads a ShEx schema from a file path or URL.
    ///
    /// Args:
    ///     input (str): String, File path or URL to the ShEx schema.
    ///     format (ShExFormat, optional): Schema format. Defaults to ``ShExFormat.ShExC``.
    ///     base (str, optional): Base IRI for resolving relative IRIs. Defaults to ``None``.
    ///     reader_mode (ReaderMode, optional): Error handling mode. Defaults to ``ReaderMode.Lax``.
    ///
    /// Raises:
    ///     RudofError: If file/URL cannot be read or schema is malformed.
    #[pyo3(signature = (input, format=None, base=None, reader_mode=None))]
    pub fn read_shex(
        &mut self,
        input: &str,
        format: Option<&PyShExFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
    ) -> PyResult<()> {
        let format = cnv_shex_format(format);
        let reader_mode = cnv_reader_mode(reader_mode);

        let input = InputSpec::parse_from_str(input, true)
            .map_err(|e| InputSpecError::InvalidInput {
                error: { e.to_string() },
            })
            .map_err(|e| cnv_err(e.into()))?;

        let mut load_shex = self.inner.load_shex_schema(&input);
        if let Some(format) = format {
            load_shex = load_shex.with_shex_schema_format(format);
        }
        if let Some(base) = base {
            load_shex = load_shex.with_base(base);
        }
        if let Some(reader_mode) = reader_mode {
            load_shex = load_shex.with_reader_mode(reader_mode);
        }
        load_shex.execute().map_err(cnv_err)?;

        Ok(())
    }

    /// Serializes the current ShEx schema to a string.
    ///
    /// Args:
    ///     format (ResultShExValidationFormat, optional): Output format. Defaults to ``ResultShExValidationFormat.Details``.
    ///
    /// Returns:
    ///     str: Serialized ShEx schema.
    ///
    /// Raises:
    ///     RudofError: If no schema is loaded or serialization fails.
    #[pyo3(signature = (shape_label=None, show_dependencies=None, show_statistics=None, show_schema=None, show_time=None, format=None))]
    pub fn serialize_current_shex(
        &self,
        shape_label: Option<&str>,
        show_dependencies: Option<bool>,
        show_statistics: Option<bool>,
        show_schema: Option<bool>,
        show_time: Option<bool>,
        format: Option<&PyShExFormat>,
    ) -> PyResult<String> {
        let mut writer = BufWriter::new(Vec::new());
        let format = cnv_shex_format(format);

        let mut serialize_shex_schema = self.inner.serialize_shex_schema(&mut writer);
        if let Some(format) = format {
            serialize_shex_schema = serialize_shex_schema.with_result_shex_format(format);
        }
        if let Some(shape_label) = shape_label {
            serialize_shex_schema = serialize_shex_schema.with_shape(shape_label);
        }
        if let Some(show_dependencies) = show_dependencies {
            serialize_shex_schema = serialize_shex_schema.with_show_dependencies(show_dependencies);
        }
        if let Some(show_statistics) = show_statistics {
            serialize_shex_schema = serialize_shex_schema.with_show_statistics(show_statistics);
        }
        if let Some(show_schema) = show_schema {
            serialize_shex_schema = serialize_shex_schema.with_show_schema(show_schema);
        }
        if let Some(show_time) = show_time {
            serialize_shex_schema = serialize_shex_schema.with_show_time(show_time);
        }
        serialize_shex_schema.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Serializes the results of the last ShEx validation operation to a string.
    ///
    /// Args:
    ///   format (ResultShExValidationFormat, optional): Output format. Defaults to ``ResultShExValidationFormat.Details``.
    ///   sort_mode (PyShexValidationSortMode, optional): Sorting mode for validation results. Defaults to ``PyShexValidationSortMode.Node``.
    ///
    /// Returns:
    ///  str: Serialized validation results.
    #[pyo3(signature = (format=None, sort_mode=None))]
    fn serialize_shex_validation_results(
        &self,
        format: Option<&PyResultShexValidationFormat>,
        sort_mode: Option<&PyShexValidationSortMode>,
    ) -> PyResult<String> {
        let mut writer = BufWriter::new(Vec::new());

        let mut serialize_shex_validation_results = self.inner.serialize_shex_validation_results(&mut writer);
        if let Some(format) = format {
            let format = cnv_shex_validation_format(format);
            serialize_shex_validation_results =
                serialize_shex_validation_results.with_result_shex_validation_format(format);
        }
        if let Some(sort_mode) = sort_mode {
            let sort_mode = cnv_shex_validation_sort_mode(sort_mode);
            serialize_shex_validation_results =
                serialize_shex_validation_results.with_shex_validation_sort_order_mode(sort_mode);
        }
        serialize_shex_validation_results.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Loads a SHACL shapes graph from a file path or URL.
    ///
    /// Args:
    ///     input (str): String, file path or URL to the SHACL shapes. If not provided it extracts from the currently loaded data.
    ///     format (ShaclFormat, optional): RDF format. Defaults to ``ShaclFormat.Turtle``.
    ///     base (str, optional): Base IRI. Defaults to ``None``.
    ///     reader_mode (ReaderMode, optional): Error handling. Defaults to ``ReaderMode.Lax``.
    ///
    /// Raises:
    ///     RudofError: If file/URL cannot be read or shapes are malformed.
    #[pyo3(signature = (input=None, format=None, base=None, reader_mode=None))]
    pub fn read_shacl(
        &mut self,
        input: Option<&str>,
        format: Option<&PyShaclFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
    ) -> PyResult<()> {
        let format = cnv_shacl_format(format);
        let reader_mode = cnv_reader_mode(reader_mode);

        let mut parsed_input = None;
        if let Some(input) = input {
            parsed_input = Some(
                InputSpec::parse_from_str(input, true)
                    .map_err(|e| InputSpecError::InvalidInput {
                        error: { e.to_string() },
                    })
                    .map_err(|e| cnv_err(e.into()))?,
            );
        }

        let mut load_shacl_schema = self.inner.load_shacl_shapes();
        if let Some(parsed_input) = &parsed_input {
            load_shacl_schema = load_shacl_schema.with_shacl_schema(parsed_input);
        }
        if let Some(format) = format {
            load_shacl_schema = load_shacl_schema.with_shacl_schema_format(format);
        }
        if let Some(base) = base {
            load_shacl_schema = load_shacl_schema.with_base(base);
        }
        if let Some(reader_mode) = reader_mode {
            load_shacl_schema = load_shacl_schema.with_reader_mode(reader_mode);
        }
        load_shacl_schema.execute().map_err(cnv_err)?;

        Ok(())
    }

    /// Serializes the current SHACL shapes graph to a string.
    ///
    /// Args:
    ///     format (ShaclFormat, optional): Output format. Defaults to ``ShaclFormat.Turtle``.
    ///
    /// Returns:
    ///     str: Serialized SHACL shapes.
    ///
    /// Raises:
    ///     RudofError: If no shapes are loaded or serialization fails.
    #[pyo3(signature = (format=None))]
    pub fn serialize_shacl(&self, format: Option<&PyShaclFormat>) -> PyResult<String> {
        let mut writer = BufWriter::new(Vec::new());
        let format = cnv_shacl_format(format);

        let mut serialize_shacl_schema = self.inner.serialize_shacl_shapes(&mut writer);
        if let Some(format) = format {
            serialize_shacl_schema = serialize_shacl_schema.with_shacl_result_format(format);
        }
        serialize_shacl_schema.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Loads a ShapeMap from a string, file path or URL.
    ///
    /// Args:
    ///     input (str): String, file path or URL to the ShapeMap.
    ///     format (ShapeMapFormat, optional): Format. Defaults to ``ShapeMapFormat.Compact``.
    ///     base_nodes (str, optional): Base IRI for resolving node IRIs. Defaults to ``None``.
    ///     base_shapes (str, optional): Base IRI for resolving shape IRIs. Defaults to ``None``.
    ///
    /// Raises:
    ///     RudofError: If file/URL cannot be read or ShapeMap is malformed.
    #[pyo3(signature = (input, format=None, base_nodes=None, base_shapes=None))]
    pub fn read_shapemap(
        &mut self,
        input: &str,
        format: Option<&PyShapeMapFormat>,
        base_nodes: Option<&str>,
        base_shapes: Option<&str>,
    ) -> PyResult<()> {
        let format = cnv_shapemap_format(format);

        let input = InputSpec::parse_from_str(input, true)
            .map_err(|e| InputSpecError::InvalidInput {
                error: { e.to_string() },
            })
            .map_err(|e| cnv_err(e.into()))?;

        let mut load_shapemap = self.inner.load_shapemap(&input);
        if let Some(format) = format {
            load_shapemap = load_shapemap.with_shapemap_format(format);
        }
        if let Some(base_nodes) = base_nodes {
            load_shapemap = load_shapemap.with_base_nodes(base_nodes);
        }
        if let Some(base_shapes) = base_shapes {
            load_shapemap = load_shapemap.with_base_shapes(base_shapes);
        }
        load_shapemap.execute().map_err(cnv_err)?;

        Ok(())
    }

    /// Serializes the current ShapeMap to a string.
    ///
    /// Args:
    ///     format (ShapeMapFormat, optional): Output format. Defaults to ``ShapeMapFormat.Compact``.
    ///
    /// Returns:
    ///     str: Serialized ShapeMap.
    ///
    /// Raises:
    ///     RudofError: If serialization fails or if the resulting bytes cannot be converted
    ///     into a valid UTF-8 string
    #[pyo3(signature = (format=None))]
    pub fn serialize_shapemap(&self, format: Option<&PyShapeMapFormat>) -> PyResult<String> {
        let mut writer = BufWriter::new(Vec::new());
        let format = cnv_shapemap_format(format);

        let mut serialize_shapemap = self.inner.serialize_shapemap(&mut writer);
        if let Some(format) = format {
            serialize_shapemap = serialize_shapemap.with_result_shapemap_format(format);
        }
        serialize_shapemap.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Validates the current RDF data against the loaded ShEx schema using the current ShapeMap.
    ///
    /// Performs ShEx validation by checking if nodes conform to their associated shapes
    /// as defined in the ShapeMap.
    ///
    /// Raises:
    ///     RudofError: If no schema, data, or ShapeMap is loaded.
    pub fn validate_shex(&mut self) -> PyResult<()> {
        self.inner.validate_shex().execute().map_err(cnv_err)?;
        Ok(())
    }

    /// Validates the current RDF data against the loaded SHACL shapes.
    ///
    /// Performs comprehensive SHACL validation checking all constraints defined
    /// in the shapes graph.
    ///
    /// Args:
    ///     mode (ShaclValidationMode, optional): Validation engine. Defaults to ``ShaclValidationMode.Native``.
    ///         - ``Native``: Fast built-in engine (recommended)
    ///         - ``Sparql``: SPARQL-based engine (slower, for debugging)
    ///
    /// Returns:
    ///     ValidationReport: Detailed validation report with conformance status and violations.
    ///
    /// Raises:
    ///     RudofError: If no data or schema is loaded, or validation fails.
    ///
    /// Note:
    ///     - Native mode is recommended for production (faster)
    ///     - SPARQL mode useful for debugging complex constraints
    #[pyo3(signature = (mode=None))]
    pub fn validate_shacl(&mut self, mode: Option<&PyShaclValidationMode>) -> PyResult<()> {
        let mode = cnv_shacl_validation_mode(mode);

        let mut valiate_shacl = self.inner.validate_shacl();
        if let Some(mode) = mode {
            valiate_shacl = valiate_shacl.with_shacl_validation_mode(mode);
        }
        valiate_shacl.execute().map_err(cnv_err)?;

        Ok(())
    }

    /// Serializes the results of the last SHACL validation operation to a string.
    ///
    /// Args:
    ///   format (ResultShaclValidationFormat, optional): Output format. Defaults to ``ResultShaclValidationFormat.Details``.
    ///   sort_mode (ShaclValidationSortMode, optional): Sorting mode for validation results. Defaults to ``ShaclValidationSortMode.Severity``.
    ///
    /// Returns:
    ///  str: Serialized validation results.
    #[pyo3(signature = (format=None, sort_mode=None))]
    pub fn serialize_shacl_validation_results(
        &self,
        format: Option<&PyResultShaclValidationFormat>,
        sort_mode: Option<&PyShaclValidationSortMode>,
    ) -> PyResult<String> {
        let mut writer = BufWriter::new(Vec::new());

        let mut serialize_shacl_validation_results = self.inner.serialize_shacl_validation_results(&mut writer);
        if let Some(format) = format {
            let format = cnv_shacl_validation_format(format);
            serialize_shacl_validation_results =
                serialize_shacl_validation_results.with_result_shacl_validation_format(format);
        }
        if let Some(sort_mode) = sort_mode {
            let sort_mode = cnv_shacl_validation_sort_mode(sort_mode);
            serialize_shacl_validation_results =
                serialize_shacl_validation_results.with_shacl_validation_sort_order_mode(sort_mode);
        }
        serialize_shacl_validation_results.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Loads a ShapeMap from a string, file path or URL.
    ///
    /// Args:
    ///     input (str): String, file path or URL to the ShapeMap.
    ///     format (DCTapFormat, optional): Data format. Defaults to ``DCTapFormat.Csv``.
    ///
    /// Raises:
    ///     RudofError: If DCTAP data is malformed.
    #[pyo3(signature = (input, format=None))]
    pub fn read_dctap(&mut self, input: &str, format: Option<&PyDCTapFormat>) -> PyResult<()> {
        let format = cnv_dctap_format(format);

        let input = InputSpec::parse_from_str(input, true)
            .map_err(|e| InputSpecError::InvalidInput {
                error: { e.to_string() },
            })
            .map_err(|e| cnv_err(e.into()))?;

        let mut load_dctap = self.inner.load_dctap(&input);
        if let Some(format) = format {
            load_dctap = load_dctap.with_dctap_format(format);
        }
        load_dctap.execute().map_err(cnv_err)?;

        Ok(())
    }

    /// Serializes the current DCTAP profile to a string.
    ///
    /// Args:
    ///     format (ResultDCTapFormat, optional): Output format. Defaults to ``ResultDCTapFormat.Internal``.
    ///
    /// Returns:
    ///     str: Serialized DCTAP profile.
    ///
    /// Raises:
    ///     RudofError: If no DCTAP profile is loaded or serialization fails.
    #[pyo3(signature = (format=None))]
    pub fn serialize_dctap(&self, format: Option<&PyResultDCTapFormat>) -> PyResult<String> {
        let mut writer = BufWriter::new(Vec::new());
        let format = cnv_result_dctap_format(format);

        let mut serialize_dctap = self.inner.serialize_dctap(&mut writer);
        if let Some(format) = format {
            serialize_dctap = serialize_dctap.with_result_dctap_format(format);
        }
        serialize_dctap.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Loads a SPARQL query from a string, file path or URL.
    ///
    /// Args:
    ///     input (str): String, file path or URL to SPARQL query.
    ///     query_type (QueryType, optional): Type of SPARQL query. Defaults to ``None`` (auto-detect).
    ///
    /// Raises:
    ///     RudofError: If file/URL cannot be read or query is malformed.
    #[pyo3(signature = (input, query_type=None))]
    pub fn read_query(&mut self, input: &str, query_type: Option<&PyQueryType>) -> PyResult<()> {
        let query_type = cnv_query_type(query_type);

        let input = InputSpec::parse_from_str(input, true)
            .map_err(|e| InputSpecError::InvalidInput {
                error: { e.to_string() },
            })
            .map_err(|e| cnv_err(e.into()))?;

        let mut load_query = self.inner.load_query(&input);
        if let Some(query_type) = query_type {
            load_query = load_query.with_query_type(query_type);
        }
        load_query.execute().map_err(cnv_err)?;

        Ok(())
    }

    /// Executes the loaded query against the loaded data.
    ///
    /// Raises:
    ///     RudofError: If query is malformed or execution fails.
    #[pyo3(signature = ())]
    pub fn run_query(&mut self) -> PyResult<()> {
        self.inner.run_query().execute().map_err(cnv_err)?;

        Ok(())
    }

    /// Serializes the results of the last executed query to a string.
    ///
    /// Args:
    ///    format (QueryResultFormat, optional): Output format. Defaults to ``QueryResultFormat.Compact``.
    ///
    /// Returns:
    ///    str: Serialized query results.
    ///
    /// Raises:
    ///   RudofError: If serialization fails or if the resulting bytes cannot be converted
    #[pyo3(signature = (format=None))]
    pub fn serialize_query_results(&self, format: Option<&PyQueryResultFormat>) -> PyResult<String> {
        let mut writer = BufWriter::new(Vec::new());
        let format = cnv_query_result_format(format);

        let mut serialize_query_results = self.inner.serialize_query_results(&mut writer);
        if let Some(format) = format {
            serialize_query_results = serialize_query_results.with_result_query_format(format);
        }
        serialize_query_results.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Lists known SPARQL endpoints.
    ///
    /// Returns:
    ///     list[tuple[str, str]]: List of (name, url) tuples for known endpoints.
    pub fn list_endpoints(&mut self) -> PyResult<Vec<(String, String)>> {
        let endpoints = self.inner.list_endpoints().execute().map_err(cnv_err)?;
        Ok(endpoints)
    }

    /// Loads a Service Description from a string, file path or URL.
    ///
    /// Args:
    ///     input (str): File path or URL to Service Description (RDF format).
    ///     format (RDFFormat, optional): RDF format. Defaults to ``RDFFormat.Turtle``.
    ///     base (str, optional): Base IRI. Defaults to ``None``.
    ///     reader_mode (ReaderMode, optional): Error handling. Defaults to ``ReaderMode.Lax``.
    ///
    /// Raises:
    ///     RudofError: If file/URL cannot be read or data is malformed.
    #[pyo3(signature = (input, format=None, base=None, reader_mode=None))]
    pub fn read_service_description(
        &mut self,
        input: &str,
        format: Option<&PyRDFFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
    ) -> PyResult<()> {
        let reader_mode = cnv_reader_mode(reader_mode);
        let format = cnv_rdf_format(format);

        let input = InputSpec::parse_from_str(input, true)
            .map_err(|e| InputSpecError::InvalidInput {
                error: { e.to_string() },
            })
            .map_err(|e| cnv_err(e.into()))?;

        let mut load_service_description = self.inner.load_service_description(&input);
        if let Some(format) = format {
            load_service_description = load_service_description.with_data_format(format);
        }
        if let Some(base) = base {
            load_service_description = load_service_description.with_base(base);
        }
        if let Some(reader_mode) = reader_mode {
            load_service_description = load_service_description.with_reader_mode(reader_mode);
        }
        load_service_description.execute().map_err(cnv_err)?;

        Ok(())
    }

    /// Writes the current Service Description to a file.
    ///
    /// Args:
    ///     format (ServiceDescriptionFormat, optional): Format. Defaults to ``ServiceDescriptionFormat.Internal``.
    ///
    /// Raises:
    ///     RudofError: If no description is loaded or file cannot be written.
    #[pyo3(signature = (format=None))]
    pub fn serialize_service_description(&self, format: Option<&PyServiceDescriptionFormat>) -> PyResult<String> {
        let format = cnv_service_description_format(format);
        let mut writer = BufWriter::new(Vec::new());

        let mut serialize_service_description = self.inner.serialize_service_description(&mut writer);
        if let Some(format) = format {
            serialize_service_description = serialize_service_description.with_result_service_format(format);
        }
        serialize_service_description.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Compares two schemas for structural equivalence.
    ///
    /// Converts both schemas to Common Shapes Model and performs structural comparison.
    ///
    /// Args:
    ///     schema1 (str): First schema content.
    ///     schema2 (str): Second schema content.
    ///     mode1 (str, optional): First schema type. Defaults to "shex".
    ///     mode2 (str, optional): Second schema type. Defaults to "shex".
    ///     format1 (str, optional): First schema format. Defaults to "turtle".
    ///     format2 (str, optional): Second schema format. Defaults to "turtle".
    ///     base1 (str, optional): First base IRI. Defaults to ``None``.
    ///     base2 (str, optional): Second base IRI. Defaults to ``None``.
    ///     label1 (str, optional): First shape label. Defaults to ``None``.
    ///     label2 (str, optional): Second shape label. Defaults to ``None``.
    ///     reader_mode (ReaderMode, optional): Error handling. Defaults to ``ReaderMode.Lax``.
    ///
    /// Returns:
    ///     ShaCo: Comparison result showing differences.
    ///
    /// Raises:
    ///     RudofError: If either schema is malformed or comparison fails.
    #[allow(clippy::too_many_arguments)]
    pub fn compare_schemas(
        &mut self,
        schema1: &str,
        schema2: &str,
        mode1: &str,
        mode2: &str,
        format1: &str,
        format2: &str,
        base1: Option<&str>,
        base2: Option<&str>,
        label1: Option<&str>,
        label2: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
    ) -> PyResult<String> {
        let schema1 = InputSpec::parse_from_str(schema1, true)
            .map_err(|e| InputSpecError::InvalidInput {
                error: { e.to_string() },
            })
            .map_err(|e| cnv_err(e.into()))?;

        let schema2 = InputSpec::parse_from_str(schema2, true)
            .map_err(|e| InputSpecError::InvalidInput {
                error: { e.to_string() },
            })
            .map_err(|e| cnv_err(e.into()))?;

        let format1 = ComparisonFormat::from_str(format1).map_err(|e| cnv_err(e.into()))?;
        let format2 = ComparisonFormat::from_str(format2).map_err(|e| cnv_err(e.into()))?;
        let mode1 = ComparisonMode::from_str(mode1).map_err(|e| cnv_err(e.into()))?;
        let mode2 = ComparisonMode::from_str(mode2).map_err(|e| cnv_err(e.into()))?;

        let mut parsed_reader_mode = None;
        if let Some(reader_mode) = reader_mode {
            parsed_reader_mode = cnv_reader_mode(Some(reader_mode));
        }

        let mut writer = BufWriter::new(Vec::new());

        let mut show_schema_comparison =
            self.inner
                .show_schema_comparison(&schema1, &schema2, &format1, &format2, &mode1, &mode2, &mut writer);
        if let Some(reader_mode) = parsed_reader_mode {
            show_schema_comparison = show_schema_comparison.with_reader_mode(reader_mode);
        }
        if let Some(base1) = base1 {
            show_schema_comparison = show_schema_comparison.with_base1(base1);
        }
        if let Some(base2) = base2 {
            show_schema_comparison = show_schema_comparison.with_base2(base2);
        }
        if let Some(label1) = label1 {
            show_schema_comparison = show_schema_comparison.with_shape1(label1);
        }
        if let Some(label2) = label2 {
            show_schema_comparison = show_schema_comparison.with_shape2(label2);
        }
        show_schema_comparison.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (schema, input_mode, output_mode, input_format, output_format, base=None, reader_mode=None, shape=None, templates_folder=None, output_folder=None))]
    pub fn convert_schemas(
        &mut self,
        schema: &str,
        input_mode: &PyConversionMode,
        output_mode: &PyResultConversionMode,
        input_format: &PyConversionFormat,
        output_format: &PyResultConversionFormat,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
        shape: Option<&str>,
        templates_folder: Option<&str>,
        output_folder: Option<&str>,
    ) -> PyResult<String> {
        let schema = InputSpec::parse_from_str(schema, true)
            .map_err(|e| InputSpecError::InvalidInput {
                error: { e.to_string() },
            })
            .map_err(|e| cnv_err(e.into()))?;

        let input_mode = cnv_conversion_mode(input_mode);
        let output_mode = cnv_result_conversion_mode(output_mode);
        let input_format = cnv_conversion_format(input_format);
        let output_format = cnv_result_conversion_format(output_format);

        let mut parsed_reader_mode = None;
        if let Some(reader_mode) = reader_mode {
            parsed_reader_mode = cnv_reader_mode(Some(reader_mode));
        }

        let mut writer = BufWriter::new(Vec::new());

        let mut show_schema_conversion = self.inner.show_schema_conversion(
            &schema,
            input_mode,
            output_mode,
            input_format,
            output_format,
            &mut writer,
        );
        if let Some(base) = base {
            show_schema_conversion = show_schema_conversion.with_base(base);
        }
        if let Some(reader_mode) = parsed_reader_mode {
            show_schema_conversion = show_schema_conversion.with_reader_mode(reader_mode);
        }
        if let Some(shape) = shape {
            show_schema_conversion = show_schema_conversion.with_shape(shape);
        }
        if let Some(templates_folder) = templates_folder {
            show_schema_conversion = show_schema_conversion.with_templates_folder(Path::new(templates_folder));
        }
        if let Some(output_folder) = output_folder {
            show_schema_conversion = show_schema_conversion.with_output_folder(Path::new(output_folder));
        }
        show_schema_conversion.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    /// Alias for :meth:`version`. Returns the current Rudof version.
    ///
    /// Returns:
    ///     str: Version string in semver format (e.g., "0.1.0").
    pub fn get_version(&self) -> PyResult<String> {
        Ok(self.inner.version().execute().to_string())
    }

    /// Clears the current ShEx validation results
    pub fn reset_validation_results(&mut self) {
        self.inner.reset_shex().execute();
    }

    /// Loads a MapState from a JSON file.
    ///
    /// The MapState records the bindings produced by ShEx validation with Map
    /// semantic actions. It is required before calling :meth:`materialize`.
    ///
    /// Args:
    ///     path (str): Path to the JSON file containing the serialized MapState.
    ///
    /// Raises:
    ///     RudofError: If the file cannot be read or the JSON is malformed.
    pub fn read_map_state(&mut self, path: &str) -> PyResult<()> {
        let path = Path::new(path);
        self.inner.load_map_state(path).execute().map_err(cnv_err)?;
        Ok(())
    }

    /// Materializes an RDF graph from the current ShEx schema and MapState.
    ///
    /// Uses the Map semantic-action state (loaded via :meth:`read_map_state` or
    /// set after ShEx validation) to populate the triples defined by the ShEx
    /// schema's Map extensions.
    ///
    /// Args:
    ///     format (ResultDataFormat, optional): RDF serialization format for the
    ///         output graph. Defaults to ``ResultDataFormat.Turtle``.
    ///     node (str, optional): IRI string used as the root subject node of the
    ///         materialized graph. A fresh blank node is minted when omitted.
    ///
    /// Returns:
    ///     str: Serialized RDF graph.
    ///
    /// Raises:
    ///     RudofError: If no ShEx schema or MapState is loaded, if the node IRI
    ///         is invalid, or if materialization or serialization fails.
    #[pyo3(signature = (format=None, node=None))]
    pub fn materialize(&self, format: Option<&PyResultDataFormat>, node: Option<&str>) -> PyResult<String> {
        let mut writer = BufWriter::new(Vec::new());
        let format = cnv_result_data_format(format);

        let mut materialize = self.inner.materialize(&mut writer);
        if let Some(format) = format {
            materialize = materialize.with_result_format(format);
        }
        if let Some(node) = node {
            materialize = materialize.with_initial_node_iri(node);
        }
        materialize.execute().map_err(cnv_err)?;

        let bytes = writer
            .into_inner()
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;
        let output = String::from_utf8(bytes)
            .map_err(|e| RudofError::Generic { error: e.to_string() })
            .map_err(cnv_err)?;

        Ok(output)
    }

    pub fn __repr__(&self) -> String {
        format!("Rudof(version='{}')", self.inner.version().execute())
    }
}

/// Declares the reader mode used when parsing RDF data.
///
/// The reader mode controls how strictly parsers react to syntax errors
/// and other issues in the input stream (files, URLs, strings).
#[pyclass(eq, eq_int, name = "ReaderMode")]
#[derive(PartialEq)]
pub enum PyReaderMode {
    /// Ignore non‑fatal errors and try to continue processing.
    Lax,

    /// Fail immediately on the first parsing error.
    Strict,
}

#[pymethods]
impl PyReaderMode {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.detach(|| PyReaderMode::Lax)
    }
}

impl From<&PyReaderMode> for DataReaderMode {
    fn from(mode: &PyReaderMode) -> Self {
        match mode {
            PyReaderMode::Lax => DataReaderMode::Lax,
            PyReaderMode::Strict => DataReaderMode::Strict,
        }
    }
}

/// Sort mode for displaying a ShEx validation ResultShapeMap as a table.
#[pyclass(eq, eq_int, name = "ShexValidationSortMode")]
#[derive(PartialEq, Clone)]
pub enum PyShexValidationSortMode {
    /// Sort rows by focus node.
    Node,
    /// Sort rows by shape label.
    Shape,
    /// Sort rows by validation status.
    Status,
    /// Sort rows by detailed information.
    Details,
}

#[pymethods]
impl PyShexValidationSortMode {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.detach(|| PyShexValidationSortMode::Node)
    }
}

/// RDF data serialization formats supported when reading or writing graphs.
#[pyclass(eq, eq_int, name = "RDFFormat")]
#[derive(PartialEq)]
pub enum PyRDFFormat {
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    JsonLd,
}

#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ResultDataFormat")]
#[derive(PartialEq)]
pub enum PyResultDataFormat {
    Turtle,
    NTriples,
    JsonLd,
    RdfXml,
    TriG,
    N3,
    NQuads,
    Compact,
    Json,
    PlantUML,
    Svg,
    Png,
}

/// Output formats for SPARQL CONSTRUCT query results.
#[pyclass(eq, eq_int, name = "QueryResultFormat")]
#[derive(PartialEq)]
pub enum PyQueryResultFormat {
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    Csv,
}

/// DCTAP input formats.
#[pyclass(eq, eq_int, name = "DCTapFormat")]
#[derive(PartialEq)]
pub enum PyDCTapFormat {
    Csv,
    Xlsx,
}

/// DCTAP output formats.
#[pyclass(eq, eq_int, name = "ResultDCTapFormat")]
#[derive(PartialEq)]
pub enum PyResultDCTapFormat {
    Internal,
    Json,
}

/// Conversion input modes.
#[pyclass(eq, eq_int, name = "ConversionMode")]
#[derive(PartialEq)]
pub enum PyConversionMode {
    Shacl,
    ShEx,
    Dctap,
}

/// Conversion output modes.
#[pyclass(eq, eq_int, name = "ResultConversionMode")]
#[derive(PartialEq)]
pub enum PyResultConversionMode {
    Sparql,
    ShEx,
    Uml,
    Html,
    Shacl,
}

/// Conversion input formats.
#[pyclass(eq, eq_int, name = "ConversionFormat")]
#[derive(PartialEq)]
pub enum PyConversionFormat {
    Csv,
    ShExC,
    ShExJ,
    Turtle,
    Xlsx,
}

/// Conversion output formats.
#[pyclass(eq, eq_int, name = "ResultConversionFormat")]
#[derive(PartialEq)]
pub enum PyResultConversionFormat {
    Default,
    Internal,
    Json,
    ShExC,
    ShExJ,
    Turtle,
    PlantUML,
    Html,
    Svg,
    Png,
}

/// Service Description serialization format.
#[pyclass(eq, eq_int, name = "ServiceDescriptionFormat")]
#[derive(PartialEq)]
pub enum PyServiceDescriptionFormat {
    Internal,
    Json,
    Mie,
}

/// ShapeMap serialization formats.
#[pyclass(eq, eq_int, name = "ShapeMapFormat")]
#[derive(PartialEq)]
pub enum PyShapeMapFormat {
    Compact,
    Json,
}

/// ShEx schema serialization formats.
#[pyclass(eq, eq_int, name = "ShExFormat")]
#[derive(PartialEq)]
pub enum PyShExFormat {
    ShExC,
    ShExJ,
    Turtle,
}

/// SHACL shapes graph serialization formats.
#[pyclass(eq, eq_int, name = "ShaclFormat")]
#[derive(PartialEq)]
pub enum PyShaclFormat {
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
}

/// SHACL validation engine.
#[pyclass(eq, eq_int, name = "ShaclValidationMode")]
#[derive(PartialEq)]
pub enum PyShaclValidationMode {
    Native,
    Sparql,
}

#[pyclass(eq, eq_int, name = "ShaclValidationSortMode")]
#[derive(PartialEq, Clone)]
pub enum PyShaclValidationSortMode {
    Severity,
    Node,
    Component,
    Value,
    Path,
    SourceShape,
    Details,
}

#[pymethods]
impl PyShaclValidationSortMode {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.detach(|| PyShaclValidationSortMode::Severity)
    }
}

#[pyclass(eq, eq_int, name = "ResultShaclValidationFormat")]
#[derive(PartialEq)]
pub enum PyResultShaclValidationFormat {
    Details,
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    Minimal,
    Compact,
    Json,
    Csv,
}

#[pyclass(eq, eq_int, name = "ResultShexValidationFormat")]
#[derive(PartialEq)]
pub enum PyResultShexValidationFormat {
    Details,
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    Compact,
    Json,
    Csv,
}

#[pyclass(eq, eq_int, name = "QueryType")]
#[derive(PartialEq)]
pub enum PyQueryType {
    Select,
    Construct,
    Ask,
    Describe,
}

/// Source of the SHACL shapes graph used during validation.
///
/// Shapes can come from the current SHACL schema or be extracted
/// from the current RDF data graph.
#[pyclass(eq, eq_int, name = "ShapesGraphSource")]
#[derive(PartialEq)]
pub enum PyShapesGraphSource {
    /// Shapes come from the current RDF data graph.
    CurrentData,
    /// Shapes come from the current SHACL schema.
    CurrentSchema,
}

/// Wrapper for Rudof errors exposed to Python code.
#[pyclass(name = "RudofError")]
pub struct PyRudofError {
    /// Internal Rust error object.
    error: Box<RudofError>,
}

impl PyRudofError {
    /// Creates a new `PyRudofError` from a string message.
    ///
    /// Args:
    ///     msg (str): Error message describing the problem.
    ///
    /// Returns:
    ///     PyRudofError: A new Python exception object wrapping the message.
    #[allow(dead_code)]
    fn str(msg: String) -> Self {
        Self {
            error: Box::new(RudofError::Generic { error: msg }),
        }
    }
}

#[pymethods]
impl PyRudofError {
    pub fn __repr__(&self) -> String {
        format!("RudofError('{}')", self.error)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.error)
    }
}

impl From<PyRudofError> for PyErr {
    fn from(e: PyRudofError) -> Self {
        PyValueError::new_err(format!("{}", e.error))
    }
}

impl From<RudofError> for PyRudofError {
    fn from(error: RudofError) -> Self {
        Self { error: Box::new(error) }
    }
}

/// Converts a Rust `RudofError` into a Python exception, logging it to stderr.
///
/// Args:
///     e (RudofError): The Rust error to convert.
///
/// Returns:
///     PyErr: Python exception corresponding to the Rust error.
pub(crate) fn cnv_err(e: RudofError) -> PyErr {
    let e: PyRudofError = e.into();
    let e: PyErr = e.into();
    e
}

/// Converts a Python DCTAP format enum into the corresponding Rust `DCTAPFormat`.
///
/// Args:
///     format (PyDCTapFormat): The Python enum representing the DCTAP format.
///
/// Returns:
///     DCTAPFormat: Corresponding Rust DCTAP format.
fn cnv_dctap_format(format: Option<&PyDCTapFormat>) -> Option<&DCTapFormat> {
    format?;

    match format.unwrap() {
        PyDCTapFormat::Csv => Some(&DCTapFormat::Csv),
        PyDCTapFormat::Xlsx => Some(&DCTapFormat::Xlsx),
    }
}

fn cnv_result_dctap_format(format: Option<&PyResultDCTapFormat>) -> Option<&ResultDCTapFormat> {
    format?;

    match format.unwrap() {
        PyResultDCTapFormat::Internal => Some(&ResultDCTapFormat::Internal),
        PyResultDCTapFormat::Json => Some(&ResultDCTapFormat::Json),
    }
}

fn cnv_conversion_mode(mode: &PyConversionMode) -> &ConversionMode {
    match mode {
        PyConversionMode::Shacl => &ConversionMode::Shacl,
        PyConversionMode::ShEx => &ConversionMode::ShEx,
        PyConversionMode::Dctap => &ConversionMode::Dctap,
    }
}

fn cnv_result_conversion_mode(mode: &PyResultConversionMode) -> &ResultConversionMode {
    match mode {
        PyResultConversionMode::Sparql => &ResultConversionMode::Sparql,
        PyResultConversionMode::ShEx => &ResultConversionMode::ShEx,
        PyResultConversionMode::Uml => &ResultConversionMode::Uml,
        PyResultConversionMode::Html => &ResultConversionMode::Html,
        PyResultConversionMode::Shacl => &ResultConversionMode::Shacl,
    }
}

fn cnv_conversion_format(format: &PyConversionFormat) -> &ConversionFormat {
    match format {
        PyConversionFormat::Csv => &ConversionFormat::Csv,
        PyConversionFormat::ShExC => &ConversionFormat::ShExC,
        PyConversionFormat::ShExJ => &ConversionFormat::ShExJ,
        PyConversionFormat::Turtle => &ConversionFormat::Turtle,
        PyConversionFormat::Xlsx => &ConversionFormat::Xlsx,
    }
}

fn cnv_result_conversion_format(format: &PyResultConversionFormat) -> &ResultConversionFormat {
    match format {
        PyResultConversionFormat::Default => &ResultConversionFormat::Default,
        PyResultConversionFormat::Internal => &ResultConversionFormat::Internal,
        PyResultConversionFormat::Json => &ResultConversionFormat::Json,
        PyResultConversionFormat::ShExC => &ResultConversionFormat::ShExC,
        PyResultConversionFormat::ShExJ => &ResultConversionFormat::ShExJ,
        PyResultConversionFormat::Turtle => &ResultConversionFormat::Turtle,
        PyResultConversionFormat::PlantUML => &ResultConversionFormat::PlantUML,
        PyResultConversionFormat::Html => &ResultConversionFormat::Html,
        PyResultConversionFormat::Svg => &ResultConversionFormat::Svg,
        PyResultConversionFormat::Png => &ResultConversionFormat::Png,
    }
}

fn cnv_shex_validation_format(format: &PyResultShexValidationFormat) -> &ResultShExValidationFormat {
    match format {
        PyResultShexValidationFormat::Details => &ResultShExValidationFormat::Details,
        PyResultShexValidationFormat::Turtle => &ResultShExValidationFormat::Turtle,
        PyResultShexValidationFormat::NTriples => &ResultShExValidationFormat::NTriples,
        PyResultShexValidationFormat::RdfXml => &ResultShExValidationFormat::RdfXml,
        PyResultShexValidationFormat::TriG => &ResultShExValidationFormat::TriG,
        PyResultShexValidationFormat::N3 => &ResultShExValidationFormat::N3,
        PyResultShexValidationFormat::NQuads => &ResultShExValidationFormat::NQuads,
        PyResultShexValidationFormat::Compact => &ResultShExValidationFormat::Compact,
        PyResultShexValidationFormat::Json => &ResultShExValidationFormat::Json,
        PyResultShexValidationFormat::Csv => &ResultShExValidationFormat::Csv,
    }
}

fn cnv_shex_validation_sort_mode(format: &PyShexValidationSortMode) -> &ShExValidationSortByMode {
    match format {
        PyShexValidationSortMode::Node => &ShExValidationSortByMode::Node,
        PyShexValidationSortMode::Shape => &ShExValidationSortByMode::Shape,
        PyShexValidationSortMode::Status => &ShExValidationSortByMode::Status,
        PyShexValidationSortMode::Details => &ShExValidationSortByMode::Details,
    }
}

fn cnv_shacl_validation_format(format: &PyResultShaclValidationFormat) -> &ResultShaclValidationFormat {
    match format {
        PyResultShaclValidationFormat::Details => &ResultShaclValidationFormat::Details,
        PyResultShaclValidationFormat::Turtle => &ResultShaclValidationFormat::Turtle,
        PyResultShaclValidationFormat::NTriples => &ResultShaclValidationFormat::NTriples,
        PyResultShaclValidationFormat::RdfXml => &ResultShaclValidationFormat::RdfXml,
        PyResultShaclValidationFormat::TriG => &ResultShaclValidationFormat::TriG,
        PyResultShaclValidationFormat::N3 => &ResultShaclValidationFormat::N3,
        PyResultShaclValidationFormat::NQuads => &ResultShaclValidationFormat::NQuads,
        PyResultShaclValidationFormat::Minimal => &ResultShaclValidationFormat::Minimal,
        PyResultShaclValidationFormat::Compact => &ResultShaclValidationFormat::Compact,
        PyResultShaclValidationFormat::Json => &ResultShaclValidationFormat::Json,
        PyResultShaclValidationFormat::Csv => &ResultShaclValidationFormat::Csv,
    }
}

fn cnv_shacl_validation_sort_mode(format: &PyShaclValidationSortMode) -> &ShaclValidationSortByMode {
    match format {
        PyShaclValidationSortMode::Severity => &ShaclValidationSortByMode::Severity,
        PyShaclValidationSortMode::Node => &ShaclValidationSortByMode::Node,
        PyShaclValidationSortMode::Component => &ShaclValidationSortByMode::Component,
        PyShaclValidationSortMode::Value => &ShaclValidationSortByMode::Value,
        PyShaclValidationSortMode::Path => &ShaclValidationSortByMode::Path,
        PyShaclValidationSortMode::SourceShape => &ShaclValidationSortByMode::SourceShape,
        PyShaclValidationSortMode::Details => &ShaclValidationSortByMode::Details,
    }
}

/// Converts a Python reader mode enum into the corresponding Rust `ReaderMode`.
///
/// Args:
///     format (PyReaderMode): Python enum indicating the reader mode.
///
/// Returns:
///     ReaderMode: Corresponding Rust reader mode.
fn cnv_reader_mode(format: Option<&PyReaderMode>) -> Option<&DataReaderMode> {
    format?;

    match format.unwrap() {
        PyReaderMode::Lax => Some(&DataReaderMode::Lax),
        PyReaderMode::Strict => Some(&DataReaderMode::Strict),
    }
}

/// Converts a Python service description format enum into the corresponding Rust `ServiceDescriptionFormat`.
///
/// Args:
///     format (PyServiceDescriptionFormat): Python enum representing service description format.
///
/// Returns:
///     ServiceDescriptionFormat: Corresponding Rust enum.
fn cnv_service_description_format(format: Option<&PyServiceDescriptionFormat>) -> Option<&ResultServiceFormat> {
    format?;

    match format.unwrap() {
        PyServiceDescriptionFormat::Internal => Some(&ResultServiceFormat::Internal),
        PyServiceDescriptionFormat::Mie => Some(&ResultServiceFormat::Mie),
        PyServiceDescriptionFormat::Json => Some(&ResultServiceFormat::Json),
    }
}

/// Converts a Python RDF format enum into the corresponding Rust `RDFFormat`.
///
/// Args:
///     format (PyRDFFormat): Python enum for RDF serialization format.
///
/// Returns:
///     RDFFormat: Corresponding Rust enum.
fn cnv_rdf_format(format: Option<&PyRDFFormat>) -> Option<&DataFormat> {
    format?;

    match format.unwrap() {
        PyRDFFormat::Turtle => Some(&DataFormat::Turtle),
        PyRDFFormat::NTriples => Some(&DataFormat::NTriples),
        PyRDFFormat::RdfXml => Some(&DataFormat::RdfXml),
        PyRDFFormat::TriG => Some(&DataFormat::TriG),
        PyRDFFormat::N3 => Some(&DataFormat::N3),
        PyRDFFormat::NQuads => Some(&DataFormat::NQuads),
        PyRDFFormat::JsonLd => Some(&DataFormat::JsonLd),
    }
}

fn cnv_result_data_format(format: Option<&PyResultDataFormat>) -> Option<&ResultDataFormat> {
    format?;
    match format.unwrap() {
        PyResultDataFormat::Turtle => Some(&ResultDataFormat::Turtle),
        PyResultDataFormat::NTriples => Some(&ResultDataFormat::NTriples),
        PyResultDataFormat::RdfXml => Some(&ResultDataFormat::RdfXml),
        PyResultDataFormat::TriG => Some(&ResultDataFormat::TriG),
        PyResultDataFormat::N3 => Some(&ResultDataFormat::N3),
        PyResultDataFormat::NQuads => Some(&ResultDataFormat::NQuads),
        PyResultDataFormat::Json => Some(&ResultDataFormat::Json),
        PyResultDataFormat::PlantUML => Some(&ResultDataFormat::PlantUML),
        PyResultDataFormat::Png => Some(&ResultDataFormat::Png),
        PyResultDataFormat::Svg => Some(&ResultDataFormat::Svg),
        PyResultDataFormat::Compact => Some(&ResultDataFormat::Compact),
        PyResultDataFormat::JsonLd => Some(&ResultDataFormat::JsonLd),
    }
}

/// Converts a Python ShapeMap format enum into the corresponding Rust `ShapeMapFormat`.
///
/// Args:
///     format (PyShapeMapFormat): Python enum for ShapeMap format.
///
/// Returns:
///     ShapeMapFormat: Corresponding Rust enum.
fn cnv_shapemap_format(format: Option<&PyShapeMapFormat>) -> Option<&ShapeMapFormat> {
    format?;

    match format.unwrap() {
        PyShapeMapFormat::Compact => Some(&ShapeMapFormat::Compact),
        PyShapeMapFormat::Json => Some(&ShapeMapFormat::Json),
    }
}

/// Converts a Python ShEx format enum into the corresponding Rust `ShExFormat`.
///
/// Args:
///     format (PyShExFormat): Python enum representing ShEx format.
///
/// Returns:
///     ShExFormat: Corresponding Rust enum.
fn cnv_shex_format(format: Option<&PyShExFormat>) -> Option<&ShExFormat> {
    format?;

    match format.unwrap() {
        PyShExFormat::ShExC => Some(&ShExFormat::ShExC),
        PyShExFormat::ShExJ => Some(&ShExFormat::ShExJ),
        PyShExFormat::Turtle => Some(&ShExFormat::Turtle),
    }
}

/// Converts a Python SHACL format enum into the corresponding Rust `ShaclFormat`.
///
/// Args:
///     format (PyShaclFormat): Python enum representing SHACL serialization format.
///
/// Returns:
///     ShaclFormat: Corresponding Rust enum.
fn cnv_shacl_format(format: Option<&PyShaclFormat>) -> Option<&ShaclFormat> {
    format?;

    match format.unwrap() {
        PyShaclFormat::Turtle => Some(&ShaclFormat::Turtle),
        PyShaclFormat::NTriples => Some(&ShaclFormat::NTriples),
        PyShaclFormat::RdfXml => Some(&ShaclFormat::RdfXml),
        PyShaclFormat::TriG => Some(&ShaclFormat::TriG),
        PyShaclFormat::N3 => Some(&ShaclFormat::N3),
        PyShaclFormat::NQuads => Some(&ShaclFormat::NQuads),
    }
}

/// Converts a Python SHACL validation mode enum into the corresponding Rust `ShaclValidationMode`.
///
/// Args:
///     mode (PyShaclValidationMode): Python enum indicating the SHACL validation mode.
///
/// Returns:
///     ShaclValidationMode: Corresponding Rust enum.
fn cnv_shacl_validation_mode(mode: Option<&PyShaclValidationMode>) -> Option<&ShaclValidationMode> {
    mode?;

    match mode.unwrap() {
        PyShaclValidationMode::Native => Some(&ShaclValidationMode::Native),
        PyShaclValidationMode::Sparql => Some(&ShaclValidationMode::Sparql),
    }
}

fn cnv_query_type(query_type: Option<&PyQueryType>) -> Option<&QueryType> {
    query_type?;

    match query_type.unwrap() {
        PyQueryType::Select => Some(&QueryType::Select),
        PyQueryType::Construct => Some(&QueryType::Construct),
        PyQueryType::Ask => Some(&QueryType::Ask),
        PyQueryType::Describe => Some(&QueryType::Describe),
    }
}

/// Converts a Python query result format enum into the corresponding Rust `QueryResultFormat`.
///
/// Args:
///     format (PyQueryResultFormat): Python enum for SPARQL query result format.
///
/// Returns:
///     QueryResultFormat: Corresponding Rust enum.
fn cnv_query_result_format(format: Option<&PyQueryResultFormat>) -> Option<&ResultQueryFormat> {
    format?;

    match format.unwrap() {
        PyQueryResultFormat::Turtle => Some(&ResultQueryFormat::Turtle),
        PyQueryResultFormat::NTriples => Some(&ResultQueryFormat::NTriples),
        PyQueryResultFormat::RdfXml => Some(&ResultQueryFormat::RdfXml),
        PyQueryResultFormat::Csv => Some(&ResultQueryFormat::Csv),
        PyQueryResultFormat::TriG => Some(&ResultQueryFormat::TriG),
        PyQueryResultFormat::N3 => Some(&ResultQueryFormat::N3),
        PyQueryResultFormat::NQuads => Some(&ResultQueryFormat::NQuads),
    }
}
