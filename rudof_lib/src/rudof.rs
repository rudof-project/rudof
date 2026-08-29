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
            DereferenceBuilder, ListEndpointsBuilder, LoadDataBuilder, LoadServiceDescriptionBuilder, ResetDataBuilder,
            ResetServiceDescriptionBuilder, SerializeDataBuilder, SerializeServiceDescriptionBuilder,
            ShowNodeInfoBuilder,
        },
        dctap::builders::{LoadDctapBuilder, ResetDctapBuilder, SerializeDctapBuilder},
        generation::builders::GenerateDataBuilder,
        map_state::builders::{LoadMapStateBuilder, SerializeMapStateBuilder},
        materialize::builders::MaterializeBuilder,
        pg_db::builders::{
            ConnectPgDbBuilder, LoadPgDbBuilder, PgDbDdlBuilder, QueryCypherBuilder, ResetPgDbConnectionBuilder,
        },
        pgschema::builders::{
            LoadPgSchemaBuilder, LoadTypemapBuilder, PgSchemaValidationBuilder, ResetPgSchemaBuilder,
            ResetPgSchemaValidationBuilder, ResetTypemapBuilder, SerializePgSchemaBuilder,
            SerializePgSchemaValidationResultsBuilder,
        },
        prefixes::builders::{
            AddPrefixBuilder, CopyPrefixBuilder, PrefixesBuilder, RemovePrefixBuilder, RenamePrefixBuilder,
        },
        query::builders::{
            LoadSparqlQueryBuilder, ResetQueryResultsBuilder, ResetSparqlQueryBuilder, RunQueryBuilder,
            SerializeQueryResultsBuilder, SerializeSparqlQueryBuilder,
        },
        rdf_config::builders::{LoadRdfConfigBuilder, ResetRdfConfigBuilder, SerializeRdfConfigBuilder},
        shacl::builders::{
            LoadShaclShapesBuilder, ResetShaclBuilder, ResetShaclShapesBuilder, SerializeShaclShapesBuilder,
            SerializeShaclValidationResultsBuilder, ValidateShaclBuilder,
        },
        shex::builders::{
            AddNodeShapeToShapemapBuilder, CheckShexSchemaBuilder, CompileShexSchemaToFileBuilder, LoadShapemapBuilder,
            LoadShexSchemaBuilder, LoadShexSchemaPrecompiledBuilder, ResetShapemapBuilder, ResetShexBuilder,
            ResetShexSchemaBuilder, SerializeShapemapBuilder, SerializeShexSchemaBuilder,
            SerializeShexValidationResultsBuilder, ValidateShexBuilder,
        },
    },
    errors::{RudofError, ShExError},
    formats::{
        ComparisonFormat, ComparisonMode, ConversionFormat, ConversionMode, DbEngine, GenerationSchemaFormat,
        InputSpec, ResultConversionFormat, ResultConversionMode,
    },
    types::{Data, QueryResult},
};
use dctap::DCTap as DCTAP;
use pgschema::{pgs::PropertyGraphSchema, type_map::TypeMap, validation_result::ValidationResult};
use prefixmap::PrefixMap;
use rdf_config::RdfConfigModel;
use rudof_rdf::rdf_core::query::SparqlQuery;
use serde::Serialize;
use shacl::ir::IRSchema;
use shacl::validator::report::ValidationReport;
use shex_ast::ir::external_resolver::{
    ExternalResolverInfo, ExternalShapeResolverRegistry, available_external_resolvers, resolver_from_spec,
};
use shex_ast::ir::schema_ir::SchemaIR as ShExSchemaIR;
use shex_ast::shapemap::{QueryShapeMap, ResultShapeMap};
use shex_ast::{Schema as ShExSchema, ir::map_state::MapState};
use shex_validation::Validator as ShExValidator;
use sparql_service::ServiceDescription;
use std::io;
use std::path::{Path, PathBuf};

/// Typedef for `Result` returned by Rudof operations, where errors are boxed into `RudofError`.
/// Allows easier error handling across library-specific subsystems.
pub type Result<T> = std::result::Result<T, RudofError>;

/// A short summary of the RDF or PG data loaded into a [`Rudof`], returned by
/// [`Rudof::data_stats`].
#[derive(Debug, Clone, Copy)]
pub enum DataStats {
    Rdf { triples: usize },
    Pg { nodes: usize, edges: usize },
}

/// Connection info for a property graph database, set by
/// [`Rudof::connect_pg_db`] and consumed by [`Rudof::load_pg_db`]/
/// [`Rudof::query_cypher`] when no explicit override is given.
///
/// Holds only the information needed to open a connection, not a live
/// handle: `lbug::Connection<'a>` borrows its `Database`, so each operation
/// opens (and drops) its own short-lived connection rather than keeping one
/// open across calls -- matching how the CLI's `connect`/`load`/`query`
/// commands already behaved before this state existed.
#[derive(Debug, Clone)]
pub struct PgDbConnection {
    pub engine: DbEngine,
    pub path: PathBuf,
    pub read_only: bool,
}

