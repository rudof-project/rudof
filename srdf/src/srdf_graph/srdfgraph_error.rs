use iri_s::IriSError;
use oxiri::IriParseError;
use oxttl::TurtleParseError;
use prefixmap::PrefixMapError;
use std::io;
use std::io::Error as IOError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SRDFGraphError {
    #[error("Query result error: {msg}")]
    QueryResultError { msg: String },

    #[error("Error extending query solutions for query: {query}: {error}")]
    ExtendingQuerySolutionsError { query: String, error: String },

    #[error("Parsing query error: {msg}")]
    ParsingQueryError { msg: String },

    #[error("Running query {query} error: {msg}")]
    RunningQueryError { query: String, msg: String },

    #[error("Error parsing Turtle data from {source_name}: {error}")]
    TurtleParseError { source_name: String, error: String },

    #[error("Parsing base iri {str}: error: {error}")]
    BaseParseError { str: String, error: String },

    #[error("Blank node generation id: {msg}")]
    BlankNodeId { msg: String },

    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingPathError { path_name: String, error: io::Error },

    #[error(transparent)]
    ErrorReadingTurtle {
        #[from]
        err: TurtleParseError,
    },

    #[error(transparent)]
    IOError {
        #[from]
        err: IOError,
    },

    #[error("Turtle error: {turtle_error}\nData:\n{data}")]
    TurtleError {
        data: String,
        turtle_error: TurtleParseError,
    },

    #[error("RDF/XML error: {error}\nData: {data}")]
    RDFXMLError { data: String, error: String },

    #[error("N-Triples error: {error}\nData: {data}")]
    NTriplesError { data: String, error: String },

    #[error("NQuads error: {error}\nData: {data}")]
    NQuadsError { data: String, error: String },

    #[error("JSON-LD error: {error}\nData: {data}")]
    JsonLDError { data: String, error: String },

    #[error(transparent)]
    IriParseError {
        #[from]
        err: IriParseError,
    },

    #[error(transparent)]
    IriSError {
        #[from]
        err: IriSError,
    },

    #[error(transparent)]
    PrefixMapError {
        #[from]
        err: PrefixMapError,
    },

    #[error("Unexepected node type: {node}")]
    UnexepectedNodeType { node: String },

    #[error("Expected node to become a subject")]
    ExpectedSubject,
}
