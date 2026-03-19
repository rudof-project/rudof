#![allow(unsafe_op_in_unsafe_fn)]
//! Python bindings for the Rudof RDF validation and manipulation library.
//!
//! This module provides Python wrappers for working with RDF data, ShEx and SHACL schemas,
//! SPARQL queries, and related semantic web technologies.

use crate::PyRudofConfig;
use pyo3::{PyErr, PyResult, Python, exceptions::PyValueError, pyclass, pymethods};
use rudof_lib_refactored::{
    Rudof,
    errors::{InputSpecError, RudofError},
    formats::{
        ComparisonFormat, ComparisonMode, DataFormat, DataReaderMode, InputSpec, NodeInspectionMode, QueryType,
        ResultDataFormat, ResultQueryFormat, ShExValidationSortByMode, DCTapFormat,
        ResultServiceFormat, ShapeMapFormat, ShExFormat, ShaclFormat, ShaclValidationMode
    },
};
use std::{
    io::BufWriter,
    str::FromStr,
};

/// Main interface for working with RDF data, schemas, and validation.
///
/// The ``Rudof`` class provides a unified interface for:
///
/// * Reading and manipulating RDF data in multiple formats
/// * Working with ShEx and SHACL schemas
/// * Validating RDF data against schemas
/// * Executing SPARQL queries (local and remote)
/// * Converting between schema formats (ShEx, SHACL, DCTAP)
/// * Generating visualizations (UML diagrams)
///
/// *State Management*: A single ``Rudof`` instance maintains:
///
/// * RDF data graph
/// * ShEx schema
/// * SHACL shapes graph
/// * ShapeMap for validation
/// * DCTAP application profiles
/// * Current SPARQL query
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
        let rudof = Rudof::new(&config.inner).map_err(PyRudofError::from)?;
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
        self.inner.update_config(&config.inner).execute();
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
        self.inner.reset_all().execute();
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

    // /// Retrieves the current DCTAP (if loaded).
    // ///
    // /// Returns:
    // ///     DCTAP | None: The loaded DCTAP object, or None if no DCTAP is loaded.
    // ///
    // /// See Also:
    // ///     :meth:`read_dctap_str`: Load DCTAP data
    // pub fn get_dctap(&self) -> Option<PyDCTAP> {
    //     let dctap = self.inner.get_dctap();
    //     dctap.map(|s| PyDCTAP { inner: s.clone() })
    // }

    // /// Retrieves the current ShEx schema (if loaded).
    // ///
    // /// Returns:
    // ///     ShExSchema | None: The loaded ShEx schema, or None if no schema is loaded.
    // ///
    // /// See Also:
    // ///     :meth:`read_shex_str`: Load ShEx schema
    // pub fn get_shex(&self) -> Option<PyShExSchema> {
    //     let shex_schema = self.inner.get_shex();
    //     shex_schema.map(|s| PyShExSchema { inner: s.clone() })
    // }

    // /// Retrieves the current SHACL schema (if loaded).
    // ///
    // /// Returns:
    // ///     ShaclSchema | None: The loaded SHACL schema, or None if no schema is loaded.
    // ///
    // /// See Also:
    // ///     :meth:`read_shacl_str`: Load SHACL schema
    // pub fn get_shacl(&self) -> Option<PyShaclSchema> {
    //     let shacl_schema = self.inner.get_shacl_ir();
    //     shacl_schema.map(|s| PyShaclSchema { inner: s.clone() })
    // }

    // /// Retrieves the current ShapeMap (if loaded).
    // ///
    // /// Returns:
    // ///     QueryShapeMap | None: The loaded ShapeMap, or None if no ShapeMap is loaded.
    // ///
    // /// See Also:
    // ///     :meth:`read_shapemap_str`: Load ShapeMap
    // pub fn get_shapemap(&self) -> Option<PyQueryShapeMap> {
    //     let shapemap = self.inner.get_shapemap();
    //     shapemap.map(|s| PyQueryShapeMap { inner: s.clone() })
    // }

    // /// Retrieves the current Service Description (if loaded).
    // ///
    // /// Returns:
    // ///     ServiceDescription | None: The loaded service description, or None if not loaded.
    // ///
    // /// See Also:
    // ///     :meth:`read_service_description`: Load service description
    // pub fn get_service_description(&self) -> Option<PyServiceDescription> {
    //     let service_description = self.inner.get_service_description();
    //     service_description.map(|s| PyServiceDescription { inner: s.clone() })
    // }

    /// Loads RDF data from a string, file path or URL.
    ///
    /// Args:
    ///     input (str): String, file path or URL to the RDF data.
    ///         Examples: ``"data.ttl"``, ``"http://example.org/data.rdf"``
    ///     format (RDFFormat, optional): Serialization format. Defaults to ``RDFFormat.Turtle``.
    ///         Available: Turtle, NTriples, Rdfxml, TriG, N3, NQuads, JsonLd
    ///     base (str, optional): Base IRI for resolving relative IRIs. Defaults to ``None``.
    ///     reader_mode (&ReaderMode, optional): Error handling strategy. Defaults to ``ReaderMode.Lax``.
    ///         - ``Lax``: Continue on errors (recommended for real-world data)
    ///         - ``Strict``: Fail on first error
    ///     merge (bool, optional): If ``True``, merge with existing data; if ``False``,
    ///         replace current data. Defaults to ``False``.
    ///
    /// Raises:
    ///     RudofError: If String/file/URL cannot be read or data is malformed (in Strict mode).
    #[pyo3(signature = (input, format=None, base=None, reader_mode=None, merge=None))]
    pub fn read_data(
        &mut self,
        input: &str,
        format: Option<&PyRDFFormat>,
        base: Option<&str>,
        reader_mode: Option<&PyReaderMode>,
        merge: Option<bool>,
    ) -> PyResult<()> {
        let reader_mode = cnv_reader_mode(reader_mode);
        let format = cnv_rdf_format(format);

        let input = InputSpec::from_str(input)
            .map_err(|e| InputSpecError::InvalidInput {
                error: { e.to_string() },
            })
            .map_err(|e| cnv_err(e.into()))?;
        let inputs = vec![input];

        let mut load_data = self.inner.load_data(&inputs);
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
        load_data.execute().map_err(cnv_err)?;

        Ok(())
    }

    // /// Loads RDF data from a string.
    // ///
    // /// Args:
    // ///     input (str): String containing RDF data in the specified format.
    // ///     format (RDFFormat, optional): Serialization format. Defaults to ``RDFFormat.Turtle``.
    // ///     base (str, optional): Base IRI for resolving relative IRIs. Defaults to ``None``.
    // ///     reader_mode (ReaderMode, optional): Error handling mode. Defaults to ``ReaderMode.Lax``.
    // ///         - ``Lax``: Continue on errors (recommended for real-world data)
    // ///         - ``Strict``: Fail on first error
    // ///     merge (bool, optional): If ``True``, merge with existing data; if ``False``,
    // ///         replace. Defaults to ``False``.
    // ///
    // /// Raises:
    // ///     RudofError: If data is malformed.
    // #[pyo3(signature = (input, format=None, base=None, reader_mode=None, merge=None))]
    // pub fn read_data_str(
    //     &mut self,
    //     input: &str,
    //     format: Option<&PyRDFFormat>,
    //     base: Option<&str>,
    //     reader_mode: Option<&PyReaderMode>,
    //     merge: Option<bool>,
    // ) -> PyResult<()> {
    //     let reader_mode = cnv_reader_mode(reader_mode);
    //     let format = cnv_rdf_format(format);

    //     self.inner
    //         .read_data(&mut input.as_bytes(), "String", format, base, reader_mode, merge)
    //         .map_err(cnv_err)?;
    //     Ok(())
    // }

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
    pub fn serialize_data(&self, format: Option<&PyResultDataFormat>) -> PyResult<String> {
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

    // /// Loads a ShEx schema from a string.
    // ///
    // /// Args:
    // ///     input (str): String containing the ShEx schema.
    // ///     format (ShExFormat, optional): Schema format. Defaults to ``ShExFormat.ShExC``.
    // ///         Available: ShExC (compact syntax), ShExJ (JSON), Turtle
    // ///     base (str, optional): Base IRI for resolving relative IRIs. Defaults to ``None``.
    // ///     reader_mode (ReaderMode, optional): Error handling mode. Defaults to ``ReaderMode.Lax``.
    // ///
    // /// Raises:
    // ///     RudofError: If schema is malformed.
    // #[pyo3(signature = (input, format=None, base=None, reader_mode=None))]
    // pub fn read_shex_str(
    //     &mut self,
    //     input: &str,
    //     format: Option<&PyShExFormat>,
    //     base: Option<&str>,
    //     reader_mode: Option<&PyReaderMode>,
    // ) -> PyResult<()> {
    //     let format = cnv_shex_format(format);
    //     let reader_mode = cnv_reader_mode(reader_mode);

    //     self.inner.reset_shex();
    //     self.inner
    //         .read_shex(input.as_bytes(), format, base, reader_mode, Some("string"))
    //         .map_err(cnv_err)?;
    //     Ok(())
    // }

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

        let input = InputSpec::from_str(input)
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

    // /// Serializes a specific ShEx schema to a string.
    // ///
    // /// Args:
    // ///     shex (ShExSchema): Schema object to serialize.
    // ///     formatter (ShExFormatter): Formatter for controlling output style.
    // ///     format (ShExFormat, optional): Output format. Defaults to ``ShExFormat.ShExC``.
    // ///
    // /// Returns:
    // ///     str: Serialized ShEx schema.
    // #[pyo3(signature = (shex, formatter, format=None))]
    // pub fn serialize_shex(
    //     &self,
    //     shex: &PyShExSchema,
    //     formatter: &PyShExFormatter,
    //     format: Option<&PyShExFormat>,
    // ) -> PyResult<String> {
    //     let mut v = Vec::new();
    //     let format = cnv_shex_format(format);
    //     self.inner
    //         .serialize_shex(&shex.inner, format, &formatter.inner, &mut v)
    //         .map_err(|e| RudofError::SerializingShEx { error: format!("{e}") })
    //         .map_err(cnv_err)?;
    //     let str = String::from_utf8(v)
    //         .map_err(|e| RudofError::SerializingShEx { error: format!("{e}") })
    //         .map_err(cnv_err)?;
    //     Ok(str)
    // }

    // /// Loads a SHACL shapes graph from a string.
    // ///
    // /// Args:
    // ///     input (str): String containing the SHACL shapes in RDF format.
    // ///     format (ShaclFormat, optional): RDF serialization format. Defaults to ``ShaclFormat.Turtle``.
    // ///     base (str, optional): Base IRI for resolving relative IRIs. Defaults to ``None``.
    // ///     reader_mode (ReaderMode, optional): Error handling mode. Defaults to ``ReaderMode.Lax``.
    // ///
    // /// Raises:
    // ///     RudofError: If shapes are malformed.
    // #[pyo3(signature = (input, format=None, base=None, reader_mode=None))]
    // pub fn read_shacl_str(
    //     &mut self,
    //     input: &str,
    //     format: Option<&PyShaclFormat>,
    //     base: Option<&str>,
    //     reader_mode: Option<&PyReaderMode>,
    // ) -> PyResult<()> {
    //     let format = cnv_shacl_format(format);
    //     let reader_mode = cnv_reader_mode(reader_mode);
    //     self.inner.reset_shacl();
    //     self.inner
    //         .read_shacl(&mut input.as_bytes(), input, format, base, reader_mode)
    //         .map_err(cnv_err)?;
    //     Ok(())
    // }

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
                InputSpec::from_str(input)
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

    // /// Loads a ShapeMap from a string.
    // ///
    // /// ShapeMaps associate nodes with shapes for validation. Format examples:
    // /// - Compact: ``:alice@:Person, :bob@:Person``
    // /// - JSON: ``[{"node": ":alice", "shape": ":Person"}]``
    // ///
    // /// Args:
    // ///     str (str): String containing the ShapeMap.
    // ///     format (ShapeMapFormat, optional): ShapeMap format. Defaults to ``ShapeMapFormat.Compact``.
    // ///     base_nodes (str, optional): Base IRI for resolving node IRIs. Defaults to ``None``.
    // ///     base_shapes (str, optional): Base IRI for resolving shape IRIs. Defaults to ``None``.
    // ///
    // /// Raises:
    // ///     RudofError: If ShapeMap is malformed.
    // #[pyo3(signature = (str, format=None, base_nodes=None, base_shapes=None))]
    // pub fn read_shapemap_str(
    //     &mut self,
    //     str: &str,
    //     format: Option<&PyShapeMapFormat>,
    //     base_nodes: Option<&str>,
    //     base_shapes: Option<&str>,
    // ) -> PyResult<()> {
    //     let format = cnv_shapemap_format(format).unwrap_or(&ShapeMapFormat::Compact);

    //     let base_nodes_iri = if let Some(base) = base_nodes {
    //         Some(
    //             IriS::from_str(base)
    //                 .map_err(|e| RudofError::BaseIriError {
    //                     str: base.to_string(),
    //                     error: format!("{e}"),
    //                 })
    //                 .map_err(cnv_err)?,
    //         )
    //     } else {
    //         None
    //     };

    //     let base_shapes_iri = if let Some(base) = base_shapes {
    //         Some(
    //             IriS::from_str(base)
    //                 .map_err(|e| RudofError::BaseIriError {
    //                     str: base.to_string(),
    //                     error: format!("{e}"),
    //                 })
    //                 .map_err(cnv_err)?,
    //         )
    //     } else {
    //         None
    //     };

    //     self.inner
    //         .read_shapemap(str.as_bytes(), "String", format, &base_nodes_iri, &base_shapes_iri)
    //         .map_err(cnv_err)?;
    //     Ok(())
    // }

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

        let input = InputSpec::from_str(input)
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

    /// Loads a ShapeMap from a string, file path or URL.
    ///
    /// Args:
    ///     input (str): String, file path or URL to the ShapeMap.
    ///     format (DCTapFormat, optional): Data format. Defaults to ``DCTapFormat.CSV``.
    ///
    /// Raises:
    ///     RudofError: If DCTAP data is malformed.
    #[pyo3(signature = (input, format=None))]
    pub fn read_dctap(&mut self, input: &str, format: Option<&PyDCTapFormat>) -> PyResult<()> {
        let format = cnv_dctap_format(format);

        let input = InputSpec::from_str(input)
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

    // /// Loads DCTAP from a string.
    // ///
    // /// Args:
    // ///     input (str): String containing DCTAP data (CSV or Excel format).
    // ///     format (DCTapFormat, optional): Data format. Defaults to ``DCTapFormat.CSV``.
    // ///
    // /// Raises:
    // ///     RudofError: If DCTAP data is malformed.
    // #[pyo3(signature = (input, format=None))]
    // pub fn read_dctap_str(&mut self, input: &str, format: Option<&PyDCTapFormat>) -> PyResult<()> {
    //     self.inner.reset_dctap();
    //     let format = cnv_dctap_format(format);
    //     self.inner.read_dctap(input.as_bytes(), format).map_err(cnv_err)?;
    //     Ok(())
    // }

    // /// Loads DCTAP from a file path.
    // ///
    // /// Args:
    // ///     path_name (str): Path to DCTAP file (CSV or Excel).
    // ///     format (DCTapFormat, optional): Format. Defaults to ``DCTapFormat.CSV``.
    // ///
    // /// Raises:
    // ///     RudofError: If file cannot be read or data is malformed.
    // #[pyo3(signature = (path_name, format=None))]
    // pub fn read_dctap_path(&mut self, path_name: &str, format: Option<&PyDCTapFormat>) -> PyResult<()> {
    //     let reader = get_path_reader(path_name, "DCTAP data")?;
    //     self.inner.reset_dctap();
    //     let format = cnv_dctap_format(format);
    //     self.inner.read_dctap(reader, format).map_err(cnv_err)?;
    //     Ok(())
    // }

    // /// Converts the current DCTAP to ShEx schema.
    // ///
    // /// Transforms a DCTAP application profile into an equivalent ShEx schema,
    // /// replacing the current ShEx schema with the conversion result.
    // ///
    // /// Raises:
    // ///     RudofError: If no DCTAP is loaded or conversion fails.
    // pub fn dctap2shex(&mut self) -> PyResult<()> {
    //     self.inner.dctap2shex().map_err(cnv_err)
    // }

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

        let input = InputSpec::from_str(input)
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

    // /// Executes a SPARQL SELECT query from a string.
    // ///
    // /// Args:
    // ///     input (str): SPARQL SELECT query string.
    // ///
    // /// Returns:
    // ///     QuerySolutions: Result set with variable bindings.
    // ///
    // /// Raises:
    // ///     RudofError: If query is malformed or execution fails.
    // pub fn run_query_str(&mut self, input: &str) -> PyResult<PyQuerySolutions> {
    //     let results = self.inner.run_query_select_str(input).map_err(cnv_err)?;
    //     Ok(PyQuerySolutions { inner: results })
    // }

    // /// Loads a SPARQL query from a string for later execution.
    // ///
    // /// Args:
    // ///     input (str): SPARQL query string (SELECT or CONSTRUCT).
    // ///
    // /// Raises:
    // ///     RudofError: If query is malformed.
    // pub fn read_query_str(&mut self, input: &str) -> PyResult<()> {
    //     self.inner.read_query_str(input).map_err(cnv_err)
    // }

    // /// Executes a SPARQL query from a file path.
    // ///
    // /// Args:
    // ///     path_name (str): Path to file containing SPARQL SELECT query.
    // ///
    // /// Returns:
    // ///     QuerySolutions: Query results.
    // ///
    // /// Raises:
    // ///     RudofError: If file cannot be read or query fails.
    // pub fn run_query_path(&mut self, path_name: &str) -> PyResult<PyQuerySolutions> {
    //     let mut reader = get_path_reader(path_name, "SPARQL query")?;
    //     let results = self.inner.run_query_select(&mut reader).map_err(cnv_err)?;
    //     Ok(PyQuerySolutions { inner: results })
    // }

    // /// Executes the previously loaded SELECT query.
    // ///
    // /// Returns:
    // ///     QuerySolutions: Query results.
    // ///
    // /// Raises:
    // ///     RudofError: If no query is loaded or it's not a SELECT query.
    // pub fn run_current_query_select(&mut self) -> PyResult<PyQuerySolutions> {
    //     let results = self.inner.run_current_query_select().map_err(cnv_err)?;
    //     Ok(PyQuerySolutions { inner: results })
    // }

    // /// Executes the previously loaded CONSTRUCT query.
    // ///
    // /// Args:
    // ///     format (QueryResultFormat, optional): Output format. Defaults to ``QueryResultFormat.Turtle``.
    // ///
    // /// Returns:
    // ///     str: Constructed RDF graph.
    // ///
    // /// Raises:
    // ///     RudofError: If no query is loaded or it's not a CONSTRUCT query.
    // #[pyo3(signature = (format=None))]
    // pub fn run_current_query_construct(&mut self, format: Option<&PyQueryResultFormat>) -> PyResult<String> {
    //     let format = cnv_query_result_format(format);
    //     let str = self.inner.run_current_query_construct(format).map_err(cnv_err)?;
    //     Ok(str)
    // }

    // /// Executes a SPARQL query against a remote endpoint.
    // ///
    // /// Args:
    // ///     query (str): SPARQL query string.
    // ///     endpoint (str): SPARQL endpoint URL.
    // ///
    // /// Returns:
    // ///     QuerySolutions: Query results from the endpoint.
    // ///
    // /// Raises:
    // ///     RudofError: If endpoint is unreachable or query fails.
    // pub fn run_query_endpoint_str(&mut self, query: &str, endpoint: &str) -> PyResult<PyQuerySolutions> {
    //     let results = self.inner.run_query_endpoint(query, endpoint).map_err(cnv_err)?;
    //     Ok(PyQuerySolutions { inner: results })
    // }

    /// Configures a SPARQL endpoint for subsequent queries.
    ///
    /// Args:
    ///     endpoint (str): SPARQL endpoint URL.
    ///
    /// Raises:
    ///     RudofError: If endpoint configuration fails.
    // pub fn use_endpoint(&mut self, endpoint: &str) -> PyResult<()> {
    //     self.inner.use_endpoint(endpoint).map_err(cnv_err)
    // }

    /// Stops using a previously configured SPARQL endpoint.
    ///
    /// Args:
    ///     endpoint (str): SPARQL endpoint URL to remove.
    // pub fn dont_use_endpoint(&mut self, endpoint: &str) -> PyResult<()> {
    //     self.inner.dont_use_endpoint(endpoint);
    //     Ok(())
    // }

    /// Lists known SPARQL endpoints.
    ///
    /// Returns:
    ///     list[tuple[str, str]]: List of (name, url) tuples for known endpoints.
    pub fn list_endpoints(&self) -> PyResult<Vec<(String, String)>> {
        let endpoints = self.inner.list_endpoints().execute();
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

        let input = InputSpec::from_str(input)
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

    // /// Loads a Service Description from a string.
    // ///
    // /// Args:
    // ///     input (str): String containing Service Description in RDF format.
    // ///     format (RDFFormat, optional): Format. Defaults to ``RDFFormat.Turtle``.
    // ///     base (str, optional): Base IRI. Defaults to ``None``.
    // ///     reader_mode (ReaderMode, optional): Error handling. Defaults to ``ReaderMode.Lax``.
    // ///
    // /// Raises:
    // ///     RudofError: If data is malformed.
    // #[pyo3(signature = (input, format=None, base=None, reader_mode=None))]
    // pub fn read_service_description_str(
    //     &mut self,
    //     input: &str,
    //     format: Option<&PyRDFFormat>,
    //     base: Option<&str>,
    //     reader_mode: Option<&PyReaderMode>,
    // ) -> PyResult<()> {
    //     let reader_mode = cnv_reader_mode(reader_mode);
    //     let format = cnv_rdf_format(format);

    //     self.inner
    //         .read_service_description(&mut input.as_bytes(), "String", format, base, reader_mode)
    //         .map_err(cnv_err)?;
    //     Ok(())
    // }

    /// Writes the current Service Description to a file.
    ///
    /// Args:
    ///     format (ServiceDescriptionFormat, optional): Format. Defaults to ``ServiceDescriptionFormat.Internal``.
    ///
    /// Raises:
    ///     RudofError: If no description is loaded or file cannot be written.
    #[pyo3(signature = (format=None))]
    pub fn serialize_service_description(
        &self,
        format: Option<&PyServiceDescriptionFormat>,
    ) -> PyResult<String> {
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

    /// Converts current RDF data to PlantUML format.
    ///
    /// Generates a visual representation that can be rendered to SVG/PNG using PlantUML.
    ///
    /// Returns:
    ///     str: PlantUML diagram source.
    ///
    /// Raises:
    ///     RudofError: If no data is loaded or generation fails.
    // pub fn data2plantuml(&self) -> PyResult<String> {
    //     let mut writer = Cursor::new(Vec::new());
    //     self.inner
    //         .data2plant_uml(&mut writer)
    //         .map_err(|e| RudofError::RDF2PlantUmlError {
    //             error: format!("Error generating UML for current RDF data: {e}"),
    //         })
    //         .map_err(cnv_err)?;
    //     let str = String::from_utf8(writer.into_inner())
    //         .map_err(|e| RudofError::RDF2PlantUmlError {
    //             error: format!("RDF2PlantUML: Error converting generated vector to UML: {e}"),
    //         })
    //         .map_err(cnv_err)?;
    //     Ok(str)
    // }

    /// Converts current RDF data to PlantUML and writes to file.
    ///
    /// Args:
    ///     file_name (str): Output file path for PlantUML source.
    ///
    /// Raises:
    ///     RudofError: If no data is loaded, generation fails, or file cannot be written.
    // pub fn data2plantuml_file(&self, file_name: &str) -> PyResult<()> {
    //     let file = File::create(file_name)?;
    //     let mut writer = BufWriter::new(file);
    //     self.inner
    //         .data2plant_uml(&mut writer)
    //         .map_err(|e| RudofError::RDF2PlantUmlError {
    //             error: format!("Error generating UML for current RDF data: {e}"),
    //         })
    //         .map_err(cnv_err)?;
    //     Ok(())
    // }

    /// Converts current ShEx schema to PlantUML class diagram.
    ///
    /// Args:
    ///     uml_mode (UmlGenerationMode): Generation mode.
    ///         - ``UmlGenerationMode.all()``: Include all shapes
    ///         - ``UmlGenerationMode.neighs(shape)``: Only neighbors of specified shape
    ///
    /// Returns:
    ///     str: PlantUML class diagram source.
    ///
    /// Raises:
    ///     RudofError: If no schema is loaded or generation fails.
    // pub fn shex2plantuml(&self, uml_mode: &PyUmlGenerationMode) -> PyResult<String> {
    //     let mut v = Vec::new();
    //     self.inner
    //         .shex2plant_uml(&uml_mode.into(), &mut v)
    //         .map_err(|e| RudofError::ShEx2PlantUmlError {
    //             error: format!("Error generating UML: {e}"),
    //         })
    //         .map_err(cnv_err)?;
    //     let str = String::from_utf8(v)
    //         .map_err(|e| RudofError::ShEx2PlantUmlError {
    //             error: format!("ShEx2PlantUML: Error converting generated vector to UML: {e}"),
    //         })
    //         .map_err(cnv_err)?;
    //     Ok(str)
    // }

    /// Converts current ShEx schema to PlantUML and writes to file.
    ///
    /// Args:
    ///     uml_mode (UmlGenerationMode): Generation mode.
    ///         - ``UmlGenerationMode.all()``: Include all shapes
    ///         - ``UmlGenerationMode.neighs(shape)``: Only neighbors of specified shape
    ///     file_name (str): Output file path.
    ///
    /// Raises:
    ///     RudofError: If no schema is loaded, generation fails, or file cannot be written.
    // pub fn shex2plantuml_file(&self, uml_mode: &PyUmlGenerationMode, file_name: &str) -> PyResult<()> {
    //     let file = File::create(file_name)?;
    //     let mut writer = BufWriter::new(file);
    //     self.inner
    //         .shex2plant_uml(&uml_mode.into(), &mut writer)
    //         .map_err(|e| RudofError::ShEx2PlantUmlError {
    //             error: format!("Error generating UML: {e} in {file_name}"),
    //         })
    //         .map_err(cnv_err)?;
    //     Ok(())
    // }

    // /// Converts a schema to Common Shapes Model (CoShaMo) for comparison.
    // ///
    // /// Args:
    // ///     schema (str): Schema content as string.
    // ///     mode (str, optional): Schema type (e.g., "shex"). Defaults to "shex".
    // ///     format (str, optional): Schema format (e.g., "turtle", "shexc"). Defaults to "turtle".
    // ///     base (str, optional): Base IRI. Defaults to ``None``.
    // ///     reader_mode (ReaderMode, optional): Error handling. Defaults to ``ReaderMode.Lax``.
    // ///     label (str, optional): Shape label to convert. Defaults to ``None`` (uses start shape).
    // ///
    // /// Returns:
    // ///     CoShaMo: Common Shapes Model representation.
    // ///
    // /// Raises:
    // ///     RudofError: If schema is malformed or conversion fails.
    // #[pyo3(signature = (schema, mode=None, format=None, base=None, reader_mode=None, label=None))]
    // pub fn get_coshamo_str(
    //     &mut self,
    //     schema: &str,
    //     mode: Option<&str>,
    //     format: Option<&str>,
    //     base: Option<&str>,
    //     reader_mode: Option<&PyReaderMode>,
    //     label: Option<&str>,
    // ) -> PyResult<PyCoShaMo> {
    //     let format = format.unwrap_or("turtle");
    //     let mode = mode.unwrap_or("shex");
    //     let reader_mode = cnv_reader_mode(reader_mode);

    //     let format = InputCompareFormat::from_str(format).map_err(cnv_string_err)?;
    //     let mode = InputCompareMode::from_str(mode).map_err(cnv_string_err)?;
    //     let mut reader = schema.as_bytes();
    //     let coshamo = self
    //         .inner
    //         .get_coshamo(&mut reader, &mode, &format, base, reader_mode, label, Some("string"))
    //         .map_err(PyRudofError::from)?;
    //     Ok(PyCoShaMo { inner: coshamo })
    // }

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
        let schema1 = InputSpec::from_str(schema1)
            .map_err(|e| InputSpecError::InvalidInput {
                error: { e.to_string() },
            })
            .map_err(|e| cnv_err(e.into()))?;

        let schema2 = InputSpec::from_str(schema2)
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

        let mut show_schema_comparison = self.inner.show_schema_comparison(
            &schema1,
            &schema2,
            &format1,
            &format2,
            &mode1,
            &mode2,
            &mut writer,
        );
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
///
/// This controls how rows are ordered when calling
/// :meth:`ResultShapeMap.show_as_table`.
#[pyclass(eq, eq_int, name = "SortModeResultMap")]
#[derive(PartialEq, Clone)]
pub enum PySortModeResultMap {
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
impl PySortModeResultMap {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.detach(|| PySortModeResultMap::Node)
    }
}

impl From<&PySortModeResultMap> for ShExValidationSortByMode {
    fn from(mode: &PySortModeResultMap) -> Self {
        match mode {
            PySortModeResultMap::Node => Self::Node,
            PySortModeResultMap::Shape => Self::Shape,
            PySortModeResultMap::Status => Self::Status,
            PySortModeResultMap::Details => Self::Details,
        }
    }
}

/// RDF data serialization formats supported when reading or writing graphs.
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "RDFFormat")]
#[derive(PartialEq)]
pub enum PyRDFFormat {
    Turtle,
    NTriples,
    Rdfxml,
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
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "QueryResultFormat")]
#[derive(PartialEq)]
pub enum PyQueryResultFormat {
    Turtle,
    NTriples,
    Rdfxml,
    TriG,
    N3,
    NQuads,
    CSV,
}

/// DCTAP input formats.
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "DCTapFormat")]
#[derive(PartialEq)]
pub enum PyDCTapFormat {
    CSV,
    XLSX,
}

