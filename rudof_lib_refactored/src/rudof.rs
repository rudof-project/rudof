use sparql_service::RdfData;
use pgschema::{validation_result::ValidationResult, pgs::PropertyGraphSchema};
use shacl_ast::ast::ShaclSchema;
use shacl_ir::compiled::schema_ir::SchemaIR as ShaclSchemaIR;
use shacl_validation::validation_report::report::ValidationReport;
use sparql_service::ServiceDescription;
use shex_ast::Schema as ShExSchema;
use shex_ast::ir::schema_ir::SchemaIR as ShExSchemaIR;
use shex_ast::shapemap::{QueryShapeMap, ResultShapeMap};
use dctap::DCTap as DCTAP;
use rudof_rdf::rdf_core::query::SparqlQuery;
use rdf_config::RdfConfigModel;
use crate::{
    RudofConfig, 
    errors::RudofError,
    formats::{
        DataFormat, InputSpec, DataReaderMode, ShapeMapFormat,
        ShExFormat, ShExValidationSortByMode, ShaclFormat, ShaclValidationMode,
        ShaclValidationSortByMode, QueryType, ResultQueryFormat, ResultServiceFormat,
        DCTapFormat, ResultDCTapFormat, RdfConfigFormat, ResultRdfConfigFormat,
        ResultPgSchemaValidationFormat, NodeInspectionMode, ComparisonFormat, ComparisonMode,
        ResultComparisonFormat, ConversionFormat, ConversionMode, ResultConversionFormat, 
        ResultConversionMode,
    },
    types::{Data, QueryResult},
};
use std::io;

pub type Result<T> = std::result::Result<T, RudofError>;

// ============================================================================
// TRAIT DEFINITIONS
// ============================================================================
// These traits define the public API surface of Rudof, organized by
// functionality. Each trait groups related operations together.

/// Core operations for Rudof initialization and configuration.
pub(crate) trait RudofCore: Sized {
    /// Creates a new Rudof instance with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration to use for this instance
    ///
    /// # Errors
    ///
    /// Returns an error if the RDF data cannot be initialized with the given configuration.
    fn new(config: &RudofConfig) -> Result<Self>;

    /// Returns the version string of Rudof.
    ///
    /// # Returns
    ///
    /// A string slice containing the version number.
    fn version(&self) -> &str;

    /// Returns a reference to the current configuration.
    ///
    /// # Returns
    ///
    /// A reference to the `RudofConfig` instance.
    fn config(&self) -> &RudofConfig;

    /// Updates the configuration of this Rudof instance.
    ///
    /// # Arguments
    ///
    /// * `config` - The new configuration to apply
    fn update_config(&mut self, config: &RudofConfig);

    /// Resets all state in this Rudof instance.
    ///
    /// This clears all loaded data, schemas, queries, and validation results,
    /// returning the instance to a clean state.
    fn reset_all(&mut self);
}

/// Operations for managing RDF data.
pub(crate) trait DataOperations {
    /// Loads RDF data from one or more input sources.
    ///
    /// # Arguments
    ///
    /// * `data` - Array of input specifications defining data sources
    /// * `data_format` - The RDF format of the input data (uses default if None)
    /// * `base` - Optional base IRI for resolving relative IRIs (uses default if None)
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
        format: Option<&DataFormat>, 
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
        predicates: Option<&[&str]>,
        show_node_mode: Option<&NodeInspectionMode>,
        depth: Option<usize>,
        show_hyperlinks: Option<bool>,
        writer: &mut W,
    ) -> Result<()>;
}

/// Operations for ShEx (Shape Expressions) schema validation.
pub(crate) trait ShExOperations {
    /// Loads a ShEx schema from an input specification.
    ///
    /// # Arguments
    ///
    /// * `schema` - Input specification defining the schema source
    /// * `schema_format` - Optional ShEx format (uses default if None)
    /// * `base_schema` - Optional base IRI for resolving relative IRIs in the schema (uses default if None)
    /// * `reader_mode` - The parsing mode (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be parsed or loaded.
    fn load_shex_schema(
        &mut self,
        schema: &InputSpec,
        schema_format: &Option<ShExFormat>,
        base_schema: &Option<&str>,
        reader_mode: &Option<DataReaderMode>,
    ) -> Result<()>;

