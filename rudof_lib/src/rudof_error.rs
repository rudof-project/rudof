use std::io;

use iri_s::IriS;
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
}