/// Service Description serialization format.
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ServiceDescriptionFormat")]
#[derive(PartialEq)]
pub enum PyServiceDescriptionFormat {
    Internal,
    Json,
    Mie,
}

/// ShapeMap serialization formats.
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ShapeMapFormat")]
#[derive(PartialEq)]
pub enum PyShapeMapFormat {
    Compact,
    JSON,
}

/// ShEx schema serialization formats.
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ShExFormat")]
#[derive(PartialEq)]
pub enum PyShExFormat {
    ShExC,
    ShExJ,
    Turtle,
}

/// SHACL shapes graph serialization formats.
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ShaclFormat")]
#[derive(PartialEq)]
pub enum PyShaclFormat {
    Turtle,
    NTriples,
    Rdfxml,
    TriG,
    N3,
    NQuads,
}

// /// Controls how ShEx schemas are pretty‑printed.
// ///
// /// Formatters can optionally disable ANSI colors for use in plain‑text
// /// environments or files viewed in non‑color editors.
// #[pyclass(frozen, name = "ShExFormatter")]
// pub struct PyShExFormatter {
//     inner: ShExFormatter,
// }

// #[pymethods]
// impl PyShExFormatter {
//     #[new]
//     pub fn __init__(py: Python<'_>) -> Self {
//         py.detach(|| Self {
//             inner: ShExFormatter::default(),
//         })
//     }

