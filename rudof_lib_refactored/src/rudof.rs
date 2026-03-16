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
        InputSpec, QueryType, ComparisonFormat, ComparisonMode, ConversionFormat, ConversionMode, 
        ResultConversionFormat, ResultConversionMode, GenerationSchemaFormat
    },
    types::{Data, QueryResult},
    api::{
        core::{CoreOperations, builders::{UpdateConfigBuilder, ResetAllBuilder, VersionBuilder, ConfigBuilder}},
        data::builders::{
            ShowNodeInfoBuilder, LoadDataBuilder, ResetDataBuilder,
            LoadServiceDescriptionBuilder, SerializeDataBuilder,
            SerializeServiceDescriptionBuilder, ResetServiceDescriptionBuilder,
        },
        shex::builders::{
            LoadShexSchemaBuilder, SerializeShexSchemaBuilder, ResetShexSchemaBuilder,
            LoadShapemapBuilder, SerializeShapemapBuilder, ResetShapemapBuilder,
            ValidateShexBuilder, SerializeShexValidationResultsBuilder, ResetShexBuilder,
        },
        shacl::builders::{
            LoadShaclSchemaBuilder, SerializeShaclSchemaBuilder, ResetShaclSchemaBuilder,
            LoadShapesBuilder, SerializeShapesBuilder, ResetShapesBuilder,
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
            RunPgSchemaValidationBuilder, SerializePgSchemaValidationResultsBuilder, ResetPgSchemaValidationBuilder,
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

impl Rudof {
    // ========================================================================
    // RudofCore methods
    // ========================================================================
    
    pub fn new(config: &RudofConfig) -> Result<Self> {
        <Self as CoreOperations>::new(config)
    }

    pub fn version<'a>(&'a self) -> VersionBuilder<'a> {
        VersionBuilder::new(self)
    }

    pub fn config<'a>(&'a self) -> ConfigBuilder<'a> {
        ConfigBuilder::new(self)
    }

    pub fn update_config<'a>(&'a mut self, config: &'a RudofConfig) -> UpdateConfigBuilder<'a> {
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
        data: &'a [InputSpec],
    ) -> LoadDataBuilder<'a> {
        LoadDataBuilder::new(self, data)
    }

    pub fn serialize_data<'a, W: io::Write>(
        &'a self, 
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
        &'a self,
        node: &'a str,
        writer: &'a mut W,
    ) -> ShowNodeInfoBuilder<'a, W> {
        ShowNodeInfoBuilder::new(self, node, writer)
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

    pub fn load_shacl_schema<'a>(
        &'a mut self,
        schema: &'a InputSpec,
    ) -> LoadShaclSchemaBuilder<'a> {
        LoadShaclSchemaBuilder::new(self, schema)
    }

    pub fn serialize_shacl_schema<'a, W: io::Write>(
        &'a self, 
        writer: &'a mut W
    ) -> SerializeShaclSchemaBuilder<'a, W> {
        SerializeShaclSchemaBuilder::new(self, writer)
    }

    pub fn reset_shacl_schema<'a>(&'a mut self) -> ResetShaclSchemaBuilder<'a> {
        ResetShaclSchemaBuilder::new(self)
    }

    pub fn load_shapes<'a>(
        &'a mut self,
        shapes: &'a InputSpec,
    ) -> LoadShapesBuilder<'a> {
        LoadShapesBuilder::new(self, shapes)
    }

    pub fn serialize_shapes<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> SerializeShapesBuilder<'a, W> {
        SerializeShapesBuilder::new(self, writer)
    }

    pub fn reset_shapes<'a>(&'a mut self) -> ResetShapesBuilder<'a> {
        ResetShapesBuilder::new(self)
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
        query_type: &'a QueryType,
    ) -> LoadQueryBuilder<'a> {
        LoadQueryBuilder::new(self, query, query_type)
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
        &'a self,
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
        &'a self,
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

    pub fn run_pgschema_validation<'a>(&'a mut self) -> RunPgSchemaValidationBuilder<'a> {
        RunPgSchemaValidationBuilder::new(self)
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