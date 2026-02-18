use iri_s::error::IriSError;
use oxiri::IriParseError;
use oxttl::TurtleParseError;
use prefixmap::PrefixMapError;
use std::io;
use std::io::Error as IOError;
use thiserror::Error;

/// Represents all possible errors that can occur when working with in-memory RDF graphs.
#[derive(Error, Debug)]
pub enum InMemoryGraphError {
    /// Error processing query results.
    ///
    /// # Fields
    /// - `msg`: Detailed description of the query result error
    #[error("Query result error: {msg}")]
    QueryResultError { msg: String },

    /// Error extending query solutions with additional data.
    ///
    /// # Fields
    /// - `query`: The query string being executed
    /// - `error`: Detailed description of the extension failure
    #[error("Error extending query solutions for query: {query}: {error}")]
    ExtendingQuerySolutionsError { query: String, error: String },

    /// Error parsing a query string.
    ///
    /// # Fields
    /// - `msg`: Detailed description of the parsing failure
    #[error("Parsing query error: {msg}")]
    ParsingQueryError { msg: String },

    /// Error executing a query.
    ///
    /// # Fields
    /// - `query`: The query string that failed to execute
    /// - `msg`: Detailed description of the execution failure
    #[error("Running query {query} error: {msg}")]
    RunningQueryError { query: String, msg: String },

    /// Error parsing Turtle RDF data.
    ///
    /// # Fields
    /// - `source_name`: The name or path of the data source
    /// - `error`: Detailed description of the parsing failure
    #[error("Error parsing Turtle data from {source_name}: {error}")]
    TurtleParseError { source_name: String, error: String },

    /// Error parsing a base IRI.
    ///
    /// # Fields
    /// - `str`: The IRI string that failed to parse
    /// - `error`: Detailed description of the parsing failure
    #[error("Parsing base iri {str}: error: {error}")]
    BaseParseError { str: String, error: String },

    /// Error generating a blank node identifier.
    ///
    /// # Fields
    /// - `msg`: Detailed description of the blank node generation failure
    #[error("Blank node generation id: {msg}")]
    BlankNodeId { msg: String },

    /// Error reading data from a file path.
    ///
    /// # Fields
    /// - `path_name`: The path that failed to be read
    /// - `error`: The underlying I/O error
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingPathError { path_name: String, error: io::Error },

    /// Error reading Turtle data.
    ///
    /// # Fields
    /// - `err`: The underlying Turtle parsing error
    #[error(transparent)]
    ErrorReadingTurtle {
        #[from]
        err: TurtleParseError,
    },

    /// General I/O error.
    ///
    /// # Fields
    /// - `err`: The underlying I/O error
    #[error(transparent)]
    IOError {
        #[from]
        err: IOError,
    },

    /// Error parsing Turtle data with context.
    ///
    /// # Fields
    /// - `data`: The Turtle data that failed to parse
    /// - `turtle_error`: The underlying Turtle parsing error
    #[error("Turtle error: {turtle_error}\nData:\n{data}")]
    TurtleError {
        data: String,
        turtle_error: TurtleParseError,
    },

    /// Error parsing RDF/XML data.
    ///
    /// # Fields
    /// - `data`: The RDF/XML data that failed to parse
    /// - `error`: Detailed description of the parsing failure
    #[error("RDF/XML error: {error}\nData: {data}")]
    RDFXMLError { data: String, error: String },

    /// Error parsing N-Triples data.
    ///
    /// # Fields
    /// - `data`: The N-Triples data that failed to parse
    /// - `error`: Detailed description of the parsing failure
    #[error("N-Triples error: {error}\nData: {data}")]
    NTriplesError { data: String, error: String },

    /// Error parsing N-Quads data.
    ///
    /// # Fields
    /// - `data`: The N-Quads data that failed to parse
    /// - `error`: Detailed description of the parsing failure
    #[error("NQuads error: {error}\nData: {data}")]
    NQuadsError { data: String, error: String },

    /// Error parsing JSON-LD data.
    ///
    /// # Fields
    /// - `data`: The JSON-LD data that failed to parse
    /// - `error`: Detailed description of the parsing failure
    #[error("JSON-LD error: {error}\nData: {data}")]
    JsonLDError { data: String, error: String },

    /// Error parsing an IRI.
    ///
    /// # Fields
    /// - `err`: The underlying IRI parsing error
    #[error(transparent)]
    IriParseError {
        #[from]
        err: IriParseError,
    },

    /// Error related to IRI string operations.
    ///
    /// # Fields
    /// - `err`: The underlying IRI string error
    #[error(transparent)]
    IriSError {
        #[from]
        err: IriSError,
    },

    /// Error related to prefix map operations.
    ///
    /// # Fields
    /// - `err`: The underlying prefix map error
    #[error(transparent)]
    PrefixMapError {
        #[from]
        err: PrefixMapError,
    },
}