//     /// Returns a ShExFormatter that doesn't print terminal colors.
//     #[staticmethod]
//     pub fn without_colors() -> Self {
//         Self {
//             inner: ShExFormatter::default().without_colors(),
//         }
//     }
// }

// /// Controls how ShapeMaps are pretty‑printed.
// ///
// /// Formatters can optionally disable ANSI colors so that output is
// /// suitable for logs or non‑color text editors.
// #[pyclass(frozen, name = "ShapeMapFormatter")]
// pub struct PyShapeMapFormatter {
//     inner: ShapeMapFormatter,
// }

// #[pymethods]
// impl PyShapeMapFormatter {
//     #[new]
//     pub fn __init__(py: Python<'_>) -> Self {
//         py.detach(|| Self {
//             inner: ShapeMapFormatter::default(),
//         })
//     }

//     /// Creates a `ShapeMapFormatter` that disables terminal colors.
//     ///
//     /// Returns:
//     ///     ShapeMapFormatter: A new formatter instance configured to not use ANSI colors in its output.
//     #[staticmethod]
//     pub fn without_colors() -> Self {
//         Self {
//             inner: ShapeMapFormatter::default().without_colors(),
//         }
//     }
// }

// /// UML generation mode for PlantUML exports.
// ///
// /// Determines whether diagrams include all shapes or only the
// /// neighbourhood of a specific node/shape.
// #[pyclass(name = "UmlGenerationMode")]
// pub enum PyUmlGenerationMode {
//     /// Generate UML for all shapes in the model.
//     #[pyo3(name = "AllNodes")]
//     PyAllNodes {},

