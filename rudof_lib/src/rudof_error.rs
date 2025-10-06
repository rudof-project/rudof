use std::io;

use iri_s::IriS;
use shacl_ast::Schema;
use shacl_ir::compiled_shacl_error::CompiledShaclError;
use sparql_service::RdfData;
use srdf::SRDFSparql;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RudofError {
    #[error(transparent)]
    IOErro(#[from] io::Error),
    #[error("Invalid shape label {label}: {error}")]
    InvalidShapeLabel { label: String, error: String },

    #[error("Error obtaining current directory: {error}")]
    CurrentDirError { error: String },

    #[error("Error converting current dir {current_dir} to URL")]
    ConvertingCurrentFolderUrl { current_dir: String },

    #[error("Parsing IRI from {source_name}: {error}")]
    SourceNameIriError { source_name: String, error: String },
    #[error("SPARQL syntax error reading {source_name}: {error}")]
    SparqlSyntaxError { error: String, source_name: String },

    #[error("Invalid endpoint IRI {endpoint}: {error}")]
    InvalidEndpointIri { endpoint: String, error: String },

    #[error("Invalid endpoint {endpoint}: {error}")]
    InvalidEndpoint { endpoint: String, error: String },

    #[error("Error running query against endpoint {endpoint}.\nQuery:\n{query}\nError: {error}")]
    QueryEndpointError {
        endpoint: String,
        error: String,
        query: String,
    },

    #[error("Parsing URL {url} reading service description: {error}")]
    ParsingUrlReadingServiceDescriptionUrl { url: String, error: String },

    #[error("Obtaining content from URL {url} reading service description: {error}")]
    ReadingServiceDescriptionUrl { url: String, error: String },

    #[error("{error}")]
    Generic { error: String },

    #[error("Common Shapes Model conversion error from ShEx: {error}")]
    CoShaMoFromShExError { schema: String, error: String },

    #[error("Invalid compare schema format: {format}: {error}")]
    InvalidCompareSchemaFormat { format: String, error: String },

    #[error("RDF Config read error: {error}")]
    RdfConfigReadError { error: String },

    #[error("Compiling SHACL: {error}")]
    ShaclCompilation { error: Box<CompiledShaclError> },

    #[error("Error reading config file from path {path}: {error}")]
    RudofConfigFromPathError { path: String, error: io::Error },

    #[error("Error reading config file from path {path}: {error}")]
    RudofConfigTomlError {
        path: String,
        error: toml::de::Error,
    },

    #[error("Error running query {str}: {error}")]
    QueryError { str: String, error: String },

    #[error("Storage error: {error}")]
    StorageError { error: String },

    #[error("Error parsing IRI from {str}: {error}")]
    BaseIriError { str: String, error: String },

    #[error("ShEx compact parser error from {source_name}: {error}")]
    ShExCParserError { error: String, source_name: String },

    #[error("ShEx JSON parser error from {source_name}: {error}")]
    ShExJParserError { error: String, source_name: String },

    #[error("Compiling schema error: {error}")]
    CompilingSchemaError { error: String },

    #[error(
        "ShEx Validator undefined. Before trying to validate with ShEx, a ShEx validator must be initialized in rudof"
    )]
    ShExValidatorUndefined {},

    #[error("Error creating schema for ShEx validation. Schema:\n{schema}\nError: {error} ")]
    ShExValidatorCreationError { schema: String, error: String },

    #[error("ShEx validation error: {error} ")]
    ShExValidatorError {
        schema: String,
        rdf_data: String,
        query_map: String,
        error: String,
    },

    #[error("ShEx validation error. Error: {error} ")]
    ShExValidatorObtainingResultMapError {
        schema: String,
        rdf_data: String,
        shapemap: String,
        error: String,
    },

    #[error(
        "Error merging current RDF data, format: {format}, base: {base}, reader_mode: {reader_mode}: {error} "
    )]
    MergeRDFDataFromReader {
        format: String,
        base: String,
        reader_mode: String,
        error: String,
    },

    #[error("Utf8 error: {error} ")]
    Utf8Error { error: String },

    #[error("Shapemap parse error on str: {str}: {error}")]
    ShapeMapParseError { str: String, error: String },

    #[error("Read error: {error} ")]
    ReadError { error: String },

    #[error("AddingEndpoint: {iri} ")]
    AddingEndpointError { iri: IriS, error: String },

    #[error("Validating shex requires to initialize a shapemap or a node/shape pair")]
    NoShapeMap { schema: String },

    #[error("Resolving imports in ShEx schema: {error}")]
    ResolvingImportsShExSchema { error: String },

    #[error("Attempt to resolve import declarations without defining ShEx schema")]
    NoShExSchemaForResolvingImports,

    #[error("Internal SHACL Format is not readable. Only for output")]
    InternalSHACLFormatNonReadable,

    #[error("SHACL Parser error: {error}")]
    SHACLParseError { error: String },

    #[error("SHACL Compilation from schema {schema} error: {error}")]
    SHACLCompilationError {
        error: String,
        schema: Box<Schema<RdfData>>,
    },

    #[error("SHACL Validation from schema {schema} error: {error}")]
    SHACLValidationError {
        error: String,
        schema: Box<Schema<RdfData>>,
    },

    #[error("Creating Endpoint validation for SHACL from endpoint {endpoint:?}. error: {error}")]
    SHACLEndpointValidationCreation {
        error: String,
        endpoint: Box<SRDFSparql>,
    },

    #[error("Parsing RDF data error: {error}")]
    ParsingRDFDataReader { error: String },

    #[error("No graph and no first endpoint to validate SHACL")]
    NoGraphNoFirstEndpoint,

    #[error("No SHACL schema defined")]
    NoShaclSchema,

    #[error("Cannot serialize current ShEx schema because it has not been defined")]
    NoShExSchemaToSerialize,

    #[error("No DCTAP defined")]
    NoDCTAP,

    #[error("ShEx2UML: No ShEx schema")]
    ShEx2UmlWithoutShEx,

    #[error("ShEx2PlantUML Error: {error}")]
    ShEx2PlantUmlError { error: String },

    #[error("RDF2PlantUML Error: {error}")]
    RDF2PlantUmlError { error: String },

    #[error("ShEx2PlantUML Error when generating PlantUML: {error}")]
    ShEx2PlantUmlErrorAsPlantUML { error: String },

    #[error("RDF2PlantUML Error when generating PlantUML: {error}")]
    RDF2PlantUmlErrorAsPlantUML { error: String },

    #[error("Reading ShEx Schema from path: {path}: {error}")]
    ReadingShExPath { path: String, error: String },

    #[error("Reading {context} from {url}: {error}")]
    ReadingUrlContext {
        url: String,
        error: String,
        context: String,
    },

    #[error("Obtaining {context} from input {input}: {error}")]
    ParsingInputSpecContext {
        input: String,
        error: String,
        context: String,
    },

    #[error("No SPARQL query has been defined")]
    NoCurrentSPARQLQuery,

    #[error("The current SPARQL query is not a SELECT query, it is:\n{query}")]
    NotSelectQuery { query: String },

    #[error("The current SPARQL query is not a SELECT or CONSTRUCT query, it is:\n{query}")]
    NotConstructQuery { query: String },

    #[error("Reading {context} from input {input}: {error}")]
    ReadingInputSpecContext {
        input: String,
        error: String,
        context: String,
    },

    #[error("Reading {context}. Parsing {url}: {error}")]
    ParsingUrlContext {
        url: String,
        error: String,
        context: String,
    },

    #[error("Reading {context} from path: {path}: {error}")]
    ReadingPathContext {
        path: String,
        error: String,
        context: String,
    },

    #[error("Error formatting schema {schema}: {error}")]
    ErrorFormattingSchema { schema: String, error: String },

    #[error("Error formatting shapemap {shapemap}: {error}")]
    ErrorFormattingShapeMap { shapemap: String, error: String },

    #[error("Error formatting schema: {error}")]
    ErrorWritingShExJson { schema: String, error: String },

    #[error("Not implemented yet: {msg}")]
    NotImplemented { msg: String },

    #[error("Cannot serialize current ShapeMap because it has not been defined")]
    NoShapeMapToSerialize,

    #[error("Cannot serialize current SHACL because it has not been defined")]
    NoShaclToSerialize,

    #[error("Converting SHACLFormat with value Internal to RDFFormat")]
    NoInternalFormatForRDF,

    #[error("Serializing SHACL to internal representation: {error}")]
    SerializingSHACLInternal { error: String },

    #[error("Writing SHACL {shacl}: {error}")]
    WritingSHACL { shacl: String, error: String },

    #[error("Serializing SHACL {shacl}: {error}")]
    SerializingSHACL { shacl: String, error: String },

    #[error("Serializing ShEx: {error}")]
    SerializingShEx { error: String },

    #[error("Serializing RDF data: {error}")]
    SerializingData { error: String },

    #[error("Serializing ShEx: {error}")]
    SerializingShacl { error: String },

    #[error("DCTAP reader from path {path} in CSV format: {error}")]
    DCTAPReaderCSV { error: String, path: String },

    #[error("DCTAP reader from path {path}: {error}")]
    ReadingDCTAPPath { error: String, path: String },

    #[error("DCTAP reader in CSV format: {error}")]
    DCTAPReaderCSVReader { error: String },

    #[error("DCTAP reader from path {path} in CSV format: {error}")]
    DCTAPReaderPathXLS {
        error: String,
        path: String,
        format: String,
    },

    #[error("Reading DCTAP from XLS format requires a Path, use read_dctap_path")]
    DCTAPReadXLSNoPath,

    #[error("Error converting DCTAP to ShEx")]
    DCTap2ShEx { error: String },

    #[error("Serializing Service Description: {error}")]
    SerializingServiceDescription { error: String },

    #[error("Cannot serialize current Service Description because it has not been defined")]
    NoServiceDescriptionToSerialize,

    #[error("Reading Service Description: {error}")]
    ReadingServiceDescription { error: String },

    #[error("Reading Service Description from path {path}: {error}")]
    ReadingServiceDescriptionPath { path: String, error: String },
}
