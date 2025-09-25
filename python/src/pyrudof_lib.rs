#![allow(unsafe_op_in_unsafe_fn)]
//! This is a wrapper of the methods provided by `rudof_lib`
//!
use pyo3::{
    Py, PyErr, PyRef, PyRefMut, PyResult, Python, exceptions::PyValueError, pyclass, pymethods,
};
use rudof_lib::{
    CoShaMo, ComparatorError, CompareSchemaFormat, CompareSchemaMode, DCTAP, DCTAPFormat,
    InputSpec, InputSpecError, InputSpecReader, Mie, PrefixMap, QueryResultFormat, QueryShapeMap,
    QuerySolution, QuerySolutions, RDFFormat, RdfData, ReaderMode, ResultShapeMap, Rudof,
    RudofError, ServiceDescription, ServiceDescriptionFormat, ShExFormat, ShExFormatter,
    ShExSchema, ShaCo, ShaclFormat, ShaclSchemaIR, ShaclValidationMode, ShapeMapFormat,
    ShapeMapFormatter, ShapesGraphSource, UmlGenerationMode, UrlSpec, ValidationReport,
    ValidationStatus, VarName, iri,
};
use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    str::FromStr,
};

use crate::PyRudofConfig;

/// Main class to handle `rudof` features.
/// There should  be only one instance of `rudof` per program.
/// It holds the current RDF data, ShEx schema, SHACL shapes graph, Shapemap and DCTAP
/// It can be used to read data, schemas, shapemaps and DCTAP from strings or files,
/// run queries, validate data, convert schemas to Common Shapes Model, compare schemas, etc.
/// It is thread safe.
#[pyclass(name = "Rudof")]
pub struct PyRudof {
    inner: Rudof,
}

#[pymethods]
impl PyRudof {
    #[new]
    pub fn __init__(config: &PyRudofConfig) -> PyResult<Self> {
        Ok(Self {
            inner: Rudof::new(&config.inner),
        })
    }

    pub fn update_config(&mut self, config: &PyRudofConfig) {
        self.inner.update_config(&config.inner)
    }

    /// Obtain the version of the Rudof library
    #[pyo3(signature = ())]
    pub fn version(&self) -> PyResult<String> {
        let str = env!("CARGO_PKG_VERSION").to_string();
        Ok(str)
    }

    /// Resets the current RDF data
    #[pyo3(signature = ())]
    pub fn reset_data(&mut self) {
        self.inner.reset_data();
    }

    /// Resets the current ShEx schema
    #[pyo3(signature = ())]
    pub fn reset_shex(&mut self) {
        self.inner.reset_shex();
    }

    /// Resets the current shapemap
    #[pyo3(signature = ())]
    pub fn reset_shapemap(&mut self) {
        self.inner.reset_shapemap();
    }

    /// Resets the current SHACL shapes graph
    #[pyo3(signature = ())]
    pub fn reset_shacl(&mut self) {
        self.inner.reset_shacl();
    }

    /// Resets all current values
    #[pyo3(signature = ())]
    pub fn reset_all(&mut self) {
        self.inner.reset_all()
    }

    /// Obtains the current DCTAP
    #[pyo3(signature = ())]
    pub fn get_dctap(&self) -> Option<PyDCTAP> {
        let dctap = self.inner.get_dctap();
        dctap.map(|s| PyDCTAP { inner: s.clone() })
    }

    /// Obtains the current ShEx Schema
    #[pyo3(signature = ())]
    pub fn get_shex(&self) -> Option<PyShExSchema> {
        let shex_schema = self.inner.get_shex();
        shex_schema.map(|s| PyShExSchema { inner: s.clone() })
    }

    /// Obtains the current Service Description
    #[pyo3(signature = ())]
    pub fn get_service_description(&self) -> Option<PyServiceDescription> {
        let service_description = self.inner.get_service_description();
        service_description.map(|s| PyServiceDescription { inner: s.clone() })
    }

    /// Get a Common Shapes Model from a schema
    /// Parameters:
    /// schema: String containing the schema
    /// mode: Mode of the schema, e.g. shex
    /// format: Format of the schema, e.g. shexc, turtle
    /// base: Optional base IRI to resolve relative IRIs in the schema
    /// reader_mode: Reader mode to use when reading the schema, e.g. lax, strict
    /// label: Optional label of the shape to convert or None to use the start shape or the first shape
    #[pyo3(signature = (schema, mode, format, base, reader_mode, label))]
    pub fn get_coshamo_str(
        &mut self,
        schema: &str,
        mode: &str,
        format: &str,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
        label: Option<&str>,
    ) -> PyResult<PyCoShaMo> {
        // Implementation goes here
        let format = CompareSchemaFormat::from_str(format).map_err(cnv_comparator_err)?;
        let mode = CompareSchemaMode::from_str(mode).map_err(cnv_comparator_err)?;
        let mut reader = schema.as_bytes();
        let coshamo = self
            .inner
            .get_coshamo(
                &mut reader,
                &mode,
                &format,
                base,
                &reader_mode.into(),
                label,
                Some("string"),
            )
            .map_err(PyRudofError::from)?;
        Ok(PyCoShaMo { inner: coshamo })
    }

    /// Compares two schemas provided as strings
    /// Parameters: schema1, schema2: Strings containing the schemas to compare
    /// mode1, mode2: Mode of the schemas, e.g. shex
    /// format1, format2: Format of the schemas, e.g. shexc, turtle
    /// label1, label2: Optional labels of the shapes to compare
    /// base1, base2: Optional base IRIs to resolve relative IRIs in the schemas
    /// reader_mode: Reader mode to use when reading the schemas, e.g. lax, strict
    #[pyo3(signature = (schema1, schema2, mode1, mode2, format1, format2, base1, base2, label1, label2, reader_mode = &PyReaderMode::Lax))]
    #[allow(clippy::too_many_arguments)]
    pub fn compare_schemas_str(
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
        reader_mode: &PyReaderMode,
    ) -> PyResult<PyShaCo> {
        let format1 = CompareSchemaFormat::from_str(format1).map_err(cnv_comparator_err)?;
        let format2 = CompareSchemaFormat::from_str(format2).map_err(cnv_comparator_err)?;
        let mode1 = CompareSchemaMode::from_str(mode1).map_err(cnv_comparator_err)?;
        let mode2 = CompareSchemaMode::from_str(mode2).map_err(cnv_comparator_err)?;
        let mut reader1 = schema1.as_bytes();
        let coshamo1 = self
            .inner
            .get_coshamo(
                &mut reader1,
                &mode1,
                &format1,
                base1,
                &reader_mode.into(),
                label1,
                Some("string"),
            )
            .map_err(PyRudofError::from)?;

        let mut reader2 = schema2.as_bytes();
        let coshamo2 = self
            .inner
            .get_coshamo(
                &mut reader2,
                &mode2,
                &format2,
                base2,
                &reader_mode.into(),
                label2,
                Some("string"),
            )
            .map_err(PyRudofError::from)?;
        let shaco = coshamo1.compare(&coshamo2);
        Ok(PyShaCo { inner: shaco })
    }

