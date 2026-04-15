use crate::{
    RudofConfig,
    api::{
        comparison::builders::ShowSchemaComparisonBuilder,
        conversion::builders::ShowSchemaConversionBuilder,
        core::{
            CoreOperations,
            builders::{ConfigBuilder, ResetAllBuilder, UpdateConfigBuilder, VersionBuilder},
        },
        data::builders::{
            ListEndpointsBuilder, LoadDataBuilder, LoadServiceDescriptionBuilder, ResetDataBuilder,
            ResetServiceDescriptionBuilder, SerializeDataBuilder, SerializeServiceDescriptionBuilder,
            ShowNodeInfoBuilder,
        },
        dctap::builders::{LoadDctapBuilder, ResetDctapBuilder, SerializeDctapBuilder},
        generation::builders::GenerateDataBuilder,
        map_state::builders::SerializeMapStateBuilder,
        pgschema::builders::{
            LoadPgSchemaBuilder, LoadTypemapBuilder, PgSchemaValidationBuilder, ResetPgSchemaBuilder,
            ResetPgSchemaValidationBuilder, ResetTypemapBuilder, SerializePgSchemaBuilder,
            SerializePgSchemaValidationResultsBuilder,
        },
        query::builders::{
            LoadQueryBuilder, ResetQueryBuilder, ResetQueryResultsBuilder, RunQueryBuilder, SerializeQueryBuilder,
            SerializeQueryResultsBuilder,
        },
        rdf_config::builders::{LoadRdfConfigBuilder, ResetRdfConfigBuilder, SerializeRdfConfigBuilder},
        shacl::builders::{
            LoadShaclShapesBuilder, ResetShaclBuilder, ResetShaclShapesBuilder, SerializeShaclShapesBuilder,
            SerializeShaclValidationResultsBuilder, ValidateShaclBuilder,
        },
        shex::builders::{
            AddNodeShapeToShapemapBuilder, CheckShexSchemaBuilder, LoadShapemapBuilder, LoadShexSchemaBuilder,
            ResetShapemapBuilder, ResetShexBuilder, ResetShexSchemaBuilder, SerializeShapemapBuilder,
            SerializeShexSchemaBuilder, SerializeShexValidationResultsBuilder, ValidateShexBuilder,
        },
    },
    errors::RudofError,
    formats::{
        ComparisonFormat, ComparisonMode, ConversionFormat, ConversionMode, GenerationSchemaFormat, InputSpec,
        ResultConversionFormat, ResultConversionMode,
    },
    types::{Data, QueryResult},
};
use dctap::DCTap as DCTAP;
use pgschema::{pgs::PropertyGraphSchema, type_map::TypeMap, validation_result::ValidationResult};
use rdf_config::RdfConfigModel;
use rudof_rdf::rdf_core::query::SparqlQuery;
use shacl_ast::ast::ShaclSchema;
use shacl_ir::compiled::schema_ir::SchemaIR as ShaclSchemaIR;
use shacl_validation::validation_report::report::ValidationReport;
use shex_ast::ir::schema_ir::SchemaIR as ShExSchemaIR;
use shex_ast::shapemap::{QueryShapeMap, ResultShapeMap};
use shex_ast::{Schema as ShExSchema, ir::map_state::MapState};
use shex_validation::Validator as ShExValidator;
use sparql_service::RdfData;
use sparql_service::ServiceDescription;
use std::io;

/// Typedef for `Result` returned by Rudof operations, where errors are boxed into `RudofError`.
/// Allows easier error handling across library-specific subsystems.
pub type Result<T> = std::result::Result<T, RudofError>;

/// The central `Rudof` struct acts as the main context and state machine.
///
/// It encapsulates everything needed for operations, holding references to currently loaded data, schemas
/// and processing results.
#[derive(Debug)]
pub struct Rudof {
    /// Version of Rudof
    pub(crate) version: String,

    /// Current configuration
    pub(crate) config: RudofConfig,

    /// Current Data
    pub(crate) data: Option<Data>,

    /// Current SHACL Shapes
    pub(crate) shacl_shapes: Option<ShaclSchema<RdfData>>,