//     /// Generate UML only for the neighbours of a given node/shape.
//     ///
//     /// Args:
//     ///     node (str): The identifier of the shape or node whose neighbors will be included in the UML diagram.
//     #[pyo3(constructor = (node), name ="Neighs")]
//     PyNeighs { node: String },
// }

// #[pymethods]
// impl PyUmlGenerationMode {
//     #[new]
//     pub fn __init__(py: Python<'_>) -> Self {
//         py.detach(|| PyUmlGenerationMode::PyAllNodes {})
//     }

//     /// Returns a UML generation mode that includes all nodes in the model.
//     ///
//     /// Returns:
//     ///     PyUmlGenerationMode: A mode that generates UML diagrams for every shape in the model.
//     #[staticmethod]
//     pub fn all() -> Self {
//         PyUmlGenerationMode::PyAllNodes {}
//     }

//     /// Returns a UML generation mode that includes only the direct neighbours of a specified node.
//     ///
//     /// Args:
//     ///     node (str): The identifier of the node whose neighbours should be included in the UML diagram.
//     ///
//     /// Returns:
//     ///     PyUmlGenerationMode: A mode that generates UML only for the specified node and its immediate neighbors.
//     #[staticmethod]
//     pub fn neighs(node: &str) -> Self {
//         PyUmlGenerationMode::PyNeighs { node: node.to_string() }
//     }
// }