    /// Obtains the current Shapemap
    #[pyo3(signature = ())]
    pub fn get_shapemap(&self) -> Option<PyQueryShapeMap> {
        let shapemap = self.inner.get_shapemap();
        shapemap.map(|s| PyQueryShapeMap { inner: s.clone() })
    }

    /// Obtains the current SHACL schema
    #[pyo3(signature = ())]
    pub fn get_shacl(&self) -> Option<PyShaclSchema> {
        let shacl_schema = self.inner.get_shacl_ir();
        shacl_schema.map(|s| PyShaclSchema { inner: s.clone() })
    }

    /// Run a SPARQL SELECT query obtained from a string on the RDF data
    #[pyo3(signature = (input))]
    pub fn run_query_str(&mut self, input: &str) -> PyResult<PyQuerySolutions> {
        let results = self.inner.run_query_select_str(input).map_err(cnv_err)?;
        Ok(PyQuerySolutions { inner: results })
    }

    /// Run a SPARQL CONSTRUCT query obtained from a string on the RDF data
    #[pyo3(signature = (input, format = &PyQueryResultFormat::Turtle))]
    pub fn run_query_construct_str(
        &mut self,
        input: &str,
        format: &PyQueryResultFormat,
    ) -> PyResult<String> {
        let format = cnv_query_result_format(format);
        let str = self
            .inner
            .run_query_construct_str(input, &format)
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Run the current query on the current RDF data if it is a CONSTRUCT query
    #[pyo3(signature = (format = &PyQueryResultFormat::Turtle))]
    pub fn run_current_query_construct(
        &mut self,
        format: &PyQueryResultFormat,
    ) -> PyResult<String> {
        let format = cnv_query_result_format(format);
        let str = self
            .inner
            .run_current_query_construct(&format)
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Run the current query on the current RDF data if it is a SELECT query
    #[pyo3(signature = ())]
    pub fn run_current_query_select(&mut self) -> PyResult<PyQuerySolutions> {
        let results = self.inner.run_current_query_select().map_err(cnv_err)?;
        Ok(PyQuerySolutions { inner: results })
    }

    /// Get the current version of Rudof
    pub fn get_version(&self) -> PyResult<String> {
        Ok(self.inner.get_version().to_string())
    }

    /// Reads a SPARQL query from a String and stores it as the current query
    pub fn read_query_str(&mut self, input: &str) -> PyResult<()> {
        self.inner.read_query_str(input).map_err(cnv_err)
    }

    /// Reads a SPARQL query from a file path or URL and stores it as the current query
    pub fn read_query(&mut self, input: &str) -> PyResult<()> {
        let mut reader = get_reader(input, Some("application/sparql-query"), "SPARQL query")?;
        self.inner
            .read_query(&mut reader, Some(input))
            .map_err(cnv_err)
    }

    /// Resets the current SPARQL query
    pub fn reset_query(&mut self) {
        self.inner.reset_query()
    }

    /// Run a SPARQL query obtained from a file path on the RDF data
    /// Parameters:
    /// path_name: Path to the file containing the SPARQL query
    /// Returns: QuerySolutions object containing the results of the query
    /// Raises: RudofError if there is an error reading the file or running the query
    /// Example:
    ///   rudof.run_query_path("query.sparql")
    #[pyo3(signature = (path_name))]
    pub fn run_query_path(&mut self, path_name: &str) -> PyResult<PyQuerySolutions> {
        let mut reader = get_path_reader(path_name, "SPARQL query")?;
        let results = self.inner.run_query_select(&mut reader).map_err(cnv_err)?;
        Ok(PyQuerySolutions { inner: results })
    }

    /// Reads DCTAP from a String
    /// Parameters:
    /// input: String containing the DCTAP data
    /// format: Format of the DCTAP data, e.g. csv, tsv
    /// Returns: None
    /// Raises: RudofError if there is an error reading the DCTAP data
    #[pyo3(signature = (input, format = &PyDCTapFormat::CSV))]
    pub fn read_dctap_str(&mut self, input: &str, format: &PyDCTapFormat) -> PyResult<()> {
        self.inner.reset_dctap();
        let format = cnv_dctap_format(format);
        self.inner
            .read_dctap(input.as_bytes(), &format)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads DCTAP from a path
    /// Parameters:
    /// path_name: Path to the file containing the DCTAP data
    /// format: Format of the DCTAP data, e.g. csv, tsv
    /// Returns: None
    /// Raises: RudofError if there is an error reading the DCTAP data
    #[pyo3(signature = (path_name, format = &PyDCTapFormat::CSV))]
    pub fn read_dctap_path(&mut self, path_name: &str, format: &PyDCTapFormat) -> PyResult<()> {
        let reader = get_path_reader(path_name, "DCTAP data")?;
        self.inner.reset_dctap();
        let format = cnv_dctap_format(format);
        self.inner.read_dctap(reader, &format).map_err(cnv_err)?;
        Ok(())
    }

    /// Reads a ShEx schema from a string
    /// Parameters:
    /// input: String containing the ShEx schema
    /// format: Format of the ShEx schema, e.g. shexc, turtle
    /// base: Optional base IRI to resolve relative IRIs in the schema
    /// reader_mode: Reader mode to use when reading the schema, e.g. lax, strict
    /// Returns: None
    /// Raises: RudofError if there is an error reading the ShEx schema
    ///
    #[pyo3(signature = (input, format = &PyShExFormat::ShExC, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_shex_str(
        &mut self,
        input: &str,
        format: &PyShExFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let format = cnv_shex_format(format);
        self.inner.reset_shex();
        self.inner
            .read_shex(
                input.as_bytes(),
                &format,
                base,
                &reader_mode.into(),
                Some("string"),
            )
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads a SHACL shapes graph from a string
    /// Parameters:
    /// input: String containing the SHACL shapes graph
    /// format: Format of the SHACL shapes graph, e.g. turtle
    /// base: Optional base IRI to resolve relative IRIs in the shapes graph
    /// reader_mode: Reader mode to use when reading the shapes graph, e.g. lax, strict
    /// Returns: None
    /// Raises: RudofError if there is an error reading the SHACL shapes graph
    #[pyo3(signature = (input, format = &PyShaclFormat::Turtle, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_shacl_str(
        &mut self,
        input: &str,
        format: &PyShaclFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let format = cnv_shacl_format(format);
        let reader_mode = cnv_reader_mode(reader_mode);
        self.inner.reset_shacl();
        self.inner
            .read_shacl(input.as_bytes(), &format, base, &reader_mode)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Obtains a ShEx schema
    /// Parameters:
    /// input: Can be a file path or an URL
    /// format: Format of the ShEx schema, e.g. shexc, turtle
    /// base: Optional base IRI to resolve relative IRIs in the schema
    /// reader_mode: Reader mode to use when reading the schema, e.g. lax, strict
    /// Returns: None
    /// Raises: RudofError if there is an error reading the ShEx schema
    ///
    #[pyo3(signature = (input, format = &PyShExFormat::ShExC, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_shex(
        &mut self,
        input: &str,
        format: &PyShExFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let format = cnv_shex_format(format);
        self.inner.reset_shex();
        let reader = get_reader(input, Some(format.mime_type()), "ShEx schema")?;
        self.inner
            .read_shex(reader, &format, base, &reader_mode.into(), Some("string"))
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads a SHACL shapes graph
    /// Parameters:
    /// input: URL of file path
    /// format: Format of the SHACL shapes graph, e.g. turtle
    /// base: Optional base IRI to resolve relative IRIs in the shapes graph
    /// reader_mode: Reader mode to use when reading the shapes graph, e.g. lax, strict
    /// Returns: None
    /// Raises: RudofError if there is an error reading the SHACL shapes graph
    #[pyo3(signature = (input, format = &PyShaclFormat::Turtle, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_shacl(
        &mut self,
        input: &str,
        format: &PyShaclFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let format = cnv_shacl_format(format);
        let reader = get_url_reader(input, Some(format.mime_type()), "SHACL shapes graph")?;
        self.inner.reset_shacl();
        let reader_mode = cnv_reader_mode(reader_mode);
        self.inner
            .read_shacl(reader, &format, base, &reader_mode)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Resets the current ShEx validation results
    /// Returns: None
    /// Raises: None
    #[pyo3(signature = ())]
    pub fn reset_validation_results(&mut self) {
        self.inner.reset_validation_results();
    }

    /// Converts the current RDF data to a Visual representation in PlantUML, that visual representation can be later converted to SVG or PNG pictures using PlantUML processors
    /// Returns: String containing the PlantUML representation of the current RDF data
    /// Raises: RudofError if there is an error generating the UML
    #[pyo3(signature = ())]
    pub fn data2plantuml(&self) -> PyResult<String> {
        let mut v = Vec::new();
        self.inner
            .data2plant_uml(&mut v)
            .map_err(|e| RudofError::RDF2PlantUmlError {
                error: format!("Error generating UML for current RDF data: {e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::RDF2PlantUmlError {
                error: format!("RDF2PlantUML: Error converting generated vector to UML: {e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Converts the current RDF data to a Visual representation in PlantUML and stores it in a file
    /// That visual representation can be later converted to SVG or PNG pictures using PlantUML processors
    /// Parameters:
    /// file_name: Path to the file where the PlantUML representation of the current RDF
    /// data will be stored
    /// Returns: None
    /// Raises: RudofError if there is an error generating the UML or writing the file
    #[pyo3(signature = (file_name))]
    pub fn data2plantuml_file(&self, file_name: &str) -> PyResult<()> {
        let file = File::create(file_name)?;
        let mut writer = BufWriter::new(file);
        self.inner
            .data2plant_uml(&mut writer)
            .map_err(|e| RudofError::RDF2PlantUmlError {
                error: format!("Error generating UML for current RDF data: {e}"),
            })
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads RDF data (and merges it with existing data)
    /// Parameters:
    /// input: Path or URL containing the RDF data
    /// format: Format of the RDF data, e.g. turtle, jsonld
    /// base: Optional base IRI to resolve relative IRIs in the RDF data
    /// reader_mode: Reader mode to use when reading the RDF data, e.g. lax, strict
    /// Returns: None
    /// Raises: RudofError if there is an error reading the RDF data
    #[pyo3(signature = (input, format = &PyRDFFormat::Turtle, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_data(
        &mut self,
        input: &str,
        format: &PyRDFFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let reader_mode = cnv_reader_mode(reader_mode);
        let format = cnv_rdf_format(format);
        let reader = get_reader(input, Some(format.mime_type()), "RDF data")?;
        self.inner
            .read_data(reader, &format, base, &reader_mode)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Read Service Description
    /// Parameters:
    /// input: Path or URL
    /// format: Format of the Service Description, e.g. turtle, jsonld
    /// base: Optional base IRI to resolve relative IRIs in the Service Description
    /// reader_mode: Reader mode to use when reading the Service Description, e.g. lax
    /// Returns: None
    /// Raises: RudofError if there is an error reading the Service Description
    #[pyo3(signature = (input, format = &PyRDFFormat::Turtle, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_service_description(
        &mut self,
        input: &str,
        format: &PyRDFFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let reader_mode = cnv_reader_mode(reader_mode);
        let format = cnv_rdf_format(format);
        let reader = get_reader(input, Some(format.mime_type()), "Service Description")?;
        self.inner
            .read_service_description(reader, &format, base, &reader_mode)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Read Service Description from a String
    /// Parameters:
    /// input: String that contains the Service Description
    /// format: Format of the Service Description, e.g. turtle, jsonld
    /// base: Optional base IRI to resolve relative IRIs in the Service Description
    /// reader_mode: Reader mode to use when reading the Service Description, e.g. lax
    /// Returns: None
    /// Raises: RudofError if there is an error reading the Service Description
    #[pyo3(signature = (input, format = &PyRDFFormat::Turtle, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_service_description_str(
        &mut self,
        input: &str,
        format: &PyRDFFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let reader_mode = cnv_reader_mode(reader_mode);
        let format = cnv_rdf_format(format);
        self.inner
            .read_service_description(input.as_bytes(), &format, base, &reader_mode)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Serialize the current Service Description to a file
    /// Parameters:
    /// format: Format of the Service Description, e.g. turtle, jsonld
    /// output: Path to the file where the Service Description will be stored
    /// Returns: None
    /// Raises: RudofError if there is an error writing the Service Description
    #[pyo3(signature = (output, format = &PyServiceDescriptionFormat::Internal))]
    pub fn serialize_service_description(
        &self,
        output: &str,
        format: &PyServiceDescriptionFormat,
    ) -> PyResult<()> {
        let file = File::create(output)?;
        let mut writer = BufWriter::new(file);
        let service_description_format = cnv_service_description_format(format);
        self.inner
            .serialize_service_description(&service_description_format, &mut writer)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Adds RDF data read from a String to the current RDF Data
    ///
    /// Parameters:
    /// input: String containing the RDF data
    /// format: Format of the RDF data, e.g. turtle, jsonld
    /// base: Optional base IRI to resolve relative IRIs in the RDF data
    /// reader_mode: Reader mode to use when reading the RDF data, e.g. lax
    /// Returns: None
    /// Raises: RudofError if there is an error reading the RDF data
    #[pyo3(signature = (input, format = &PyRDFFormat::Turtle, base = None, reader_mode = &PyReaderMode::Lax))]
    pub fn read_data_str(
        &mut self,
        input: &str,
        format: &PyRDFFormat,
        base: Option<&str>,
        reader_mode: &PyReaderMode,
    ) -> PyResult<()> {
        let reader_mode = cnv_reader_mode(reader_mode);
        let format = cnv_rdf_format(format);
        self.inner
            .read_data(input.as_bytes(), &format, base, &reader_mode)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Serialize the current ShEx schema
    #[pyo3(signature = (format = &PyRDFFormat::Turtle))]
    pub fn serialize_data(&self, format: &PyRDFFormat) -> PyResult<String> {
        let mut v = Vec::new();
        let format = cnv_rdf_format(format);
        self.inner
            .serialize_data(&format, &mut v)
            .map_err(|e| RudofError::SerializingData {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::SerializingData {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Reads the current Shapemap from a String
    #[pyo3(signature = (input,format = &PyShapeMapFormat::Compact))]
    pub fn read_shapemap_str(&mut self, input: &str, format: &PyShapeMapFormat) -> PyResult<()> {
        let format = cnv_shapemap_format(format);
        self.inner
            .read_shapemap(input.as_bytes(), &format)
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Reads the current Shapemap from a file path
    #[pyo3(signature = (input,format = &PyShapeMapFormat::Compact))]
    pub fn read_shapemap(&mut self, input: &str, format: &PyShapeMapFormat) -> PyResult<()> {
        let format = cnv_shapemap_format(format);
        let reader = get_reader(input, Some(format.mime_type()), "Shapemap")?;
        self.inner.read_shapemap(reader, &format).map_err(cnv_err)?;
        Ok(())
    }

    /// Validate the current RDF Data with the current ShEx schema and the current Shapemap
    ///
    /// In order to validate, a ShEx Schema and a ShapeMap has to be read
    #[pyo3(signature = ())]
    pub fn validate_shex(&mut self) -> PyResult<PyResultShapeMap> {
        let result = self.inner.validate_shex().map_err(cnv_err)?;
        Ok(PyResultShapeMap { inner: result })
    }

    /// Validates the current RDF Data
    ///
    /// mode can be native to use Native implementation or SPARQL to use the SPARQL based implementation
    /// shapes_graph_source: Indicates the source of the shapes graph,
    /// which can be extracted from the current RDF data,
    /// or from the current SHACL schema.
    /// If there is no current SHACL schema, it tries to get it from the current RDF data
    #[pyo3(signature = (mode = &PyShaclValidationMode::Native, shapes_graph_source = &PyShapesGraphSource::CurrentSchema ))]
    pub fn validate_shacl(
        &mut self,
        mode: &PyShaclValidationMode,
        shapes_graph_source: &PyShapesGraphSource,
    ) -> PyResult<PyValidationReport> {
        let mode = cnv_shacl_validation_mode(mode);
        let shapes_graph_source = cnv_shapes_graph_source(shapes_graph_source);
        let result = self
            .inner
            .validate_shacl(&mode, &shapes_graph_source)
            .map_err(cnv_err)?;
        Ok(PyValidationReport { inner: result })
    }

    /// Converts the current DCTAP to ShEx and replaces the current ShEx by the resulting ShEx
    pub fn dctap2shex(&mut self) -> PyResult<()> {
        self.inner.dctap2shex().map_err(cnv_err)
    }

    /// Converts the current ShEx to a Class-like diagram using PlantUML syntax
    #[pyo3(signature = (uml_mode))]
    pub fn shex2plantuml(&self, uml_mode: &PyUmlGenerationMode) -> PyResult<String> {
        let mut v = Vec::new();
        self.inner
            .shex2plant_uml(&uml_mode.into(), &mut v)
            .map_err(|e| RudofError::ShEx2PlantUmlError {
                error: format!("Error generating UML: {e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::ShEx2PlantUmlError {
                error: format!("ShEx2PlantUML: Error converting generated vector to UML: {e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Converts the current ShEx to a Class-like diagram using PlantUML syntax and stores it in a file
    #[pyo3(signature = (uml_mode, file_name))]
    pub fn shex2plantuml_file(
        &self,
        uml_mode: &PyUmlGenerationMode,
        file_name: &str,
    ) -> PyResult<()> {
        let file = File::create(file_name)?;
        let mut writer = BufWriter::new(file);
        self.inner
            .shex2plant_uml(&uml_mode.into(), &mut writer)
            .map_err(|e| RudofError::ShEx2PlantUmlError {
                error: format!("Error generating UML: {e} in {file_name}"),
            })
            .map_err(cnv_err)?;
        Ok(())
    }

    /// Serialize the current ShEx schema
    #[pyo3(signature = (formatter, format = &PyShExFormat::ShExC))]
    pub fn serialize_current_shex(
        &self,
        formatter: &PyShExFormatter,
        format: &PyShExFormat,
    ) -> PyResult<String> {
        let mut v = Vec::new();
        let format = cnv_shex_format(format);
        self.inner
            .serialize_current_shex(&format, &formatter.inner, &mut v)
            .map_err(|e| RudofError::SerializingShEx {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::SerializingShEx {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Serialize a ShEx schema
    #[pyo3(signature = (shex, formatter, format = &PyShExFormat::ShExC))]
    pub fn serialize_shex(
        &self,
        shex: &PyShExSchema,
        formatter: &PyShExFormatter,
        format: &PyShExFormat,
    ) -> PyResult<String> {
        let mut v = Vec::new();
        let format = cnv_shex_format(format);
        self.inner
            .serialize_shex(&shex.inner, &format, &formatter.inner, &mut v)
            .map_err(|e| RudofError::SerializingShEx {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::SerializingShEx {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Serialize the current SHACL shapes graph
    #[pyo3(signature = (format = &PyShaclFormat::Turtle))]
    pub fn serialize_shacl(&self, format: &PyShaclFormat) -> PyResult<String> {
        let mut v = Vec::new();
        let format = cnv_shacl_format(format);
        self.inner
            .serialize_shacl(&format, &mut v)
            .map_err(|e| RudofError::SerializingShacl {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::SerializingShacl {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Serialize the current Query Shape Map
    #[pyo3(signature = (formatter, format = &PyShapeMapFormat::Compact))]
    pub fn serialize_shapemap(
        &self,
        formatter: &PyShapeMapFormatter,
        format: &PyShapeMapFormat,
    ) -> PyResult<String> {
        let mut v = Vec::new();
        let format = cnv_shapemap_format(format);
        self.inner
            .serialize_shapemap(&format, &formatter.inner, &mut v)
            .map_err(|e| RudofError::SerializingShacl {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::SerializingShacl {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /// Adds an endpoint to the current RDF Data
    #[pyo3(signature = (endpoint))]
    pub fn add_endpoint(&mut self, endpoint: &str) -> PyResult<()> {
        // TODO: Check if it is in the RDF Data Config endpoints...
        let config = self.inner.config();
        let (endpoint_iri, prefixmap) =
            if let Some(endpoint_descr) = config.rdf_data_config().find_endpoint(endpoint) {
                (
                    endpoint_descr.query_url().clone(),
                    endpoint_descr.prefixmap().clone(),
                )
            } else {
                let iri = iri!(endpoint);
                (iri, PrefixMap::basic())
            };
        self.inner
            .add_endpoint(&endpoint_iri, &prefixmap)
            .map_err(cnv_err)
    }
}

/// Declares a `ReaderMode` for parsing RDF data
#[pyclass(eq, eq_int, name = "ReaderMode")]
#[derive(PartialEq)]
pub enum PyReaderMode {
    /// It ignores the errors and tries to continue the processing
    Lax,

    /// It fails with the first error
    Strict,
}

#[pymethods]
impl PyReaderMode {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.detach(|| PyReaderMode::Lax)
    }
}

impl From<&PyReaderMode> for ReaderMode {
    fn from(mode: &PyReaderMode) -> Self {
        match mode {
            PyReaderMode::Lax => ReaderMode::Lax,
            PyReaderMode::Strict => ReaderMode::Strict,
        }
    }
}

/// RDF Data format
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "RDFFormat")]
#[derive(PartialEq)]
pub enum PyRDFFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    JsonLd,
}

/// Query Result format
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "QueryResultFormat")]
#[derive(PartialEq)]
pub enum PyQueryResultFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    CSV,
}

/// DCTAP format
/// Currently, only CSV and XLSX are supported
/// The default is CSV
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "DCTapFormat")]
#[derive(PartialEq)]
pub enum PyDCTapFormat {
    CSV,
    XLSX,
}

/// Service Description format
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ServiceDescriptionFormat")]
#[derive(PartialEq)]
pub enum PyServiceDescriptionFormat {
    Internal,
    Json,
    Mie,
}

/// ShapeMap format
/// Currently, only Compact and JSON are supported
/// The default is Compact
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ShapeMapFormat")]
#[derive(PartialEq)]
pub enum PyShapeMapFormat {
    Compact,
    JSON,
}

/// ShEx format
/// Currently, only ShExC, ShExJ and Turtle are supported
/// The default is ShExC
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ShExFormat")]
#[derive(PartialEq)]
pub enum PyShExFormat {
    ShExC,
    ShExJ,
    Turtle,
}

/// SHACL format
/// Currently, only Turtle, RDFXML, NTriples, TriG, N3 and
/// NQuads are supported
/// The default is Turtle
#[allow(clippy::upper_case_acronyms)]
#[pyclass(eq, eq_int, name = "ShaclFormat")]
#[derive(PartialEq)]
pub enum PyShaclFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

/// Defines how to format a ShEx schema
/// It can be configured to print or not terminal colors
/// The default is to print terminal colors
/// This is useful when printing to a terminal that supports colors
/// or when printing to a file that will be viewed in a terminal
/// that supports colors
/// The formatter can be configured to not print colors
/// when printing to a file that will be viewed in a text editor
/// that does not support colors
#[pyclass(frozen, name = "ShExFormatter")]
pub struct PyShExFormatter {
    inner: ShExFormatter,
}

#[pymethods]
impl PyShExFormatter {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.detach(|| Self {
            inner: ShExFormatter::default(),
        })
    }

    /// Returns a ShExFormatter that doesn't print terminal colors
    #[staticmethod]
    pub fn without_colors() -> Self {
        Self {
            inner: ShExFormatter::default().without_colors(),
        }
    }
}

/// Defines how to format a ShapeMap
/// It can be configured to print or not terminal colors
/// The default is to print terminal colors
/// This is useful when printing to a terminal that supports colors
/// or when printing to a file that will be viewed in a terminal
/// that supports colors
/// The formatter can be configured to not print colors
/// when printing to a file that will be viewed in a text editor
/// that does not support colors
/// The default is to print terminal colors
#[pyclass(frozen, name = "ShapeMapFormatter")]
pub struct PyShapeMapFormatter {
    inner: ShapeMapFormatter,
}

#[pymethods]
impl PyShapeMapFormatter {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.detach(|| Self {
            inner: ShapeMapFormatter::default(),
        })
    }

    /// Returns a Shapemap formatter that doesn't print terminal colors
    #[staticmethod]
    pub fn without_colors() -> Self {
        Self {
            inner: ShapeMapFormatter::default().without_colors(),
        }
    }
}

/// UML Generation Mode
/// It can be configured to generate UML for all nodes
/// or only for the neighbours of a given node
/// The default is to generate UML for all nodes
#[pyclass(name = "UmlGenerationMode")]
pub enum PyUmlGenerationMode {
    /// Generate UML for all nodes
    #[pyo3(name = "AllNodes")]
    PyAllNodes {},

    /// Generate UML only for the neighbours of a shape
    #[pyo3(constructor = (node), name ="Neighs")]
    PyNeighs { node: String },
}

#[pymethods]
impl PyUmlGenerationMode {
    #[new]
    pub fn __init__(py: Python<'_>) -> Self {
        py.detach(|| PyUmlGenerationMode::PyAllNodes {})
    }

    /// Show all nodes
    #[staticmethod]
    pub fn all() -> Self {
        PyUmlGenerationMode::PyAllNodes {}
    }

    /// Show only the neighbours of a given node
    #[staticmethod]
    pub fn neighs(node: &str) -> Self {
        PyUmlGenerationMode::PyNeighs {
            node: node.to_string(),
        }
    }
}

impl From<&PyUmlGenerationMode> for UmlGenerationMode {
    fn from(m: &PyUmlGenerationMode) -> UmlGenerationMode {
        match m {
            PyUmlGenerationMode::PyAllNodes {} => UmlGenerationMode::AllNodes,
            PyUmlGenerationMode::PyNeighs { node } => UmlGenerationMode::Neighs(node.to_string()),
        }
    }
}

impl From<UmlGenerationMode> for PyUmlGenerationMode {
    fn from(value: UmlGenerationMode) -> Self {
        match value {
            UmlGenerationMode::AllNodes => PyUmlGenerationMode::PyAllNodes {},
            UmlGenerationMode::Neighs(node) => PyUmlGenerationMode::PyNeighs { node },
        }
    }
}

/// MIE representation
#[pyclass(name = "Mie")]
pub struct PyMie {
    inner: Mie,
}

#[pymethods]
impl PyMie {
    /// Returns a string representation of the schema
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    /// Converts the MIE spec to JSON
    pub fn as_json(&self) -> PyResult<String> {
        let str = self
            .inner
            .to_json()
            .map_err(|e| PyRudofError::str(e.to_string()))?;
        Ok(str)
    }

    pub fn as_yaml(&self) -> PyResult<String> {
        let yaml = self.inner.to_yaml_str();
        Ok(yaml)
    }
}

/// ShEx Schema representation
/// It can be converted to JSON
/// It can be serialized to different formats
/// It can be printed with or without terminal colors
/// The default is to print with terminal colors
/// The formatter can be configured to not print colors
/// when printing to a file that will be viewed in a text editor
/// that does not support colors
#[pyclass(name = "ShExSchema")]
pub struct PyShExSchema {
    inner: ShExSchema,
}

#[pymethods]
impl PyShExSchema {
    /// Returns a string representation of the schema
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    /*     /// Converts the schema to JSON
    pub fn as_json(&self) -> PyResult<String> {
        let str =  self
            .inner
            .as_json()
            .map_err(|e| PyRudofError::str(e.to_string()))?;
        Ok(str)
    } */
}

/// Service Description representation
/// This is based on [SPARQL Service Description](https://www.w3.org/TR/sparql11-service-description/) + [VoID](https://www.w3.org/TR/void/) vocabulary
#[pyclass(name = "ServiceDescription")]
pub struct PyServiceDescription {
    inner: ServiceDescription,
}

#[pymethods]
impl PyServiceDescription {
    /// Returns a string representation of the schema
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn as_mie(&self) -> PyResult<PyMie> {
        let str = self.inner.service2mie();
        Ok(PyMie { inner: str })
    }

    /// Serialize the current Service Description
    /// The default format is Json
    #[pyo3(signature = (format = &PyServiceDescriptionFormat::Json))]
    pub fn serialize(&self, format: &PyServiceDescriptionFormat) -> PyResult<String> {
        let mut v = Vec::new();
        let service_description_format = cnv_service_description_format(format);
        self.inner
            .serialize(&service_description_format, &mut v)
            .map_err(|e| RudofError::SerializingServiceDescription {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        let str = String::from_utf8(v)
            .map_err(|e| RudofError::SerializingServiceDescription {
                error: format!("{e}"),
            })
            .map_err(cnv_err)?;
        Ok(str)
    }

    /*     /// Converts the schema to JSON
    pub fn as_json(&self) -> PyResult<String> {
        let str =  self
            .inner
            .as_json()
            .map_err(|e| PyRudofError::str(e.to_string()))?;
        Ok(str)
    } */
}

/// [DCTAP](https://www.dublincore.org/specifications/dctap/) representation
#[pyclass(name = "DCTAP")]
pub struct PyDCTAP {
    inner: DCTAP,
}

#[pymethods]
impl PyDCTAP {
    /// Returns a string representation of the DCTAP
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    /// Returns a string representation of the DCTAP
    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }
}

/// ShapeMap used for querying and validation
/// It can be converted to JSON
#[pyclass(name = "QueryShapeMap")]
pub struct PyQueryShapeMap {
    inner: QueryShapeMap,
}

#[pymethods]
impl PyQueryShapeMap {
    /// Returns a string representation of the shape map
    fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    /*pub fn as_json(&self) -> PyResult<String> {
        let str = self
            .inner
            .as_json()
            .map_err(|e| PyRudofError::str(e.to_string()))?;
        Ok(str)
    }*/
}

/// Shapes Comparator result
/// It contains the differences between two schemas
/// It can be converted to JSON
#[pyclass(name = "ShaCo")]
pub struct PyShaCo {
    inner: ShaCo,
}

#[pymethods]
impl PyShaCo {
    /// Returns a string representation of the schema comparison result
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    /// Converts the schema comparison result to JSON
    pub fn as_json(&self) -> PyResult<String> {
        let str = self
            .inner
            .as_json()
            .map_err(|e| PyRudofError::str(e.to_string()))?;
        Ok(str)
    }
}

/// Common Shapes Model
/// This is a structure used to compare shapes
#[pyclass(name = "CoShaMo")]
pub struct PyCoShaMo {
    inner: CoShaMo,
}

#[pymethods]
impl PyCoShaMo {
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }
}

/// Format of schema to compare, e.g. shexc, turtle, ...
#[pyclass(name = "CompareSchemaFormat")]
pub struct PyCompareSchemaFormat {
    inner: CompareSchemaFormat,
}

#[pymethods]
impl PyCompareSchemaFormat {
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    /// Returns a CompareSchemaFormat for ShExC
    #[staticmethod]
    pub fn shexc() -> Self {
        Self {
            inner: CompareSchemaFormat::ShExC,
        }
    }

    /// Returns a CompareSchemaFormat for Turtle
    #[staticmethod]
    pub fn turtle() -> Self {
        Self {
            inner: CompareSchemaFormat::Turtle,
        }
    }
}

/// Mode of schema to compare, e.g. shex, ...
#[pyclass(name = "CompareSchemaMode")]
pub struct PyCompareSchemaMode {
    inner: CompareSchemaMode,
}

#[pymethods]
impl PyCompareSchemaMode {
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    /// Returns a CompareSchemaMode for ShEx
    #[staticmethod]
    pub fn shex() -> Self {
        Self {
            inner: CompareSchemaMode::ShEx,
        }
    }
}

/// Intermediate Representation of a SHACL Schema
#[pyclass(name = "ShaclSchema")]
pub struct PyShaclSchema {
    inner: ShaclSchemaIR,
}

#[pymethods]
impl PyShaclSchema {
    pub fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }
}

/// SHACL validation mode
/// It can be native or SPARQL
/// The default is native
#[pyclass(eq, eq_int, name = "ShaclValidationMode")]
#[derive(PartialEq)]
pub enum PyShaclValidationMode {
    Native,
    Sparql,
}

/// Source of the shapes graph for SHACL validation
/// It can be the current RDF data or the current SHACL schema
/// The default is the current SHACL schema
#[pyclass(eq, eq_int, name = "ShapesGraphSource")]
#[derive(PartialEq)]
pub enum PyShapesGraphSource {
    CurrentData,
    CurrentSchema,
}

/// A single solution of a SPARQL query
/// It can be converted to a String
/// It can return the list of variables in this solution
/// It can return the value of a variable name if exists, None if it doesn't
#[pyclass(name = "QuerySolution")]
pub struct PyQuerySolution {
    inner: QuerySolution<RdfData>,
}

#[pymethods]
impl PyQuerySolution {
    /// Converts the solution to a String
    pub fn show(&self) -> String {
        self.inner.show().to_string()
    }

    /// Returns the list of variables in this solution
    pub fn variables(&self) -> Vec<String> {
        let vars: Vec<String> = self.inner.variables().map(|v| v.to_string()).collect();
        vars
    }

    /// Returns the value of a variable name if exists, None if it doesn't
    pub fn find(&self, var_name: &str) -> Option<String> {
        self.inner
            .find_solution(&VarName::new(var_name))
            .map(|t| format!("{t}"))
    }
}

/// A set of solutions of a SPARQL query
/// It can be converted to a String
/// It can return the number of solutions
/// It can be iterated to get each solution
/// It can be converted to a list of solutions
#[pyclass(name = "QuerySolutions")]
pub struct PyQuerySolutions {
    inner: QuerySolutions<RdfData>,
}

#[pymethods]
impl PyQuerySolutions {
    /// Converts the solutions to a String
    pub fn show(&self) -> String {
        format!("Solutions: {:?}", self.inner)
    }

    /// Converts the solutions to a JSON string
    pub fn as_json(&self) -> String {
        self.inner.as_json()
    }

    /// Returns the number of solutions
    pub fn count(&self) -> usize {
        self.inner.count()
    }

    /// Returns an iterator over the solutions
    /// This allows to iterate over the solutions in a for loop
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<QuerySolutionIter>> {
        let rs: Vec<PyQuerySolution> = slf
            .inner
            .iter()
            .map(|qs| PyQuerySolution { inner: qs.clone() })
            .collect();
        let iter = QuerySolutionIter {
            inner: rs.into_iter(),
        };
        Py::new(slf.py(), iter)
    }
}

/// Iterator over the solutions of a SPARQL query
#[pyclass]
struct QuerySolutionIter {
    inner: std::vec::IntoIter<PyQuerySolution>,
}

#[pymethods]
impl QuerySolutionIter {
    /// Returns the iterator itself
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    /// Returns the next solution in the iterator
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyQuerySolution> {
        slf.inner.next()
    }
}

/// Result of a ShEx validation
/// It can be converted to a String
#[pyclass(frozen, name = "ResultShapeMap")]
pub struct PyResultShapeMap {
    inner: ResultShapeMap,
}

#[pymethods]
impl PyResultShapeMap {
    /// Convert a ResultShapeMap to a String
    pub fn show(&self) -> String {
        let result = &self.inner;
        format!("{result}")
    }
}

/// Result of a SHACL validation
/// It can be converted to a String
/// It can return if the data conforms to the shapes
#[pyclass(frozen, name = "ValidationReport")]
pub struct PyValidationReport {
    inner: ValidationReport,
}

#[pymethods]
impl PyValidationReport {
    /// Convert ValidationReport to a String
    pub fn show(&self) -> String {
        let result = &self.inner;
        format!("{result}")
    }

    /// Returns true if there were no violation errors
    pub fn conforms(&self) -> bool {
        self.inner.conforms()
    }
}

/// Status of a validation
/// It can be converted to a String
#[pyclass(frozen, name = "ValidationStatus")]
pub struct PyValidationStatus {
    inner: ValidationStatus,
}

#[pymethods]
impl PyValidationStatus {
    /// Convert ValidationStatus to a String
    pub fn show(&self) -> String {
        let result = &self.inner;
        format!("{result}")
    }
}

/// RudofError is the error type used in the Rudof library
/// It can be converted to a Python exception
#[pyclass(name = "RudofError")]
pub struct PyRudofError {
    error: RudofError,
}

impl PyRudofError {
    fn str(msg: String) -> Self {
        Self {
            error: RudofError::Generic { error: msg },
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
        Self { error }
    }
}

fn cnv_err(e: RudofError) -> PyErr {
    println!("RudofError: {e}");
    let e: PyRudofError = e.into();
    let e: PyErr = e.into();
    e
}

fn cnv_comparator_err(e: ComparatorError) -> PyErr {
    println!("ComparatorError: {e}");
    let e: PyRudofError = PyRudofError::str(format!("{}", e));
    let e: PyErr = e.into();
    e
}

fn cnv_dctap_format(format: &PyDCTapFormat) -> DCTAPFormat {
    match format {
        PyDCTapFormat::CSV => DCTAPFormat::CSV,
        PyDCTapFormat::XLSX => DCTAPFormat::XLSX,
    }
}

fn cnv_reader_mode(format: &PyReaderMode) -> ReaderMode {
    match format {
        PyReaderMode::Lax => ReaderMode::Lax,
        PyReaderMode::Strict => ReaderMode::Strict,
    }
}

fn cnv_service_description_format(format: &PyServiceDescriptionFormat) -> ServiceDescriptionFormat {
    match format {
        PyServiceDescriptionFormat::Internal => ServiceDescriptionFormat::Internal,
        PyServiceDescriptionFormat::Mie => ServiceDescriptionFormat::Mie,
        PyServiceDescriptionFormat::Json => ServiceDescriptionFormat::Json,
    }
}

fn cnv_rdf_format(format: &PyRDFFormat) -> RDFFormat {
    match format {
        PyRDFFormat::Turtle => RDFFormat::Turtle,
        PyRDFFormat::NTriples => RDFFormat::NTriples,
        PyRDFFormat::RDFXML => RDFFormat::RDFXML,
        PyRDFFormat::TriG => RDFFormat::TriG,
        PyRDFFormat::N3 => RDFFormat::N3,
        PyRDFFormat::NQuads => RDFFormat::NQuads,
        PyRDFFormat::JsonLd => RDFFormat::JsonLd,
    }
}

fn cnv_shapemap_format(format: &PyShapeMapFormat) -> ShapeMapFormat {
    match format {
        PyShapeMapFormat::Compact => ShapeMapFormat::Compact,
        PyShapeMapFormat::JSON => ShapeMapFormat::JSON,
    }
}

fn cnv_shex_format(format: &PyShExFormat) -> ShExFormat {
    match format {
        PyShExFormat::ShExC => ShExFormat::ShExC,
        PyShExFormat::ShExJ => ShExFormat::ShExJ,
        PyShExFormat::Turtle => ShExFormat::Turtle,
    }
}

fn cnv_shacl_format(format: &PyShaclFormat) -> ShaclFormat {
    match format {
        PyShaclFormat::Turtle => ShaclFormat::Turtle,
        PyShaclFormat::NTriples => ShaclFormat::NTriples,
        PyShaclFormat::RDFXML => ShaclFormat::RDFXML,
        PyShaclFormat::TriG => ShaclFormat::TriG,
        PyShaclFormat::N3 => ShaclFormat::N3,
        PyShaclFormat::NQuads => ShaclFormat::NQuads,
    }
}

fn cnv_shacl_validation_mode(mode: &PyShaclValidationMode) -> ShaclValidationMode {
    match mode {
        PyShaclValidationMode::Native => ShaclValidationMode::Native,
        PyShaclValidationMode::Sparql => ShaclValidationMode::Sparql,
    }
}

fn cnv_shapes_graph_source(sgs: &PyShapesGraphSource) -> ShapesGraphSource {
    match sgs {
        PyShapesGraphSource::CurrentData => ShapesGraphSource::CurrentData,
        PyShapesGraphSource::CurrentSchema => ShapesGraphSource::CurrentSchema,
    }
}

fn cnv_query_result_format(format: &PyQueryResultFormat) -> QueryResultFormat {
    match format {
        PyQueryResultFormat::Turtle => QueryResultFormat::Turtle,
        PyQueryResultFormat::NTriples => QueryResultFormat::NTriples,
        PyQueryResultFormat::RDFXML => QueryResultFormat::RdfXml,
        PyQueryResultFormat::CSV => QueryResultFormat::Csv,
        PyQueryResultFormat::TriG => QueryResultFormat::TriG,
        PyQueryResultFormat::N3 => QueryResultFormat::N3,
        PyQueryResultFormat::NQuads => QueryResultFormat::NQuads,
    }
}

fn get_path_reader(path_name: &str, context: &str) -> PyResult<BufReader<File>> {
    let path = Path::new(path_name);
    let file = File::open::<&OsStr>(path.as_ref())
        .map_err(|e| RudofError::ReadingPathContext {
            path: path_name.to_string(),
            context: context.to_string(),
            error: format!("{e}"),
        })
        .map_err(cnv_err)?;
    let reader = BufReader::new(file);
    Ok(reader)
}

fn get_url_reader(url: &str, accept: Option<&str>, context: &str) -> PyResult<InputSpecReader> {
    let url_spec = UrlSpec::parse(url)
        .map_err(|e| RudofError::ParsingUrlContext {
            url: url.to_string(),
            context: context.to_string(),
            error: e.to_string(),
        })
        .map_err(cnv_err)?;
    let input_spec = InputSpec::Url(url_spec);
    let reader = input_spec
        .open_read(accept, context)
        .map_err(|e| RudofError::ReadingUrlContext {
            url: url.to_string(),
            context: context.to_string(),
            error: e.to_string(),
        })
        .map_err(cnv_err)?;
    Ok(reader)
}

fn get_reader(input: &str, accept: Option<&str>, context: &str) -> PyResult<InputSpecReader> {
    let input_spec: InputSpec = FromStr::from_str(input)
        .map_err(|e: InputSpecError| RudofError::ParsingInputSpecContext {
            input: input.to_string(),
            context: context.to_string(),
            error: e.to_string(),
        })
        .map_err(cnv_err)?;
    let reader = input_spec
        .open_read(accept, context)
        .map_err(|e| RudofError::ReadingInputSpecContext {
            input: input.to_string(),
            context: context.to_string(),
            error: e.to_string(),
        })
        .map_err(cnv_err)?;
    Ok(reader)
}