    /// Current SHACL Schema Internal Representation
    pub(crate) shacl_shapes_ir: Option<ShaclSchemaIR>,

    /// Current SHACL validation results
    pub(crate) shacl_validation_results: Option<ValidationReport>,

    /// Current ShEx Schema
    pub(crate) shex_schema: Option<ShExSchema>,

    /// ShEx Schema Internal Representation
    pub(crate) shex_schema_ir: Option<ShExSchemaIR>,

    /// Current Shape Map
    pub(crate) shapemap: Option<QueryShapeMap>,

    /// Current ShEx validator. It holds the compiled schema and the validator which can be reused several times if needed
    pub(crate) shex_validator: Option<ShExValidator>,

    /// Current ShEx validation results
    pub(crate) shex_validation_results: Option<ResultShapeMap>,

    /// Current PGSchema
    pub(crate) pg_schema: Option<PropertyGraphSchema>,

    /// Current typemap
    pub(crate) typemap: Option<TypeMap>,

    /// Current PGSchema validation results
    pub(crate) pg_schema_validation_results: Option<ValidationResult>,

    /// Current SPARQL Query
    pub(crate) query: Option<SparqlQuery>,

    /// Current query results
    pub(crate) query_results: Option<QueryResult>,

    /// Current DCTAP
    pub(crate) dctap: Option<DCTAP>,

    /// Current Service Description
    pub(crate) service_description: Option<ServiceDescription>,

    /// Current rdf_config model
    pub(crate) rdf_config: Option<RdfConfigModel>,

    /// Current map state for ShEx validation used by Map Semantic Actions and materialize option
    pub(crate) map_state: Option<MapState>,
}

impl Rudof {
    // ========================================================================
    // RudofCore methods
    // ========================================================================

    /// Create a new `Rudof` instance from the provided `RudofConfig`.
    ///
    /// # Parameters
    /// - `config`: `Rudof` configuration settings (`RudofConfig`).
    pub fn new(config: RudofConfig) -> Self {
        <Self as CoreOperations>::new(config)
    }