// impl From<&PyUmlGenerationMode> for UmlGenerationMode {
//     fn from(m: &PyUmlGenerationMode) -> UmlGenerationMode {
//         match m {
//             PyUmlGenerationMode::PyAllNodes {} => UmlGenerationMode::AllNodes,
//             PyUmlGenerationMode::PyNeighs { node } => UmlGenerationMode::Neighs(node.to_string()),
//         }
//     }
// }

// impl From<UmlGenerationMode> for PyUmlGenerationMode {
//     fn from(value: UmlGenerationMode) -> Self {
//         match value {
//             UmlGenerationMode::AllNodes => PyUmlGenerationMode::PyAllNodes {},
//             UmlGenerationMode::Neighs(node) => PyUmlGenerationMode::PyNeighs { node },
//         }
//     }
// }

// /// Wrapper for a MIE specification.
// ///
// /// Provides conversions to JSON and YAML representations.
// #[pyclass(name = "Mie")]
// pub struct PyMie {
//     /// Internal Rust struct holding the MIE specification.
//     inner: Mie,
// }

// #[pymethods]
// impl PyMie {
//     /// Returns a string representation of the MIE specification.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the MIE schema.
//     pub fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }

//     /// Converts the MIE specification to a JSON string.
//     ///
//     /// Serializes the internal schema to JSON.
//     ///
//     /// Returns:
//     ///     str: JSON representation of the MIE specification.
//     ///
//     /// Raises:
//     ///     PyRudofError: If serialization fails.
//     pub fn as_json(&self) -> PyResult<String> {
//         let str = self.inner.to_json().map_err(|e| PyRudofError::str(e.to_string()))?;
//         Ok(str)
//     }

//     /// Converts the MIE specification to a YAML string.
//     ///
//     /// Serializes the internal schema to YAML.
//     ///
//     /// Returns:
//     ///     str: YAML representation of the MIE specification.
//     pub fn as_yaml(&self) -> PyResult<String> {
//         let yaml = self.inner.to_yaml_str();
//         Ok(yaml)
//     }
// }

// /// Wrapper for a ShEx schema.
// ///
// /// Can be rendered as a string using the current formatter and used
// /// in validation, conversion and UML generation workflows.
// #[pyclass(name = "ShExSchema")]
// pub struct PyShExSchema {
//     /// Internal Rust struct holding the ShEx schema.
//     inner: ShExSchema,
// }

// #[pymethods]
// impl PyShExSchema {
//     /// Returns a string representation of the ShEx schema.
//     ///
//     /// Returns:
//     ///     str: A human-readable string representing the current ShEx schema.
//     pub fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }
// }

// /// Wrapper for a SPARQL Service Description.
// ///
// /// Based on SPARQL Service Description + VoID vocabularies.
// #[pyclass(name = "ServiceDescription")]
// pub struct PyServiceDescription {
//     /// Internal Rust struct holding the SPARQL Service Description.
//     inner: ServiceDescription,
// }

// #[pymethods]
// impl PyServiceDescription {
//     /// Returns a string representation of the service description.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the current service description.
//     pub fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }

//     /// Converts the Service Description to a MIE specification.
//     ///
//     /// Returns:
//     ///     PyMie: A Python `Mie` object containing the converted MIE specification.
//     pub fn as_mie(&self) -> PyResult<PyMie> {
//         let str = self.inner.service2mie();
//         Ok(PyMie { inner: str })
//     }

//     /// Serializes the current Service Description.
//     ///
//     /// By default, the serialization format is JSON. Other formats can be specified
//     /// via the `format` argument.
//     ///
//     /// Args:
//     ///     format (PyServiceDescriptionFormat, optional): The desired serialization format.
//     ///         Defaults to `PyServiceDescriptionFormat::Json`.
//     ///
//     /// Returns:
//     ///     str: The serialized service description as a string.
//     ///
//     /// Raises:
//     ///     RudofError: If serialization fails for any reason.
//     #[pyo3(signature = (format=None))]
//     pub fn serialize(&self, format: Option<&PyServiceDescriptionFormat>) -> PyResult<String> {
//         let mut v = Vec::new();
//         let service_description_format = cnv_service_description_format(format);
//         self.inner
//             .serialize(service_description_format, &mut v)
//             .map_err(|e| RudofError::SerializingServiceDescription { error: format!("{e}") })
//             .map_err(cnv_err)?;
//         let str = String::from_utf8(v)
//             .map_err(|e| RudofError::SerializingServiceDescription { error: format!("{e}") })
//             .map_err(cnv_err)?;
//         Ok(str)
//     }
// }

// /// Wrapper for a DCTAP profile.
// #[pyclass(name = "DCTAP")]
// pub struct PyDCTAP {
//     /// Internal Rust struct holding the DCTAP profile.
//     inner: DCTAP,
// }

// #[pymethods]
// impl PyDCTAP {
//     /// Returns a string representation of the DCTAP profile.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the DCTAP profile.
//     pub fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }

//     /// Returns a string representation of the DCTAP profile.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the DCTAP profile.
//     pub fn __str__(&self) -> String {
//         format!("{}", self.inner)
//     }
// }

// /// ShapeMap used for querying and validation.
// ///
// /// Represents associations between RDF nodes and shapes that can
// /// be used as input to ShEx validation.
// #[pyclass(name = "QueryShapeMap")]
// pub struct PyQueryShapeMap {
//     /// Internal Rust struct holding the query shape map.
//     inner: QueryShapeMap,
// }

// #[pymethods]
// impl PyQueryShapeMap {
//     /// Returns a string representation of the shape map.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the current shape map.
//     fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }
// }

// ///Result of comparing two schemas (ShaCo).
// ///
// /// Encapsulates structural differences and can be exported as JSON.
// #[pyclass(name = "ShaCo")]
// pub struct PyShaCo {
//     /// Internal Rust struct holding the schema comparison result.
//     inner: ShaCo,
// }

// #[pymethods]
// impl PyShaCo {
//     /// Returns a string representation of the schema comparison result.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the ShaCo comparison result.
//     pub fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }

//     /// Converts the schema comparison result to JSON.
//     ///
//     /// Returns:
//     ///     str: JSON representation of the schema comparison result.
//     ///
//     /// Raises:
//     ///     PyRudofError: If serialization fails.
//     pub fn as_json(&self) -> PyResult<String> {
//         let str = self.inner.as_json().map_err(|e| PyRudofError::str(e.to_string()))?;
//         Ok(str)
//     }
// }

// /// Common Shapes Model (CoShaMo) representation.
// ///
// /// An intermediate, comparison‑friendly representation derived from
// /// concrete schema languages such as ShEx or SHACL.
// #[pyclass(name = "CoShaMo")]
// pub struct PyCoShaMo {
//     /// Internal Rust struct holding the CoShaMo model.
//     inner: CoShaMo,
// }