/// Information about a database reported by [`Rudof::connect_pg_db`].
#[derive(Debug, Clone)]
pub struct PgDbInfo {
    pub storage_version: u64,
    pub library_source: String,
}

/// The result of a Cypher query run by [`Rudof::query_cypher`].
///
/// Row values are serialized to JSON rather than exposed as `lbug`'s own
/// value type, so this is independently useful (and, via `serde`, directly
/// convertible to a native Python object) without leaking a `lbug`-specific
/// type across the public API.
#[derive(Debug, Clone, Serialize)]
pub struct CypherQueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub compiling_time_ms: f64,
    pub execution_time_ms: f64,
}

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

    /// Current SHACL Schema Internal Representation
    pub(crate) shacl_shapes: Option<IRSchema>,

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

    /// Connection info for a property graph database, set by `connect_pg_db`
    pub(crate) pg_db_connection: Option<PgDbConnection>,

    /// Current SPARQL query, loaded by `sparql` (show) or `query -q` (load + run)
    pub(crate) sparql_query: Option<SparqlQuery>,

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

    /// Current list of prefix map declarations. These prefix map declarations will be assumed and prepended by default to RDF data, SPARQL queries, ShEx schemas and SHACL shapes to facilitate handling prefixes
    pub(crate) prefixes: Option<PrefixMap>,
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
    // External-shape resolver management
    // ========================================================================

    /// Register an external-shape resolver from a spec string.
    ///
    /// The spec follows the grammar `<kind>[:<arg>]`. Built-in kinds:
    /// - `reject-all` — reject any EXTERNAL shape not handled by an earlier resolver.
    /// - `schema:<path>` — substitute EXTERNAL shape declarations using a ShEx file at `<path>`.
    ///
    /// Resolvers are prepended to the chain, so the most recently added is consulted first.
    /// Use [`Self::list_external_resolvers`] to enumerate the built-in kinds.
    pub fn add_external_resolver(&mut self, spec: &str) -> Result<()> {
        let resolver = resolver_from_spec(spec).map_err(|e| ShExError::InvalidExternalResolverSpec {
            spec: spec.to_string(),
            error: e.to_string(),
        })?;
        let vc = self
            .config
            .shex_validator()
            .clone()
            .with_external_resolver_arc(resolver);
        self.config = std::mem::take(&mut self.config).with_shex_validator(vc);
        Ok(())
    }

    /// Reset the external-shape resolver chain to the default
    /// (only `RejectAllExternalResolver`).
    pub fn clear_external_resolvers(&mut self) {
        let vc = self
            .config
            .shex_validator()
            .clone()
            .with_external_shape_resolver_registry(ExternalShapeResolverRegistry::default());
        self.config = std::mem::take(&mut self.config).with_shex_validator(vc);
    }

    /// Enumerate the built-in external-shape resolver kinds. Each entry
    /// describes a `name`, a one-line `description`, and the `spec_syntax`
    /// accepted by [`Self::add_external_resolver`].
    pub fn list_external_resolvers() -> Vec<ExternalResolverInfo> {
        available_external_resolvers()
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

    /// Returns a `LoadMapStateBuilder` to load a MapState from a JSON file at `path`.
    ///
    /// # Parameters
    /// - `path`: filesystem path to the JSON-encoded MapState file.
    pub fn load_map_state<'a>(&'a mut self, path: &'a std::path::Path) -> LoadMapStateBuilder<'a> {
        LoadMapStateBuilder::new(self, path)
    }

    pub fn serialize_map_state<'a, W: io::Write>(&'a mut self, writer: &'a mut W) -> SerializeMapStateBuilder<'a, W> {
        SerializeMapStateBuilder::new(self, writer)
    }

    /// Returns a `MaterializeBuilder` to generate an RDF graph from the current
    /// ShEx schema and Map semantic-action state.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized RDF graph.
    pub fn materialize<'a, W: io::Write>(&'a self, writer: &'a mut W) -> MaterializeBuilder<'a, W> {
        MaterializeBuilder::new(self, writer)
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

    /// Returns whether RDF or PG data is currently loaded in `Rudof`'s state.
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    /// Returns a short summary of the RDF or PG data currently loaded, if any.
    ///
    /// This exists for callers (e.g. the interactive shell) that just want
    /// counts rather than the whole loaded value, which isn't part of this
    /// crate's public API.
    pub fn data_stats(&self) -> Option<DataStats> {
        match self.data.as_ref()? {
            Data::RDFData(rdf) => Some(DataStats::Rdf {
                triples: rdf.all_triples().map(Iterator::count).unwrap_or(0),
            }),
            Data::PGData(pg) => Some(DataStats::Pg {
                nodes: pg.node_count(),
                edges: pg.edge_count(),
            }),
        }
    }

    /// Returns whether a SPARQL query is currently loaded, regardless of
    /// whether it has been run yet (e.g. loaded by `sparql`, which only
    /// loads and shows, without running).
    pub fn has_sparql_query(&self) -> bool {
        self.sparql_query.is_some()
    }

    /// Returns the currently loaded ShEx schema, if any.
    pub fn shex_schema(&self) -> Option<&ShExSchema> {
        self.shex_schema.as_ref()
    }

    /// Returns the currently loaded SHACL shapes, if any.
    pub fn shacl_shapes(&self) -> Option<&IRSchema> {
        self.shacl_shapes.as_ref()
    }

    /// Returns the currently loaded DCTAP model, if any.
    pub fn dctap(&self) -> Option<&DCTAP> {
        self.dctap.as_ref()
    }

    /// Returns the currently loaded PGSchema, if any.
    pub fn pg_schema(&self) -> Option<&PropertyGraphSchema> {
        self.pg_schema.as_ref()
    }

    /// Returns the currently loaded SPARQL service description, if any.
    pub fn service_description(&self) -> Option<&ServiceDescription> {
        self.service_description.as_ref()
    }

    /// Returns a `DereferenceBuilder` to fetch `uri` over HTTP(S) — content-negotiating
    /// for an RDF serialization and following redirects — and merge the result into
    /// `Rudof`'s state.
    ///
    /// # Parameters
    /// - `uri`: the absolute IRI to dereference.
    pub fn dereference<'a>(&'a mut self, uri: &'a str) -> DereferenceBuilder<'a> {
        DereferenceBuilder::new(self, uri)
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

    /// Returns a `CompileShexSchemaToFileBuilder` that writes the currently loaded
    /// ShEx `SchemaIR` to `writer` as a precompiled cache.
    ///
    /// # Parameters
    /// - `writer`: output target for the cache bytes.
    pub fn compile_shex_schema_to_file<'a, W: io::Write>(
        &'a self,
        writer: &'a mut W,
    ) -> CompileShexSchemaToFileBuilder<'a, W> {
        CompileShexSchemaToFileBuilder::new(self, writer)
    }

    /// Returns a `LoadShexSchemaPrecompiledBuilder` to load a precompiled
    /// ShEx `SchemaIR` cache from `schema` (`InputSpec`).
    ///
    /// # Parameters
    /// - `schema`: input specification pointing at the cache.
    pub fn load_shex_schema_precompiled<'a>(
        &'a mut self,
        schema: &'a InputSpec,
    ) -> LoadShexSchemaPrecompiledBuilder<'a> {
        LoadShexSchemaPrecompiledBuilder::new(self, schema)
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

    /// Returns the result of the most recent `validate_shex()` call, if any.
    pub fn shex_validation_results(&self) -> Option<&ResultShapeMap> {
        self.shex_validation_results.as_ref()
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

    /// Returns a `LoadSparqlQueryBuilder` to load a SPARQL query into state
    /// from `query` (`InputSpec`).
    ///
    /// # Parameters
    /// - `query`: input specification for the SPARQL query to load.
    pub fn load_sparql_query<'a>(&'a mut self, query: &'a InputSpec) -> LoadSparqlQueryBuilder<'a> {
        LoadSparqlQueryBuilder::new(self, query)
    }

    /// Returns a `SerializeSparqlQueryBuilder` that writes the
    /// currently-loaded query to `writer`.
    ///
    /// # Parameters
    /// - `writer`: output target for the serialized SPARQL query.
    pub fn serialize_sparql_query<'a, W: io::Write>(&'a self, writer: &'a mut W) -> SerializeSparqlQueryBuilder<'a, W> {
        SerializeSparqlQueryBuilder::new(self, writer)
    }

    /// Returns a `ResetSparqlQueryBuilder` to clear the stored query.
    pub fn reset_sparql_query<'a>(&'a mut self) -> ResetSparqlQueryBuilder<'a> {
        ResetSparqlQueryBuilder::new(self)
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

    /// Returns the results of the most recent `run_query()` call, if any.
    pub fn query_results(&self) -> Option<&QueryResult> {
        self.query_results.as_ref()
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
    // PgDbOperations methods
    // ========================================================================

    /// Returns a `ConnectPgDbBuilder` to open (creating if necessary) a
    /// property graph database and store its connection info for later
    /// `load_pg_db`/`query_cypher` calls.
    ///
    /// # Parameters
    /// - `path`: path to the database directory (not needed with `.with_in_memory(true)`).
    pub fn connect_pg_db<'a>(&'a mut self, path: Option<&'a Path>) -> ConnectPgDbBuilder<'a> {
        ConnectPgDbBuilder::new(self, path)
    }

    /// Returns a `PgDbDdlBuilder` to derive a property graph schema from
    /// `data` and emit it as DDL. Never touches loaded RDF data or the
    /// database connection.
    ///
    /// # Parameters
    /// - `data`: RDF data to derive the schema from.
    pub fn pg_db_ddl<'a>(&'a self, data: &'a [InputSpec]) -> PgDbDdlBuilder<'a> {
        PgDbDdlBuilder::new(self, data)
    }

    /// Returns a `LoadPgDbBuilder` to load `data`, validate it with SHACL,
    /// derive a property graph schema, and copy it into the connected
    /// database.
    ///
    /// # Parameters
    /// - `data`: RDF data to load.
    /// - `writer`: destination for progress output.
    pub fn load_pg_db<'a, W: io::Write>(&'a mut self, data: &'a [InputSpec], writer: &'a mut W) -> LoadPgDbBuilder<'a, W> {
        LoadPgDbBuilder::new(self, data, writer)
    }

    /// Returns a `QueryCypherBuilder` to run a Cypher query against the
    /// connected database.
    ///
    /// # Parameters
    /// - `query`: a file, a URL, `-` for stdin, or the Cypher query text itself.
    pub fn query_cypher<'a>(&'a mut self, query: &'a InputSpec) -> QueryCypherBuilder<'a> {
        QueryCypherBuilder::new(self, query)
    }

    /// Returns a `ResetPgDbConnectionBuilder` to clear the stored property
    /// graph database connection info.
    pub fn reset_pg_db_connection<'a>(&'a mut self) -> ResetPgDbConnectionBuilder<'a> {
        ResetPgDbConnectionBuilder::new(self)
    }

    /// Returns the property graph database connection info stored by the
    /// most recent `connect_pg_db` call, if any.
    pub fn pg_db_connection(&self) -> Option<&PgDbConnection> {
        self.pg_db_connection.as_ref()
    }

    // ========================================================================
    // GenerationOperations methods
    // ========================================================================

    /// Returns a `GenerateDataBuilder` to synthesize mock data based on the provided schema.
    ///
    /// # Parameters
    /// - `schema`: input specification for the schema (e.g., ShEx file)
    /// - `schema_format`: format of the provided schema
    /// - `number_entities`: approximate number of target entities to generate. `None` defers to
    ///   the `entity_count` set by [`GenerateDataBuilder::with_config_file`], or the generator's
    ///   own default if neither is given.
    pub fn generate_data<'a>(
        &'a self,
        schema: &'a InputSpec,
        schema_format: &'a GenerationSchemaFormat,
        number_entities: Option<usize>,
    ) -> GenerateDataBuilder<'a> {
        GenerateDataBuilder::new(self, schema, schema_format, number_entities)
    }

    // ========================================================================
    // PrefixesOperations methods
    // ========================================================================

    /// Returns a `PrefixesBuilder` that exposes the current default `PrefixMap`.
    pub fn prefixes<'a>(&'a self) -> PrefixesBuilder<'a> {
        PrefixesBuilder::new(self)
    }

    /// Returns an `AddPrefixBuilder` to add `alias` associated with `iri` to
    /// the default prefixes.
    pub fn add_prefix<'a>(&'a mut self, alias: &'a str, iri: &'a str) -> AddPrefixBuilder<'a> {
        AddPrefixBuilder::new(self, alias, iri)
    }

    /// Returns a `RemovePrefixBuilder` to remove `alias` from the default prefixes.
    pub fn remove_prefix<'a>(&'a mut self, alias: &'a str) -> RemovePrefixBuilder<'a> {
        RemovePrefixBuilder::new(self, alias)
    }

    /// Returns a `RenamePrefixBuilder` to rename `old_alias` to `new_alias`
    /// in the default prefixes, keeping the same associated IRI.
    pub fn rename_prefix<'a>(&'a mut self, old_alias: &'a str, new_alias: &'a str) -> RenamePrefixBuilder<'a> {
        RenamePrefixBuilder::new(self, old_alias, new_alias)
    }

    /// Returns a `CopyPrefixBuilder` to add `new_alias` to the default
    /// prefixes, associated with the same IRI as `old_alias`.
    pub fn copy_prefix<'a>(&'a mut self, old_alias: &'a str, new_alias: &'a str) -> CopyPrefixBuilder<'a> {
        CopyPrefixBuilder::new(self, old_alias, new_alias)
    }
}