    /// Returns a `VersionBuilder` for retrieving Rudof's version.
    pub fn version<'a>(&'a self) -> VersionBuilder<'a> {
        VersionBuilder::new(self)
    }

    /// Returns a `ConfigBuilder` that exposes the current `RudofConfig`.
    pub fn config<'a>(&'a self) -> ConfigBuilder<'a> {
        ConfigBuilder::new(self)
    }

    /// Returns an `UpdateConfigBuilder` to replace/update the current config.
    ///
    /// # Parameters
    /// - `config`: new configuration to replace the current one.
    pub fn update_config<'a>(&'a mut self, config: RudofConfig) -> UpdateConfigBuilder<'a> {
        UpdateConfigBuilder::new(self, config)
    }

    /// Returns a `ResetAllBuilder` that resets all runtime state in `Rudof`.
    pub fn reset_all<'a>(&'a mut self) -> ResetAllBuilder<'a> {
        ResetAllBuilder::new(self)
    }

    // ========================================================================
    // DataOperations methods
    // ========================================================================

    /// Returns a `LoadDataBuilder` to load RDF or PG data into `Rudof`'s state.
    pub fn load_data<'a>(&'a mut self) -> LoadDataBuilder<'a> {
        LoadDataBuilder::new(self)
    }

    /// Returns a `SerializeDataBuilder` that writes the currently-loaded data to the given `writer` (any `io::Write`).
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized data (e.g., file, stdout, in-memory buffer).
    pub fn serialize_data<'a, W: io::Write>(&'a mut self, writer: &'a mut W) -> SerializeDataBuilder<'a, W> {
        SerializeDataBuilder::new(self, writer)
    }

    pub fn serialize_map_state<'a, W: io::Write>(&'a mut self, writer: &'a mut W) -> SerializeMapStateBuilder<'a, W> {
        SerializeMapStateBuilder::new(self, writer)
    }

    /// Returns a `ResetDataBuilder` to clear loaded data from `Rudof`.
    pub fn reset_data<'a>(&'a mut self) -> ResetDataBuilder<'a> {
        ResetDataBuilder::new(self)
    }

    /// Returns a `LoadServiceDescriptionBuilder` to load a service description described by `service` (`InputSpec`).
    ///
    /// # Parameters
    /// - `service`: input specification for the service description.
    pub fn load_service_description<'a>(&'a mut self, service: &'a InputSpec) -> LoadServiceDescriptionBuilder<'a> {
        LoadServiceDescriptionBuilder::new(self, service)
    }

    /// Returns a `SerializeServiceDescriptionBuilder` to write the currentservice description to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized service description.
    pub fn serialize_service_description<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeServiceDescriptionBuilder<'a, W> {
        SerializeServiceDescriptionBuilder::new(self, writer)
    }

    /// Returns a `ResetServiceDescriptionBuilder` to clear any loaded service description from the internal state.
    pub fn reset_service_description<'a>(&'a mut self) -> ResetServiceDescriptionBuilder<'a> {
        ResetServiceDescriptionBuilder::new(self)
    }

    /// Returns a `ShowNodeInfoBuilder` that writes structural inspection information
    /// about the given `node` (within the loaded data) to `writer`.
    ///
    /// # Parameters
    /// - `node`: the IRI or ID of the node to inspect.
    /// - `writer`: output target for the formatted node information.
    pub fn show_node_info<'a, W: io::Write>(
        &'a mut self,
        node: &'a str,
        writer: &'a mut W,
    ) -> ShowNodeInfoBuilder<'a, W> {
        ShowNodeInfoBuilder::new(self, node, writer)
    }

    /// Returns a `ListEndpointsBuilder` that enumerates known endpoints.
    pub fn list_endpoints<'a>(&'a mut self) -> ListEndpointsBuilder<'a> {
        ListEndpointsBuilder::new(self)
    }

    // ========================================================================
    // ShExOperations methods
    // ========================================================================

    /// Returns a `LoadShexSchemaBuilder` to load a ShEx schema from `schema` (`InputSpec`) into the internal state.
    ///
    /// # Parameters
    /// - `schema`: input specification for the ShEx schema to load.
    pub fn load_shex_schema<'a>(&'a mut self, schema: &'a InputSpec) -> LoadShexSchemaBuilder<'a> {
        LoadShexSchemaBuilder::new(self, schema)
    }

    /// Returns a `CheckShexSchemaBuilder` to perform syntactic/semantic checks on a ShEx schema described by `schema` and write results to
    /// `writer`.
    ///
    /// # Parameters
    /// - `schema`: input specification for the ShEx schema to check.
    /// - `writer`: output target for the check results.
    pub fn check_shex_schema<'a, W: io::Write>(
        &'a self,
        schema: &'a InputSpec,
        writer: &'a mut W,
    ) -> CheckShexSchemaBuilder<'a, W> {
        CheckShexSchemaBuilder::new(self, schema, writer)
    }

    /// Returns a `SerializeShexSchemaBuilder` that writes the currently loaded ShEx schema to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized ShEx schema.
    pub fn serialize_shex_schema<'a, W: io::Write>(&'a self, writer: &'a mut W) -> SerializeShexSchemaBuilder<'a, W> {
        SerializeShexSchemaBuilder::new(self, writer)
    }

    /// Returns a `ResetShexSchemaBuilder` to clear the currently-loaded ShEx schema from state.
    pub fn reset_shex_schema<'a>(&'a mut self) -> ResetShexSchemaBuilder<'a> {
        ResetShexSchemaBuilder::new(self)
    }

    /// Returns a `LoadShapemapBuilder` to load a ShEx shapemap from `shapemap` (`InputSpec`).
    ///
    /// # Parameters
    /// - `shapemap`: input specification for the ShEx shapemap to load.
    pub fn load_shapemap<'a>(&'a mut self, shapemap: &'a InputSpec) -> LoadShapemapBuilder<'a> {
        LoadShapemapBuilder::new(self, shapemap)
    }

    /// Returns an `AddNodeShapeToShapemapBuilder` to add a node/shape association to the shapemap.
    ///
    /// Creates the shapemap if none is currently loaded.
    ///
    /// # Parameters
    /// - `node`: node selector string (e.g. `<http://example.org/node>`).
    pub fn add_node_shape_to_shapemap<'a>(&'a mut self, node: &'a str) -> AddNodeShapeToShapemapBuilder<'a> {
        AddNodeShapeToShapemapBuilder::new(self, node)
    }

    /// Returns a `SerializeShapemapBuilder` that writes the current shapemap to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized shapemap.
    pub fn serialize_shapemap<'a, W: io::Write>(&'a self, writer: &'a mut W) -> SerializeShapemapBuilder<'a, W> {
        SerializeShapemapBuilder::new(self, writer)
    }

    /// Returns a `ResetShapemapBuilder` to clear the stored shapemap.
    pub fn reset_shapemap<'a>(&'a mut self) -> ResetShapemapBuilder<'a> {
        ResetShapemapBuilder::new(self)
    }

    /// Returns a `ValidateShexBuilder` to run ShEx validation using the currently-loaded schema, shapemap and data.
    pub fn validate_shex<'a>(&'a mut self) -> ValidateShexBuilder<'a> {
        ValidateShexBuilder::new(self)
    }

    /// Returns a `SerializeShexValidationResultsBuilder` to write ShEx validation results to `writer`.
    pub fn serialize_shex_validation_results<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeShexValidationResultsBuilder<'a, W> {
        SerializeShexValidationResultsBuilder::new(self, writer)
    }

    /// Returns a `ResetShexBuilder` to clear ShEx validation state and results.
    pub fn reset_shex<'a>(&'a mut self) -> ResetShexBuilder<'a> {
        ResetShexBuilder::new(self)
    }

    // ========================================================================
    // ShaclOperations methods
    // ========================================================================

    /// Returns a `LoadShaclShapesBuilder` to load SHACL shapes into the internal state.
    pub fn load_shacl_shapes<'a>(&'a mut self) -> LoadShaclShapesBuilder<'a> {
        LoadShaclShapesBuilder::new(self)
    }

    /// Returns a `SerializeShaclShapesBuilder` that writes loaded SHACL shapes to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized SHACL shapes.
    pub fn serialize_shacl_shapes<'a, W: io::Write>(&'a self, writer: &'a mut W) -> SerializeShaclShapesBuilder<'a, W> {
        SerializeShaclShapesBuilder::new(self, writer)
    }

    /// Returns a `ResetShaclShapesBuilder` to clear loaded SHACL shapes.
    pub fn reset_shacl_shapes<'a>(&'a mut self) -> ResetShaclShapesBuilder<'a> {
        ResetShaclShapesBuilder::new(self)
    }

    /// Returns a `ValidateShaclBuilder` to perform SHACL validation on the
    /// currently-loaded shapes and data.
    pub fn validate_shacl<'a>(&'a mut self) -> ValidateShaclBuilder<'a> {
        ValidateShaclBuilder::new(self)
    }

    /// Returns a `SerializeShaclValidationResultsBuilder` to write SHACL
    /// validation results to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized SHACL validation results.
    pub fn serialize_shacl_validation_results<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeShaclValidationResultsBuilder<'a, W> {
        SerializeShaclValidationResultsBuilder::new(self, writer)
    }

    /// Returns a `ResetShaclBuilder` to clear SHACL validation
    /// results from the internal state.
    pub fn reset_shacl<'a>(&'a mut self) -> ResetShaclBuilder<'a> {
        ResetShaclBuilder::new(self)
    }

    // ========================================================================
    // QueryOperations methods
    // ========================================================================

    /// Returns a `LoadQueryBuilder` to load a SPARQL query into state from
    /// `query` (`InputSpec`).
    ///
    /// # Parameters
    /// - `query`: input specification for the SPARQL query to load.
    pub fn load_query<'a>(&'a mut self, query: &'a InputSpec) -> LoadQueryBuilder<'a> {
        LoadQueryBuilder::new(self, query)
    }

    /// Returns a `SerializeQueryBuilder` that writes the currently-loaded
    /// query to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized SPARQL query.
    pub fn serialize_query<'a, W: io::Write>(&'a self, writer: &'a mut W) -> SerializeQueryBuilder<'a, W> {
        SerializeQueryBuilder::new(self, writer)
    }

    /// Returns a `ResetQueryBuilder` to clear the stored query.
    pub fn reset_query<'a>(&'a mut self) -> ResetQueryBuilder<'a> {
        ResetQueryBuilder::new(self)
    }

    /// Returns a `RunQueryBuilder` to execute the currently-loaded SPARQL
    /// query against the loaded data.
    pub fn run_query<'a>(&'a mut self) -> RunQueryBuilder<'a> {
        RunQueryBuilder::new(self)
    }

    /// Returns a `SerializeQueryResultsBuilder` that writes query results
    /// to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized query results.
    pub fn serialize_query_results<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeQueryResultsBuilder<'a, W> {
        SerializeQueryResultsBuilder::new(self, writer)
    }

    /// Returns a `ResetQueryResultsBuilder` to clear stored query results.
    pub fn reset_query_results<'a>(&'a mut self) -> ResetQueryResultsBuilder<'a> {
        ResetQueryResultsBuilder::new(self)
    }

    // ========================================================================
    // ComparisonOperations methods
    // ========================================================================

    /// Returns a `ShowSchemaComparisonBuilder` to compare two schemas.
    ///
    /// - `schema1`/`schema2`: input specifications for both schemas.
    /// - `format1`/`format2`: formats for the inputs.
    /// - `mode1`/`mode2`: types for the comparison.
    /// - `writer`: output target for the comparison report.
    pub fn show_schema_comparison<'a, W: io::Write>(
        &'a mut self,
        schema1: &'a InputSpec,
        schema2: &'a InputSpec,
        format1: &'a ComparisonFormat,
        format2: &'a ComparisonFormat,
        mode1: &'a ComparisonMode,
        mode2: &'a ComparisonMode,
        writer: &'a mut W,
    ) -> ShowSchemaComparisonBuilder<'a, W> {
        ShowSchemaComparisonBuilder::new(self, schema1, schema2, format1, format2, mode1, mode2, writer)
    }

    // ========================================================================
    // ConversionOperations methods
    // ========================================================================

    /// Returns a `ShowSchemaConversionBuilder` to convert a schema between
    /// formats or representations.
    ///
    /// - `schema`: input specification
    /// - `input_mode`/`output_mode`: types for the conversion
    /// - `input_format`/`output_format`: concrete format choices
    /// - `writer`: output target for the converted schema/result
    pub fn show_schema_conversion<'a, W: io::Write>(
        &'a mut self,
        schema: &'a InputSpec,
        input_mode: &'a ConversionMode,
        output_mode: &'a ResultConversionMode,
        input_format: &'a ConversionFormat,
        output_format: &'a ResultConversionFormat,
        writer: &'a mut W,
    ) -> ShowSchemaConversionBuilder<'a, W> {
        ShowSchemaConversionBuilder::new(
            self,
            schema,
            input_mode,
            output_mode,
            input_format,
            output_format,
            writer,
        )
    }

    // ========================================================================
    // DctapOperations methods
    // ========================================================================

    /// Returns a `LoadDctapBuilder` to load a DCTAP model from `dctap`.
    ///
    /// # Parameters
    /// - `dctap`: input specification for the DCTAP model to load.
    pub fn load_dctap<'a>(&'a mut self, dctap: &'a InputSpec) -> LoadDctapBuilder<'a> {
        LoadDctapBuilder::new(self, dctap)
    }

    /// Returns a `SerializeDctapBuilder` that writes the loaded DCTAP
    /// model to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized DCTAP model.
    pub fn serialize_dctap<'a, W: io::Write>(&'a self, writer: &'a mut W) -> SerializeDctapBuilder<'a, W> {
        SerializeDctapBuilder::new(self, writer)
    }

    /// Returns a `ResetDctapBuilder` to clear loaded DCTAP from state.
    pub fn reset_dctap<'a>(&'a mut self) -> ResetDctapBuilder<'a> {
        ResetDctapBuilder::new(self)
    }

    // ========================================================================
    // RdfConfigOperations methods
    // ========================================================================

    /// Returns a `LoadRdfConfigBuilder` to load RDF configuration from
    /// `rdf_config` (`InputSpec`).
    pub fn load_rdf_config<'a>(&'a mut self, rdf_config: &'a InputSpec) -> LoadRdfConfigBuilder<'a> {
        LoadRdfConfigBuilder::new(self, rdf_config)
    }

    /// Returns a `SerializeRdfConfigBuilder` that writes the loaded RDF
    /// configuration to `writer`.
    pub fn serialize_rdf_config<'a, W: io::Write>(&'a self, writer: &'a mut W) -> SerializeRdfConfigBuilder<'a, W> {
        SerializeRdfConfigBuilder::new(self, writer)
    }

    /// Returns a `ResetRdfConfigBuilder` to clear the loaded RDF
    /// configuration.
    pub fn reset_rdf_config<'a>(&'a mut self) -> ResetRdfConfigBuilder<'a> {
        ResetRdfConfigBuilder::new(self)
    }

    // ========================================================================
    // PgSchemaOperations methods
    // ========================================================================

    /// Returns a `LoadPgSchemaBuilder` to load a PGSchema from `pg_schema`.
    ///
    /// # Parameters
    /// - `pg_schema`: input specification for the PGSchema to load.
    pub fn load_pg_schema<'a>(&'a mut self, pg_schema: &'a InputSpec) -> LoadPgSchemaBuilder<'a> {
        LoadPgSchemaBuilder::new(self, pg_schema)
    }

    /// Returns a `SerializePgSchemaBuilder` that writes the loaded
    /// PGSchema to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized PGSchema.
    pub fn serialize_pg_schema<'a, W: io::Write>(&'a self, writer: &'a mut W) -> SerializePgSchemaBuilder<'a, W> {
        SerializePgSchemaBuilder::new(self, writer)
    }

    /// Returns a `ResetPgSchemaBuilder` to clear the loaded PGSchema.
    pub fn reset_pg_schema<'a>(&'a mut self) -> ResetPgSchemaBuilder<'a> {
        ResetPgSchemaBuilder::new(self)
    }

    /// Returns a `LoadTypemapBuilder` to load a typemap into state.
    ///
    /// # Parameters
    /// - `typemap`: input specification for the typemap to load.
    pub fn load_typemap<'a>(&'a mut self, typemap: &'a InputSpec) -> LoadTypemapBuilder<'a> {
        LoadTypemapBuilder::new(self, typemap)
    }

    /// Returns a `ResetTypemapBuilder` to clear the typemap.
    pub fn reset_typemap<'a>(&'a mut self) -> ResetTypemapBuilder<'a> {
        ResetTypemapBuilder::new(self)
    }

    /// Returns a `PgSchemaValidationBuilder` to validate the currently
    /// loaded PGSchema and typemap.
    pub fn validate_pgschema<'a>(&'a mut self) -> PgSchemaValidationBuilder<'a> {
        PgSchemaValidationBuilder::new(self)
    }

    /// Returns a `SerializePgSchemaValidationResultsBuilder` to write PG
    /// schema validation results to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized PGSchema validation results.
    pub fn serialize_pgschema_validation_results<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializePgSchemaValidationResultsBuilder<'a, W> {
        SerializePgSchemaValidationResultsBuilder::new(self, writer)
    }

    /// Returns a `ResetPgSchemaValidationBuilder` to clear PGSchema
    /// validation results.
    pub fn reset_pg_schema_validation<'a>(&'a mut self) -> ResetPgSchemaValidationBuilder<'a> {
        ResetPgSchemaValidationBuilder::new(self)
    }

    // ========================================================================
    // GenerationOperations methods
    // ========================================================================

    /// Returns a `GenerateDataBuilder` to synthesize mock data based on the provided schema.
    ///
    /// # Parameters
    /// - `schema`: input specification for the schema (e.g., ShEx file)
    /// - `schema_format`: format of the provided schema
    /// - `number_entities`: approximate number of target entities to generate
    pub fn generate_data<'a>(
        &'a self,
        schema: &'a InputSpec,
        schema_format: &'a GenerationSchemaFormat,
        number_entities: usize,
    ) -> GenerateDataBuilder<'a> {
        GenerateDataBuilder::new(self, schema, schema_format, number_entities)
    }
}
