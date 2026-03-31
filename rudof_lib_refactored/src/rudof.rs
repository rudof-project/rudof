use sparql_service::RdfData;
use pgschema::{type_map::TypeMap, validation_result::ValidationResult, pgs::PropertyGraphSchema};
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
use shex_validation::Validator as ShExValidator;
use crate::{
    RudofConfig, 
    errors::RudofError,
    formats::{
        InputSpec, ComparisonFormat, ComparisonMode, ConversionFormat, ConversionMode, 
        ResultConversionFormat, ResultConversionMode, GenerationSchemaFormat
    },
    types::{Data, QueryResult},
    api::{
        core::{CoreOperations, builders::{UpdateConfigBuilder, ResetAllBuilder, VersionBuilder, ConfigBuilder}},
        data::builders::{
            ShowNodeInfoBuilder, LoadDataBuilder, ResetDataBuilder,
            LoadServiceDescriptionBuilder, SerializeDataBuilder,
            SerializeServiceDescriptionBuilder, ResetServiceDescriptionBuilder, ListEndpointsBuilder
        },
        shex::builders::{
            CheckShexSchemaBuilder, LoadShexSchemaBuilder, SerializeShexSchemaBuilder, ResetShexSchemaBuilder,
            LoadShapemapBuilder, SerializeShapemapBuilder, ResetShapemapBuilder,
            ValidateShexBuilder, SerializeShexValidationResultsBuilder, ResetShexBuilder,
        },
        shacl::builders::{
            LoadShaclShapesBuilder, SerializeShaclShapesBuilder, ResetShaclShapesBuilder,
            ValidateShaclBuilder, SerializeShaclValidationResultsBuilder, ResetShaclValidationBuilder,
        },
        query::builders::{
            LoadQueryBuilder, SerializeQueryBuilder, ResetQueryBuilder,
            RunQueryBuilder, SerializeQueryResultsBuilder, ResetQueryResultsBuilder,
        },
        comparison::builders::ShowSchemaComparisonBuilder,
        conversion::builders::ShowSchemaConversionBuilder,
        dctap::builders::{LoadDctapBuilder, SerializeDctapBuilder, ResetDctapBuilder},
        rdf_config::builders::{LoadRdfConfigBuilder, SerializeRdfConfigBuilder, ResetRdfConfigBuilder},
        pgschema::builders::{
            LoadPgSchemaBuilder, SerializePgSchemaBuilder, ResetPgSchemaBuilder,
            PgSchemaValidationBuilder, SerializePgSchemaValidationResultsBuilder, ResetPgSchemaValidationBuilder,
            LoadTypemapBuilder, ResetTypemapBuilder
        },
        generation::builders::GenerateDataBuilder,
    },
};
use std::io;

pub type Result<T> = std::result::Result<T, RudofError>;

/// This represents the public API to interact with `rudof`
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
}

impl Rudof {
    // ========================================================================
    // RudofCore methods
    // ========================================================================
    
    pub fn new(config: RudofConfig) -> Self {
        <Self as CoreOperations>::new(config)
    }