    /// Serializes the current ShEx schema to a writer.
    ///
    /// # Arguments
    ///
    /// * `shape_label` - Optional specific shape label to serialize (serializes entire schema if None)
    /// * `show_schema` - Whether to include the schema in the output (true by default)
    /// * `show_statistics` - Whether to include statistics in the output (false by default)
    /// * `show_dependencies` - Whether to show shape dependencies (false by default)
    /// * `show_time` - Whether to include timing information (false by default)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no schema is loaded or serialization fails.
    fn serialize_shex_schema<W: io::Write>(
        &self,
        shape_label: Option<&str>,
        show_schema: Option<bool>,
        show_statistics: Option<bool>,
        show_dependencies: Option<bool>,
        show_time: Option<bool>,
        writer: &mut W
    ) -> Result<()>;

    /// Resets the ShEx schema.
    fn reset_shex_schema(&mut self);

    /// Loads a shape map from an input specification.
    ///
    /// # Arguments
    ///
    /// * `shapemap` - Input specification defining the shape map source
    /// * `shapemap_format` - Optional shape map format (uses default if None)
    /// * `base_nodes` - Optional base IRI for resolving node IRIs (uses default if None)
    /// * `base_shapes` - Optional base IRI for resolving shape IRIs (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the shape map cannot be parsed or loaded.
    fn load_shapemap(
        &mut self,
        shapemap: &InputSpec,
        shapemap_format: Option<&ShapeMapFormat>,
        base_nodes: &Option<&str>,
        base_shapes: &Option<&str>,
    ) -> Result<()>;

