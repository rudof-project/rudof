use std::io;

use iri_s::IriS;
use shacl_ast::Schema;
use srdf::SRDFSparql;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RudofError {
    #[error("Error reading config file from path {path}: {error}")]
    RudofConfigFromPathError { path: String, error: io::Error },

    #[error("Error reading config file from path {path}: {error}")]
    RudofConfigYamlError {
        path: String,
        error: serde_yml::Error,
    },

    #[error("Error parsing IRI from {str}: {error}")]
    BaseIriError { str: String, error: String },

    #[error("ShEx compact parser error: {error}")]
    ShExCParserError { error: String },

    #[error("ShEx JSON parser error: {error}")]
    ShExJParserError { error: String },

    #[error("Compiling schema error: {error}")]
    CompilingSchemaError { error: String },

    #[error("ShEx Validator undefined. Before trying to validate with ShEx, a ShEx validator must be initialized in rudof")]
    ShExValidatorUndefined {},

    #[error(
        "ShEx validation error. Query map: {query_map}\nSchema:\n{schema}\nData:\n{rdf_data}\nError: {error} "
    )]
    ShExValidatorError {
        schema: String,
        rdf_data: String,
        query_map: String,
        error: String,
    },

    #[error(
        "ShEx validation error. Obtaining result map error: {shapemap}\nSchema:\n{schema}\nData:\n{rdf_data}\nError: {error} "
    )]
    ShExValidatorObtainingResultMapError {
        schema: String,
        rdf_data: String,
        shapemap: String,
        error: String,
    },

    #[error("Error merging current RDF data, format: {format}, base: {base}, reader_mode: {reader_mode}: {error} ")]
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

    #[error("Rsolving imports in ShEx schema: {error}")]
    ResolvingImportsShExSchema { error: String },

    #[error("Attempt to resolve import declarations without defining ShEx schema")]
    NoShExSchemaForResolvingImports,

    #[error("Internal SHACL Format is not readable. Only for output")]
    InternalSHACLFormatNonReadable,

    #[error("SHACL Parser error: {error}")]
    SHACLParseError { error: String },

    #[error("SHACL Compilation from schema {schema} error: {error}")]
    SHACLCompilationError { error: String, schema: Box<Schema> },

    #[error("SHACL Validation from schema {schema} error: {error}")]
    SHACLValidationError { error: String, schema: Box<Schema> },

    #[error("Creating Endpoint validation for SHACL from endpoint {endpoint:?}. error: {error}")]
    SHACLEndpointValidationCreation { error: String, endpoint: SRDFSparql },

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

    #[error("ShEx2PlantUML Error when generating PlantUML: {error}")]
    ShEx2PlantUmlErrorAsPlantUML { error: String },

    #[error("Reading ShEx Schema from path: {path}: {error}")]
    ReadingShExPath { path: String, error: String },

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

    #[error("Serializing ShEx: {error}")]
    SerializingShacl { error: String },
}