    pub fn version<'a>(&'a self) -> VersionBuilder<'a> {
        VersionBuilder::new(self)
    }

    pub fn config<'a>(&'a self) -> ConfigBuilder<'a> {
        ConfigBuilder::new(self)
    }

    pub fn update_config<'a>(&'a mut self, config: RudofConfig) -> UpdateConfigBuilder<'a> {
        UpdateConfigBuilder::new(self, config)
    }

    pub fn reset_all<'a>(&'a mut self) -> ResetAllBuilder<'a> {
        ResetAllBuilder::new(self)
    }

    // ========================================================================
    // DataOperations methods
    // ========================================================================

    pub fn load_data<'a>(
        &'a mut self,
    ) -> LoadDataBuilder<'a> {
        LoadDataBuilder::new(self)
    }

    pub fn serialize_data<'a, W: io::Write>(
        &'a mut self, 
        writer: &'a mut W
    ) -> SerializeDataBuilder<'a, W> {
        SerializeDataBuilder::new(self, writer)
    }

    pub fn reset_data<'a>(&'a mut self) -> ResetDataBuilder<'a> {
        ResetDataBuilder::new(self)
    }

    pub fn load_service_description<'a>(
        &'a mut self,
        service: &'a InputSpec,
    ) -> LoadServiceDescriptionBuilder<'a> {
        LoadServiceDescriptionBuilder::new(self, service)
    }

    pub fn serialize_service_description<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeServiceDescriptionBuilder<'a, W> {
        SerializeServiceDescriptionBuilder::new(self, writer)
    }

    pub fn reset_service_description<'a>(&'a mut self) -> ResetServiceDescriptionBuilder<'a> {
        ResetServiceDescriptionBuilder::new(self)
    }

    pub fn show_node_info<'a, W: io::Write>(
        &'a mut self,
        node: &'a str,
        writer: &'a mut W,
    ) -> ShowNodeInfoBuilder<'a, W> {
        ShowNodeInfoBuilder::new(self, node, writer)
    }

    pub fn list_endpoints<'a>(&'a mut self) -> ListEndpointsBuilder<'a> {
        ListEndpointsBuilder::new(self)
    }

    // ========================================================================
    // ShExOperations methods
    // ========================================================================

    pub fn load_shex_schema<'a>(
        &'a mut self,
        schema: &'a InputSpec,
    ) -> LoadShexSchemaBuilder<'a> {
        LoadShexSchemaBuilder::new(self, schema)
    }

    pub fn check_shex_schema<'a, W: io::Write>(
        &'a self,
        schema: &'a InputSpec,
        writer: &'a mut W
    ) -> CheckShexSchemaBuilder<'a, W> {
        CheckShexSchemaBuilder::new(self, schema, writer)
    }

    pub fn serialize_shex_schema<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W
    ) -> SerializeShexSchemaBuilder<'a, W> {
        SerializeShexSchemaBuilder::new(self, writer)
    }

    pub fn reset_shex_schema<'a>(&'a mut self) -> ResetShexSchemaBuilder<'a> {
        ResetShexSchemaBuilder::new(self)
    }

    pub fn load_shapemap<'a>(
        &'a mut self,
        shapemap: &'a InputSpec,
    ) -> LoadShapemapBuilder<'a> {
        LoadShapemapBuilder::new(self, shapemap)
    }

    pub fn serialize_shapemap<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeShapemapBuilder<'a, W> {
        SerializeShapemapBuilder::new(self, writer)
    }

    pub fn reset_shapemap<'a>(&'a mut self) -> ResetShapemapBuilder<'a> {
        ResetShapemapBuilder::new(self)
    }

    pub fn validate_shex<'a>(&'a mut self) -> ValidateShexBuilder<'a> {
        ValidateShexBuilder::new(self)
    }

    pub fn serialize_shex_validation_results<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeShexValidationResultsBuilder<'a, W> {
        SerializeShexValidationResultsBuilder::new(self, writer)
    }

    pub fn reset_shex<'a>(&'a mut self) -> ResetShexBuilder<'a> {
        ResetShexBuilder::new(self)
    }

    // ========================================================================
    // ShaclOperations methods
    // ========================================================================

    pub fn load_shacl_shapes<'a>(
        &'a mut self,
    ) -> LoadShaclShapesBuilder<'a> {
        LoadShaclShapesBuilder::new(self)
    }

    pub fn serialize_shacl_shapes<'a, W: io::Write>(
        &'a self, 
        writer: &'a mut W
    ) -> SerializeShaclShapesBuilder<'a, W> {
        SerializeShaclShapesBuilder::new(self, writer)
    }

    pub fn reset_shacl_shapes<'a>(&'a mut self) -> ResetShaclShapesBuilder<'a> {
        ResetShaclShapesBuilder::new(self)
    }

    pub fn validate_shacl<'a>(&'a mut self) -> ValidateShaclBuilder<'a> {
        ValidateShaclBuilder::new(self)
    }

    pub fn serialize_shacl_validation_results<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeShaclValidationResultsBuilder<'a, W> {
        SerializeShaclValidationResultsBuilder::new(self, writer)
    }

    pub fn reset_shacl_validation<'a>(&'a mut self) -> ResetShaclValidationBuilder<'a> {
        ResetShaclValidationBuilder::new(self)
    }

    // ========================================================================
    // QueryOperations methods
    // ========================================================================

    pub fn load_query<'a>(
        &'a mut self,
        query: &'a InputSpec,
    ) -> LoadQueryBuilder<'a> {
        LoadQueryBuilder::new(self, query)
    }

    pub fn serialize_query<'a, W: io::Write>(&'a self, writer: &'a mut W) -> SerializeQueryBuilder<'a, W> {
        SerializeQueryBuilder::new(self, writer)
    }

    pub fn reset_query<'a>(&'a mut self) -> ResetQueryBuilder<'a> {
        ResetQueryBuilder::new(self)
    }

    pub fn run_query<'a>(&'a mut self) -> RunQueryBuilder<'a> {
        RunQueryBuilder::new(self)
    }

    pub fn serialize_query_results<'a, W: io::Write>(
        &'a self, 
        writer: &'a mut W
    ) -> SerializeQueryResultsBuilder<'a, W> {
        SerializeQueryResultsBuilder::new(self, writer)
    }

    pub fn reset_query_results<'a>(&'a mut self) -> ResetQueryResultsBuilder<'a> {
        ResetQueryResultsBuilder::new(self)
    }

    // ========================================================================
    // ComparisonOperations methods
    // ========================================================================

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

    pub fn show_schema_conversion<'a, W: io::Write>(
        &'a mut self,
        schema: &'a InputSpec,
        input_mode: &'a ConversionMode,
        output_mode: &'a ResultConversionMode,
        input_format: &'a ConversionFormat,
        output_format: &'a ResultConversionFormat,
        writer: &'a mut W,
    ) -> ShowSchemaConversionBuilder<'a, W> {
        ShowSchemaConversionBuilder::new(self, schema, input_mode, output_mode, input_format, output_format, writer)
    }

    // ========================================================================
    // DctapOperations methods
    // ========================================================================

    pub fn load_dctap<'a>(
        &'a mut self,
        dctap: &'a InputSpec,
    ) -> LoadDctapBuilder<'a> {
        LoadDctapBuilder::new(self, dctap)
    }

    pub fn serialize_dctap<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeDctapBuilder<'a, W> {
        SerializeDctapBuilder::new(self, writer)
    }

    pub fn reset_dctap<'a>(&'a mut self) -> ResetDctapBuilder<'a> {
        ResetDctapBuilder::new(self)
    }

    // ========================================================================
    // RdfConfigOperations methods
    // ========================================================================

    pub fn load_rdf_config<'a>(
        &'a mut self,
        rdf_config: &'a InputSpec,
    ) -> LoadRdfConfigBuilder<'a> {
        LoadRdfConfigBuilder::new(self, rdf_config)
    }

    pub fn serialize_rdf_config<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeRdfConfigBuilder<'a, W> {
        SerializeRdfConfigBuilder::new(self, writer)
    }

    pub fn reset_rdf_config<'a>(&'a mut self) -> ResetRdfConfigBuilder<'a> {
        ResetRdfConfigBuilder::new(self)
    }

    // ========================================================================
    // PgSchemaOperations methods
    // ========================================================================

    pub fn load_pg_schema<'a>(
        &'a mut self,
        pg_schema: &'a InputSpec,
    ) -> LoadPgSchemaBuilder<'a> {
        LoadPgSchemaBuilder::new(self, pg_schema)
    }

    pub fn serialize_pg_schema<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializePgSchemaBuilder<'a, W> {
        SerializePgSchemaBuilder::new(self, writer)
    }

    pub fn reset_pg_schema<'a>(&'a mut self) -> ResetPgSchemaBuilder<'a> {
        ResetPgSchemaBuilder::new(self)
    }

    pub fn load_typemap<'a>(
        &'a mut self,
        typemap: &'a InputSpec,
    ) -> LoadTypemapBuilder<'a> {
        LoadTypemapBuilder::new(self, typemap)
    }

    pub fn reset_typemap<'a>(&'a mut self) -> ResetTypemapBuilder<'a> {
        ResetTypemapBuilder::new(self)
    }

    pub fn validate_pgschema<'a>(&'a mut self) -> PgSchemaValidationBuilder<'a> {
        PgSchemaValidationBuilder::new(self)
    }

    pub fn serialize_pgschema_validation_results<'a,W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializePgSchemaValidationResultsBuilder<'a, W> {
        SerializePgSchemaValidationResultsBuilder::new(self, writer)
    }

    pub fn reset_pg_schema_validation<'a>(&'a mut self) -> ResetPgSchemaValidationBuilder<'a> {
        ResetPgSchemaValidationBuilder::new(self)
    }

    // ========================================================================
    // GenerationOperations methods
    // ========================================================================

    pub fn generate_data<'a>(
        &'a self,
        schema: &'a InputSpec,
        schema_format: &'a GenerationSchemaFormat,
        number_entities: usize,
    ) -> GenerateDataBuilder<'a> {
        GenerateDataBuilder::new(self, schema, schema_format, number_entities)
    }
}