    /// Serializes the current shape map to a writer.
    ///
    /// # Arguments
    ///
    /// * `shapemap_format` - Optional output format (uses default if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no shape map is loaded or serialization fails.
    fn serialize_shapemap<W: io::Write>(
        &self,
        shapemap_format: Option<&ShapeMapFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current shape map.
    fn reset_shapemap(&mut self);

    /// Validates the current RDF data using the loaded ShEx schema and shape map.
    ///
    /// # Errors
    ///
    /// Returns an error if no schema or shape map is loaded.
    fn validate_shex(&mut self) -> Result<()>;

    /// Serializes the ShEx validation results to a writer.
    ///
    /// # Arguments
    ///
    /// * `sort_order` - Optional sorting mode for the validation results (uses default order if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available.
    fn serialize_shex_validation_results<W: io::Write>(
        &self,
        sort_order: Option<&ShExValidationSortByMode>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the shex validation.
    fn reset_shex(&mut self);
}

/// Operations for SHACL (Shapes Constraint Language) validation.
pub(crate) trait ShaclOperations {
    /// Loads a SHACL schema from an input specification.
    ///
    /// # Arguments
    ///
    /// * `schema` - Input specification defining the schema source
    /// * `schema_format` - Optional SHACL format (uses default if None)
    /// * `base` - Optional base IRI for resolving relative IRIs (uses default if None)
    /// * `reader_mode` - The parsing mode (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be parsed or loaded.
    fn load_shacl_schema(
        &mut self,
        schema: &InputSpec,
        schema_format: &Option<ShaclFormat>,
        base: &Option<&str>,
        reader_mode: &Option<DataReaderMode>,
    ) -> Result<()>;

    /// Serializes the current SHACL schema to a writer.
    ///
    /// # Arguments
    ///
    /// * `format` - Optional output format (uses default if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no schema is loaded or serialization fails.
    fn serialize_shacl_schema<W: io::Write>(
        &self, 
        format: Option<&ShaclFormat>, 
        writer: &mut W
    ) -> Result<()>;

    /// Resets the SHACL schema.
    fn reset_shacl_schema(&mut self);

    /// Loads SHACL shapes from an input specification.
    ///
    /// # Arguments
    ///
    /// * `shapes` - Input specification defining the shapes source
    /// * `format` - Optional SHACL format (uses default if None)
    /// * `base` - Optional base IRI for resolving relative IRIs (uses default if None)
    /// * `reader_mode` - The parsing mode (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the shapes cannot be parsed or loaded.
    fn load_shapes(
        &mut self,
        shapes: &InputSpec,
        format: Option<&ShaclFormat>,
        base: &Option<&str>,
        reader_mode: &Option<DataReaderMode>,
    ) -> Result<()>;

    /// Serializes the current SHACL shapes to a writer.
    ///
    /// # Arguments
    ///
    /// * `format` - Optional output format (uses default if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no shapes are loaded or serialization fails.
    fn serialize_shapes<W: io::Write>(
        &self,
        format: Option<&ShaclFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current SHACL shapes.
    fn reset_shapes(&mut self);

    /// Validates the current RDF data using the loaded SHACL schema and shapes.
    ///
    /// If no shapes are explicitly loaded, the validation assumes that the shapes
    /// are defined within the SHACL schema itself.
    ///
    /// # Arguments
    ///
    /// * `mode` - Optional validation mode (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if no SHACL schema or shapes is loaded.
    fn validate_shacl(
        &mut self, 
        mode: Option<&ShaclValidationMode>,
    ) -> Result<()>;

    /// Serializes the SHACL validation results to a writer.
    ///
    /// # Arguments
    ///
    /// * `sort_order` - Optional sorting mode for the validation results (uses default order if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available.
    fn serialize_shacl_validation_results<W: io::Write>(
        &self,
        sort_order: Option<&ShaclValidationSortByMode>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the SHACL validation.
    fn reset_shacl_validation(&mut self);
}

/// Operations for executing SPARQL queries.
pub(crate) trait QueryOperations {

    /// Loads a SPARQL query from an input specification.
    ///
    /// # Arguments
    ///
    /// * `query` - Input specification defining the query source
    /// * `query_type` - The type of query (SELECT, CONSTRUCT, etc.)
    ///
    /// # Errors
    ///
    /// Returns an error if the query cannot be parsed or loaded.
    fn load_query(
        &mut self,
        query: &InputSpec,
        query_type: &QueryType,
    ) -> Result<()>;

    /// Serializes the currently loaded query to a writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The destination to write the serialized query to
    ///
    /// # Errors
    ///
    /// Returns an error if no query is loaded or serialization fails.
    fn serialize_query<W: io::Write>(&self, writer: &mut W) -> Result<()>;

    /// Resets the current query.
    fn reset_query(&mut self);

    /// Executes the currently loaded query.
    ///
    /// If `endpoint` is not specified, the query is executed against the loaded RDF data.
    /// If `endpoint` is specified, the query is executed against that SPARQL endpoint.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - Optional name or URL of the SPARQL endpoint to query
    ///
    /// # Errors
    ///
    /// Returns an error if no query is loaded, or if query execution fails.
    fn run_query(&mut self, endpoint: Option<&str>) -> Result<()>;

    /// Serializes the query results to a writer.
    ///
    /// # Arguments
    ///
    /// * `result_format` - Optional output format for the results (uses default if None)
    /// * `writer` - The destination to write the serialized results to
    ///
    /// # Errors
    ///
    /// Returns an error if no query results are available or serialization fails.
    fn serialize_query_results<W: io::Write>(
        &self, 
        result_format: Option<&ResultQueryFormat>, 
        writer: &mut W
    ) -> Result<()>;

    /// Resets the current query and results.
    fn reset_query_results(&mut self);
}

/// Comparison operations
pub(crate) trait ComparisonOperations {
    /// Compares two schemas and returns their differences in the requested format.
    ///
    /// # Arguments
    ///
    /// * `schema1` - Input specification defining the first schema source
    /// * `schema2` - Input specification defining the second schema source
    /// * `base1` - Optional base IRI for resolving relative IRIs in the first schema (uses default if None)
    /// * `base2` - Optional base IRI for resolving relative IRIs in the second schema (uses default if None)
    /// * `reader_mode` - Optional parsing mode used to read both schemas (uses default if None)
    /// * `format1` - Format of the first schema
    /// * `format2` - Format of the second schema
    /// * `mode1` - Comparison mode applied to the first schema
    /// * `mode2` - Comparison mode applied to the second schema
    /// * `shape1` - Optional shape identifier to focus the comparison in the first schema
    /// * `shape2` - Optional shape identifier to focus the comparison in the second schema
    /// * `show_time` - Whether to include timing information in the comparison output (false by default)
    /// * `result_format` - Optional output format for comparison results (uses default if None)
    /// * `writer` - The destination to write the serialized results to
    ///
    /// # Errors
    ///
    /// Returns an error if schemas cannot be loaded, compared, or serialized.
    fn show_schema_comparison<W: io::Write>(
        &self,
        schema1: &InputSpec,
        schema2: &InputSpec,
        base1: Option<&str>,
        base2: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
        format1: &ComparisonFormat,
        format2: &ComparisonFormat,
        mode1: &ComparisonMode,
        mode2: &ComparisonMode,
        shape1: Option<&str>,
        shape2: Option<&str>,
        show_time: Option<bool>,
        result_format: Option<&ResultComparisonFormat>,
        writer: &mut W,
    ) -> Result<()>;
}

/// Conversion operations
pub(crate) trait ConversionOperations {
    /// Converts a schema from one format to another.
    ///
    /// # Arguments
    ///
    /// * `schema` - Input specification defining the schema source
    /// * `base` - Optional base IRI for resolving relative IRIs (uses default if None)
    /// * `reader_mode` - Optional parsing mode used to read the schema (uses default if None)
    /// * `input_mode` - The conversion mode for interpreting the input schema
    /// * `output_mode` - The conversion mode for generating the output schema
    /// * `input_format` - Format of the input schema
    /// * `output_format` - Format of the output schema
    /// * `shape` - Optional shape identifier to focus the conversion on a specific shape
    /// * `show_time` - Whether to include timing information in the conversion output (false by default)
    /// * `writer` - The destination to write the converted schema to
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be loaded, converted, or serialized.
    fn show_schema_conversion<W: io::Write>(
        &self,
        schema: &InputSpec,
        base: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
        input_mode: &ConversionMode,
        output_mode: &ResultConversionMode,
        input_format: &ConversionFormat,
        output_format: &ResultConversionFormat,
        shape: Option<&str>,
        show_time: Option<bool>,
        writer: &mut W,
    ) -> Result<()>;
}

/// Operations for DC-TAP (Dublin Core Tabular Application Profiles).
pub(crate) trait DctapOperations {
    /// Loads a DC-TAP profile from an input specification.
    ///
    /// # Arguments
    ///
    /// * `dctap` - Input specification defining the DC-TAP source
    /// * `format` - Optional DC-TAP format (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the DC-TAP profile cannot be parsed or loaded.
    fn load_dctap(
        &mut self,
        dctap: &InputSpec,
        format: Option<&DCTapFormat>,
    ) -> Result<()>;

    /// Serializes the current DC-TAP profile to a writer.
    ///
    /// # Arguments
    ///
    /// * `format` - Optional output format for the DC-TAP profile (uses default if None)
    /// * `writer` - The destination to write the serialized DC-TAP profile to
    ///
    /// # Errors
    ///
    /// Returns an error if no DC-TAP profile is loaded or serialization fails.
    fn serialize_dctap<W: io::Write>(
        &self,
        format: Option<&ResultDCTapFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current dctap.
    fn reset_dctap(&mut self);
}

/// Operations for RDF-config.
///
/// RDF-config is a tool to generate SPARQL queries, schema diagrams, and files
/// required for Grasp, TogoStanza and ShEx validator from simple YAML-based
/// configuration files.
pub(crate) trait RdfConfigOperations {
    /// Loads an RDF-config specification from an input source.
    ///
    /// # Arguments
    ///
    /// * `rdf_config` - Input specification defining the RDF-config source
    /// * `format` - Optional RDF-config format (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the RDF-config specification cannot be parsed or loaded.
    fn load_rdf_config(
        &mut self,
        rdf_config: &InputSpec,
        format: Option<&RdfConfigFormat>,
    ) -> Result<()>;

    /// Serializes the current RDF-config model to a writer.
    ///
    /// # Arguments
    ///
    /// * `format` - Optional output format for the RDF-config model (uses default if None)
    /// * `writer` - The destination to write the serialized RDF-config model to
    ///
    /// # Errors
    ///
    /// Returns an error if no RDF-config model is loaded or serialization fails.
    fn serialize_rdf_config<W: io::Write>(
        &self,
        format: Option<&ResultRdfConfigFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current RDF-config model.
    fn reset_rdf_config(&mut self);
}

/// Operations for Property Graph schema management and validation.
pub(crate) trait PgSchemaOperations {
    /// Loads a Property Graph schema from an input specification.
    ///
    /// # Arguments
    ///
    /// * `pg_schema` - Input specification defining the Property Graph schema source
    ///
    /// # Errors
    ///
    /// Returns an error if the Property Graph schema cannot be parsed or loaded.
    fn load_pg_schema(
        &mut self,
        pg_schema: &InputSpec,
    ) -> Result<()>;

    /// Serializes the current Property Graph schema to a writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The destination to write the serialized Property Graph schema to
    ///
    /// # Errors
    ///
    /// Returns an error if no Property Graph schema is loaded or serialization fails.
    fn serialize_pg_schema<W: io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current Property Graph schema.
    fn reset_pg_schema(&mut self);

    /// Runs validation on the Property Graph schema using the loaded pgschema and shape map.
    ///
    /// # Errors
    ///
    /// Returns an error if no pgschema or shapemap is loaded.
    fn run_pgschema_validation(&mut self) -> Result<()>;

    /// Serializes the Property Graph schema validation results to a writer.
    ///
    /// # Arguments
    ///
    /// * `result_format` - Optional output format for the validation results (uses default if None)
    /// * `writer` - The destination to write the serialized validation results to
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available or serialization fails.
    fn serialize_pgschema_validation_results<W: io::Write>(
        &self,
        result_format: Option<&ResultPgSchemaValidationFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the Property Graph schema validation.
    fn reset_pg_schema_validation(&mut self);
}

/// This represents the public API to interact with `rudof`
#[derive(Debug)]
pub struct Rudof {
    /// Version of Rudof
    version: String,

    /// Current configuration
    config: RudofConfig,

    /// Current Data
    data: Option<Data>,

    /// Current SHACL Schema
    shacl_schema: Option<ShaclSchema<RdfData>>,

    /// Current SHACL Schema Internal Representation
    shacl_schema_ir: Option<ShaclSchemaIR>,

    /// Current SHACL validation results
    shacl_validation_results: Option<ValidationReport>,

    /// Current ShEx Schema
    shex_schema: Option<ShExSchema>,

    /// ShEx Schema Internal Representation
    shex_schema_ir: Option<ShExSchemaIR>,

    /// Current ShEx validation results
    shex_validation_results: Option<ResultShapeMap>,

    /// Current PGSchema
    pg_schema: Option<PropertyGraphSchema>,

    /// Current PGSchema validation results
    pg_schema_validation_results: Option<ValidationResult>,

    /// Current Shape Map
    shapemap: Option<QueryShapeMap>,

    /// Current SPARQL Query
    sparql_query: Option<SparqlQuery>,

    /// Current query results
    query_results: Option<QueryResult>,

    /// Current DCTAP
    dctap: Option<DCTAP>,

    /// Current Service Description
    service_description: Option<ServiceDescription>,

    /// Current rdf_config model
    rdf_config: Option<RdfConfigModel>,
}