// #[pymethods]
// impl PyCoShaMo {
//     /// Returns a string representation of the CoShaMo.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the CoShaMo instance.
//     pub fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }
// }

// /// Schema comparison format (e.g. ShExC, Turtle) used by the comparator.
// #[pyclass(name = "CompareSchemaFormat")]
// pub struct PyCompareSchemaFormat {
//     /// Internal Rust enum holding the schema format.
//     inner: CompareSchemaFormat,
// }

// #[pymethods]
// impl PyCompareSchemaFormat {
//     /// Returns a string representation of the CompareSchemaFormat.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the current format.
//     pub fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }

//     /// Returns a string representation of the CompareSchemaFormat.
//     ///
//     /// Equivalent to `__repr__`, but used for Python `str()` conversion.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the current format.
//     pub fn __str__(&self) -> String {
//         format!("{}", self.inner)
//     }

//     /// Returns a CompareSchemaFormat representing ShExC.
//     ///
//     /// Returns:
//     ///     PyCompareSchemaFormat: A static instance for ShExC format.
//     #[staticmethod]
//     pub fn shexc() -> Self {
//         Self {
//             inner: CompareSchemaFormat::ShExC,
//         }
//     }

//     /// Returns a CompareSchemaFormat representing Turtle.
//     ///
//     /// Returns:
//     ///     PyCompareSchemaFormat: A static instance for Turtle format.
//     #[staticmethod]
//     pub fn turtle() -> Self {
//         Self {
//             inner: CompareSchemaFormat::Turtle,
//         }
//     }
// }

// /// Schema comparison mode (e.g. ShEx) indicating the schema language.
// #[pyclass(name = "CompareSchemaMode")]
// pub struct PyCompareSchemaMode {
//     /// Internal Rust enum holding the schema mode.
//     inner: CompareSchemaMode,
// }

// #[pymethods]
// impl PyCompareSchemaMode {
//     /// Returns a string representation of the CompareSchemaMode.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the current schema mode.
//     pub fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }

//     /// Returns a string representation of the CompareSchemaMode.
//     ///
//     /// Equivalent to `__repr__`, but used for Python `str()` conversion.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the current schema mode.
//     pub fn __str__(&self) -> String {
//         format!("{}", self.inner)
//     }

//     /// Returns a CompareSchemaMode for ShEx.
//     ///
//     /// Returns:
//     ///     PyCompareSchemaMode: A static instance representing the ShEx schema mode.
//     #[staticmethod]
//     pub fn shex() -> Self {
//         Self {
//             inner: CompareSchemaMode::ShEx,
//         }
//     }
// }

// /// Intermediate representation of a SHACL schema.
// ///
// /// Used internally for SHACL validation and inspection.
// #[pyclass(name = "ShaclSchema")]
// pub struct PyShaclSchema {
//     /// Internal Rust struct holding the SHACL schema intermediate representation.
//     inner: ShaclSchemaIR,
// }

// #[pymethods]
// impl PyShaclSchema {
//     /// Returns a string representation of the ShaclSchema.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the SHACL schema.
//     pub fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }
// }

/// SHACL validation engine.
#[pyclass(eq, eq_int, name = "ShaclValidationMode")]
#[derive(PartialEq)]
pub enum PyShaclValidationMode {
    Native,
    Sparql,
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

// /// A single solution (row) of a SPARQL query.
// #[pyclass(name = "QuerySolution")]
// pub struct PyQuerySolution {
//     /// Internal Rust struct holding the solution data.
//     inner: QuerySolution<RdfData>,
// }

// #[pymethods]
// impl PyQuerySolution {
//     /// Converts the solution to a string representation.
//     ///
//     /// This is primarily used for displaying or debugging the solution.
//     ///
//     /// Returns:
//     ///     str: The string representation of the solution.
//     pub fn show(&self) -> String {
//         self.inner.show().to_string()
//     }

//     /// Returns the list of variables in this solution.
//     ///
//     /// Returns:
//     ///     List[str]: A vector containing the names of all variables in the solution.
//     pub fn variables(&self) -> Vec<String> {
//         let vars: Vec<String> = self.inner.variables().iter().map(|v| v.to_string()).collect();
//         vars
//     }

//     /// Returns the value of a variable by name, if it exists.
//     ///
//     /// Args:
//     ///     var_name (str): The name of the variable to look up.
//     ///
//     /// Returns:
//     ///     str | None: The value of the variable as a string, or `None` if the variable does not exist.
//     pub fn find(&self, var_name: &str) -> Option<String> {
//         self.inner
//             .find_solution(&VarName::new(var_name))
//             .map(|t| format!("{t}"))
//     }
// }

// /// A set of SPARQL query solutions.
// #[pyclass(name = "QuerySolutions")]
// pub struct PyQuerySolutions {
//     /// Internal Rust struct holding the SPARQL query solutions.
//     inner: QuerySolutions<RdfData>,
// }

// #[pymethods]
// impl PyQuerySolutions {
//     /// Converts the solutions into a human-readable table string.
//     ///
//     /// Returns:
//     ///     str: A formatted table representing all query solutions.
//     ///
//     /// Raises:
//     ///     PyRudofError: If an error occurs while converting the solutions to a string.
//     ///
//     /// Note:
//     ///     The table is encoded as UTF-8. Invalid UTF-8 sequences will cause a panic.
//     pub fn show(&self) -> Result<String, PyRudofError> {
//         let mut writer = Cursor::new(Vec::new());
//         self.inner
//             .write_table(&mut writer)
//             .map_err(|e| PyRudofError::str(format!("Error converting QuerySolutions to table: {e}")))?;
//         let result = String::from_utf8(writer.into_inner()).expect("Invalid UTF-8");
//         Ok(result)
//     }

//     /// Converts the solutions into a JSON string.
//     ///
//     /// Returns:
//     ///     str: JSON representation of all query solutions.
//     pub fn as_json(&self) -> String {
//         self.inner.as_json()
//     }

//     /// Returns the number of query solutions.
//     ///
//     /// Returns:
//     ///     int: The total number of solutions contained in this object.
//     pub fn count(&self) -> usize {
//         self.inner.count()
//     }

//     /// Converts the solutions into a list of `PyQuerySolution` objects.
//     ///
//     /// Returns:
//     ///     List[PyQuerySolution]: A list where each element represents a single query solution.
//     pub fn to_list(&self) -> Vec<PyQuerySolution> {
//         self.inner
//             .iter()
//             .map(|qs| PyQuerySolution { inner: qs.clone() })
//             .collect()
//     }

//     /// Returns an iterator over the query solutions.
//     ///
//     /// Returns:
//     ///     QuerySolutionIter: An iterator that allows looping over the solutions using a `for` loop.
//     fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<QuerySolutionIter>> {
//         let rs: Vec<PyQuerySolution> = slf
//             .inner
//             .iter()
//             .map(|qs| PyQuerySolution { inner: qs.clone() })
//             .collect();
//         let iter = QuerySolutionIter { inner: rs.into_iter() };
//         Py::new(slf.py(), iter)
//     }
// }

// /// Iterator over the solutions of a SPARQL query.
// #[pyclass]
// struct QuerySolutionIter {
//     /// Internal Rust iterator over the query solutions.
//     inner: std::vec::IntoIter<PyQuerySolution>,
// }

// #[pymethods]
// impl QuerySolutionIter {
//     /// Returns the iterator itself.
//     ///
//     /// This allows the iterator to be used in Python `for` loops.
//     ///
//     /// Returns:
//     ///     QuerySolutionIter: The iterator instance itself.
//     fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
//         slf
//     }

//     /// Returns the next solution in the iterator.
//     ///
//     /// Returns:
//     ///     PyQuerySolution | None: The next query solution if available, otherwise `None` when the iterator is exhausted.
//     fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyQuerySolution> {
//         slf.inner.next()
//     }
// }

// /// Result of a ShEx validation.
// #[pyclass(frozen, name = "ResultShapeMap")]
// pub struct PyResultShapeMap {
//     /// Internal Rust struct holding the validation results.
//     inner: ResultShapeMap,
// }

// #[pymethods]
// impl PyResultShapeMap {
//     /// Returns a list of tuples `(node, shape, status)` in the ResultShapeMap.
//     ///
//     /// Returns:
//     ///     List[Tuple[PyNode, PyShapeLabel, PyValidationStatus]]:
//     ///     Each tuple represents one validation result entry.
//     pub fn to_list(&self) -> Vec<(PyNode, PyShapeLabel, PyValidationStatus)> {
//         self.inner
//             .iter()
//             .map(|(node, shape, status)| {
//                 (
//                     PyNode {
//                         inner: node.as_object().clone(),
//                     },
//                     PyShapeLabel { inner: shape.clone() },
//                     PyValidationStatus { inner: status.clone() },
//                 )
//             })
//             .collect()
//     }

//     /// Convert the ResultShapeMap to a human-readable table string.
//     ///
//     /// The table can be sorted by node, shape, status, or details (default: Node),
//     /// may include validation details (default: False), and can be formatted
//     /// to fit a specific terminal width (default: 80 characters).
//     ///
//     /// Args:
//     ///     sort_mode (PySortModeResultMap, optional): Sorting mode for the table. Defaults to `Node`.
//     ///     with_details (bool, optional): Include detailed validation info. Defaults to `False`.
//     ///     terminal_width (int, optional): Width of the table for terminal formatting. Defaults to `80`.
//     ///
//     /// Returns:
//     ///     str: Formatted table representing the validation results.
//     ///
//     /// Raises:
//     ///     PyRudofError: If an error occurs during table formatting.
//     #[pyo3(signature = (sort_mode=None, with_details=None, terminal_width=None))]
//     pub fn show_as_table(
//         &self,
//         sort_mode: Option<&PySortModeResultMap>,
//         with_details: Option<bool>,
//         terminal_width: Option<usize>,
//     ) -> PyResult<String> {
//         let capture = CaptureWriter::new();
//         let capture_clone = capture.clone();
//         let boxed: Box<dyn Write> = Box::new(capture);
//         let sort_mode = cnv_sort_mode(sort_mode);

//         self.inner
//             .as_table(boxed, sort_mode, with_details, terminal_width)
//             .map_err(|e| PyRudofError::str(format!("Error converting ResultShapeMap to table: {e}")))?;
//         let result = capture_clone.to_string();
//         Ok(result)
//     }
// }

// /// A thread-safe writer that captures written bytes into an internal buffer.
// #[derive(Clone)]
// struct CaptureWriter(Arc<Mutex<Vec<u8>>>);

// impl CaptureWriter {
//     fn new() -> Self {
//         Self(Arc::new(Mutex::new(Vec::new())))
//     }
// }

// impl Display for CaptureWriter {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let buffer = self.0.lock().unwrap();
//         write!(f, "{}", String::from_utf8_lossy(&buffer).into_owned())
//     }
// }

// impl Write for CaptureWriter {
//     fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
//         self.0.lock().unwrap().write(buf)
//     }

//     fn flush(&mut self) -> std::io::Result<()> {
//         self.0.lock().unwrap().flush()
//     }
// }

// /// Result of a SHACL validation run.
// #[pyclass(frozen, name = "ValidationReport")]
// pub struct PyValidationReport {
//     /// Internal Rust representation of the validation report.
//     inner: ValidationReport,
// }

// #[pymethods]
// impl PyValidationReport {
//     /// Returns a human-readable string representation of the validation report.
//     ///
//     /// The string shows overall conformance status and a summary of violations
//     /// (if any). Suitable for logging or console output.
//     ///
//     /// Returns:
//     ///     str: Text representation of the validation results.
//     pub fn show(&self) -> String {
//         let result = &self.inner;
//         result.to_string()
//     }

//     /// Formats the validation report as a formatted table.
//     ///
//     /// Generates an aligned table showing each validation result with optional
//     /// detailed information about constraint violations. The table adapts to
//     /// the specified terminal width.
//     ///
//     /// Args:
//     ///     with_details (bool, optional): Include detailed violation messages
//     ///         and constraint component information. Defaults to ``False``.
//     ///     terminal_width (int, optional): Maximum width for the table (columns
//     ///         will wrap if needed). Defaults to ``80``.
//     ///
//     /// Returns:
//     ///     str: Formatted table as a multi-line string.
//     ///
//     /// Note:
//     ///     The table uses fixed-width columns and may include ANSI colors if
//     ///     the underlying formatter is configured for terminal output.
//     #[pyo3(signature = (with_details=None, terminal_width=None))]
//     pub fn show_as_table(&self, with_details: Option<bool>, terminal_width: Option<usize>) -> PyResult<String> {
//         let result = &self.inner;
//         let capture = CaptureWriter::new();
//         let capture_clone = capture.clone();
//         let boxed: Box<dyn Write> = Box::new(capture);
//         let sort_mode = SortModeReport::default();
//         result.show_as_table(boxed, sort_mode, with_details, terminal_width)?;
//         let result = capture_clone.to_string();
//         Ok(result)
//     }

//     /// Checks if the validation was fully successful.
//     ///
//     /// Returns:
//     ///     bool: ``True`` if data conforms to all SHACL constraints.
//     pub fn conforms(&self) -> bool {
//         self.inner.conforms()
//     }

//     /// Returns all individual validation results as a list.
//     ///
//     /// Each result represents a single constraint evaluation (success or violation)
//     /// and provides access to the focus node, constraint component, source shape,
//     /// path, value and error message.
//     ///
//     /// Returns:
//     ///     List[ValidationResult]: List of all validation results from the report.
//     pub fn validation_results(&self) -> Vec<PyValidationResult> {
//         self.inner
//             .results()
//             .iter()
//             .cloned()
//             .map(|result| PyValidationResult { inner: result })
//             .collect()
//     }
// }

// /// Single SHACL validation result (violation or success).
// #[pyclass(frozen, name = "ValidationResult")]
// pub struct PyValidationResult {
//     /// Internal Rust struct holding the validation result.
//     inner: ValidationResult,
// }

// #[pymethods]
// impl PyValidationResult {
//     /// Returns a string representation of the validation result.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing this validation result.
//     pub fn __repr__(&self) -> String {
//         format!("{}", self.inner)
//     }

//     /// Returns a string representation of the validation result.
//     ///
//     /// Equivalent to `__repr__`, used for Python `str()` conversion.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing this validation result.
//     pub fn __str__(&self) -> String {
//         format!("{}", self.inner)
//     }

//     /// Returns the focus node of the validation result.
//     ///
//     /// Returns:
//     ///     str: The RDF node being validated.
//     pub fn focus_node(&self) -> String {
//         self.inner.focus_node().to_string()
//     }

//     /// Returns the constraint component of the validation result.
//     ///
//     /// Returns:
//     ///     str: The SHACL constraint component that was violated or passed.
//     pub fn constraint_component(&self) -> String {
//         self.inner.component().to_string()
//     }

//     /// Returns the value of the validation result, if any.
//     ///
//     /// Returns:
//     ///     str: The value associated with the validation result, or empty string if none.
//     pub fn value(&self) -> String {
//         self.inner.value().map(|n| n.to_string()).unwrap_or_default()
//     }

//     /// Returns the path of the validation result, if any.
//     ///
//     /// Returns:
//     ///     str: The SHACL path related to this validation result, or empty string if none.
//     pub fn path(&self) -> String {
//         self.inner.path().map(|p| p.to_string()).unwrap_or_default()
//     }

//     /// Returns the source shape of the validation result, if any.
//     ///
//     /// Returns:
//     ///     str: The shape that produced this validation result, or empty string if none.
//     pub fn source_shape(&self) -> String {
//         self.inner.source().map(|s| s.to_string()).unwrap_or_default()
//     }

//     /// Returns a natural language message describing the validation result.
//     ///
//     /// Returns:
//     ///     str: Optional human-readable message explaining the result, or empty string if none.
//     pub fn message(&self) -> String {
//         self.inner.message().map(|m| m.to_string()).unwrap_or_default()
//     }
// }

// /// RDF node wrapper used in validation results and ShapeMaps.
// #[pyclass(frozen, name = "Node")]
// pub struct PyNode {
//     /// Internal Rust RDF object.
//     inner: Object,
// }

// #[pymethods]
// impl PyNode {
//     /// Convert the node to a string representation.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the RDF node.
//     pub fn show(&self) -> String {
//         let result = &self.inner;
//         format!("{result}")
//     }
// }

// /// Shape label wrapper used in ShEx results and ShapeMaps.
// #[pyclass(frozen, name = "ShapeLabel")]
// pub struct PyShapeLabel {
//     /// Internal Rust shape label.
//     inner: ShapeLabel,
// }

// #[pymethods]
// impl PyShapeLabel {
//     /// Convert the shape label to a string representation.
//     ///
//     /// Returns:
//     ///     str: Human-readable string representing the shape label.
//     pub fn show(&self) -> String {
//         let result = &self.inner;
//         result.to_string()
//     }
// }

// /// Status of a ShEx validation for a given node/shape pair.
// #[pyclass(frozen, name = "ValidationStatus")]
// pub struct PyValidationStatus {
//     /// Internal Rust validation status.
//     inner: ValidationStatus,
// }

// #[pymethods]
// impl PyValidationStatus {
//     /// Convert the validation status to a string representation.
//     ///
//     /// Returns:
//     ///     str: Human-readable string describing the validation status.
//     pub fn show(&self) -> String {
//         let result = &self.inner;
//         format!("{result}")
//     }

//     /// Returns true if the status is Conformant, false otherwise.
//     ///
//     /// Returns:
//     ///     bool: True if the node conforms to the shape, false if it violates.
//     pub fn is_conformant(&self) -> bool {
//         matches!(self.inner, ValidationStatus::Conformant(_))
//     }

//     /// Returns a natural language explanation for the reason of this status.
//     ///
//     /// Returns:
//     ///     str: Human-readable explanation describing the reason for the validation result.
//     pub fn reason(&self) -> String {
//         self.inner.reason().to_string()
//     }

//     /// Returns a JSON representation of the reason of this status.
//     ///
//     /// Returns:
//     ///     Any: Python object representing the JSON structure of the reason.
//     ///
//     /// Raises:
//     ///     PyRudofError: If the conversion to Python object fails.
//     pub fn as_json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
//         let value = self.inner.app_info();
//         let any = pythonize(py, &value)
//             .map_err(|e| PyRudofError::str(format!("Error converting appinfo to Python Object: {e}")))?;
//         Ok(any)
//     }
// }

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

impl From<PyRudofError> for PyErr {
    fn from(e: PyRudofError) -> Self {
        PyValueError::new_err(format!("{}", e.error))
    }
}

impl From<RudofError> for PyRudofError {
    fn from(error: RudofError) -> Self {
        println!("From<RudofError>: {error}");
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

// /// Converts a String error into a Python exception.
// ///
// /// Args:
// ///     e (String): The error message string to convert.
// ///
// /// Returns:
// ///     PyErr: Python exception corresponding to the error message.
// fn cnv_string_err(e: String) -> PyErr {
//     let e: PyRudofError = PyRudofError::str(e);
//     let e: PyErr = e.into();
//     e
// }

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
        PyDCTapFormat::CSV => Some(&DCTapFormat::Csv),
        PyDCTapFormat::XLSX => Some(&DCTapFormat::Xlsx),
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
        PyRDFFormat::Rdfxml => Some(&DataFormat::RdfXml),
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
        PyShapeMapFormat::JSON => Some(&ShapeMapFormat::Json),
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

// fn cnv_result_shex_validation_format(
//     format: Option<&PyResultShexValidationFormat>,
// ) -> Option<&ResultShExValidationFormat> {
//     format?;

//     match format.unwrap() {
//         PyResultShexValidationFormat::Compact => Some(&ResultShExValidationFormat::Compact),
//         PyResultShexValidationFormat::Csv => Some(&ResultShExValidationFormat::Csv),
//         PyResultShexValidationFormat::Details => Some(&ResultShExValidationFormat::Details),
//         PyResultShexValidationFormat::Json => Some(&ResultShExValidationFormat::Json),
//         PyResultShexValidationFormat::N3 => Some(&ResultShExValidationFormat::N3),
//         PyResultShexValidationFormat::NQuads => Some(&ResultShExValidationFormat::NQuads),
//         PyResultShexValidationFormat::RdfXml => Some(&ResultShExValidationFormat::RdfXml),
//         PyResultShexValidationFormat::TriG => Some(&ResultShExValidationFormat::TriG),
//         PyResultShexValidationFormat::Turtle => Some(&ResultShExValidationFormat::Turtle),
//         PyResultShexValidationFormat::NTriples => Some(&ResultShExValidationFormat::NTriples),
//     }
// }

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
        PyShaclFormat::Rdfxml => Some(&ShaclFormat::RdfXml),
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

// /// Converts a Python shapes graph source enum into the corresponding Rust `ShapesGraphSource`.
// ///
// /// Args:
// ///     sgs (PyShapesGraphSource): Python enum indicating source of SHACL shapes.
// ///
// /// Returns:
// ///     ShapesGraphSource: Corresponding Rust enum.
// fn cnv_shapes_graph_source(sgs: Option<&PyShapesGraphSource>) -> Option<&ShapesGraphSource> {
//     sgs?;

//     match sgs.unwrap() {
//         PyShapesGraphSource::CurrentData => Some(&ShapesGraphSource::CurrentData),
//         PyShapesGraphSource::CurrentSchema => Some(&ShapesGraphSource::CurrentSchema),
//     }
// }

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
        PyQueryResultFormat::Rdfxml => Some(&ResultQueryFormat::RdfXml),
        PyQueryResultFormat::CSV => Some(&ResultQueryFormat::Csv),
        PyQueryResultFormat::TriG => Some(&ResultQueryFormat::TriG),
        PyQueryResultFormat::N3 => Some(&ResultQueryFormat::N3),
        PyQueryResultFormat::NQuads => Some(&ResultQueryFormat::NQuads),
    }
}

// /// Converts a Python sort mode result map into the corresponding Rust `SortMode`.
// ///
// /// Args:
// ///     format (PySortModeResultMap): Python enum for sort mode result format.
// ///
// /// Returns:
// ///     SortMode: Corresponding Rust enum.
// fn cnv_sort_mode(mode: Option<&PySortModeResultMap>) -> Option<&SortMode> {
//     mode?;

//     match mode.unwrap() {
//         PySortModeResultMap::Node => Some(&SortMode::Node),
//         PySortModeResultMap::Shape => Some(&SortMode::Shape),
//         PySortModeResultMap::Status => Some(&SortMode::Status),
//         PySortModeResultMap::Details => Some(&SortMode::Details),
//     }
// }

// /// Opens a file at the given path and returns a buffered reader.
// ///
// /// Args:
// ///     path_name (str): Path to the file.
// ///     context (str): Context description for error messages.
// ///
// /// Returns:
// ///     BufReader<File>: Buffered reader for the opened file.
// ///
// /// Raises:
// ///     RudofError: If the file cannot be opened.
// fn get_path_reader(path_name: &str, context: &str) -> PyResult<BufReader<File>> {
//     let path = Path::new(path_name);
//     let file = File::open::<&OsStr>(path.as_ref())
//         .map_err(|e| RudofError::ReadingPathContext {
//             path: path_name.to_string(),
//             context: context.to_string(),
//             error: format!("{e}"),
//         })
//         .map_err(cnv_err)?;
//     let reader = BufReader::new(file);
//     Ok(reader)
// }

// /// Returns a reader for an input specification string.
// ///
// /// Args:
// ///     input (str): Input specification (path, URL, or inline data).
// ///     accept (Optional[str]): Accepted format(s) of the input.
// ///     context (str): Context description for error messages.
// ///
// /// Returns:
// ///     InputSpecReader: Reader for the given input specification.
// ///
// /// Raises:
// ///     RudofError: If parsing or opening the input fails.
// fn get_reader(input: &str, accept: Option<&str>, context: &str) -> PyResult<InputSpecReader> {
//     let input_spec: InputSpec = FromStr::from_str(input)
//         .map_err(|e: InputSpecError| RudofError::ParsingInputSpecContext {
//             input: input.to_string(),
//             context: context.to_string(),
//             error: e.to_string(),
//         })
//         .map_err(cnv_err)?;
//     let reader = input_spec
//         .open_read(accept, context)
//         .map_err(|e| RudofError::ReadingInputSpecContext {
//             input: input.to_string(),
//             context: context.to_string(),
//             error: e.to_string(),
//         })
//         .map_err(cnv_err)?;
//     Ok(reader)
// }
