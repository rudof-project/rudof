use rudof_rdf::rdf_core::query::QueryResultFormat;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::errors::QueryError;

/// Output formats for SPARQL query results supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ResultQueryFormat {
    /// Internal format - internal representation for processing
    Internal,
    /// Turtle - compact RDF format (for CONSTRUCT/DESCRIBE)
    Turtle,
    /// N-Triples - simple line-based RDF format
    NTriples,
    /// JSON-LD - JSON format for Linked Data (for CONSTRUCT/DESCRIBE)
    JsonLd,
    /// RDF/XML - XML-based RDF serialization (for CONSTRUCT/DESCRIBE)
    RdfXml,
    /// CSV - comma-separated values (for SELECT)
    Csv,
    /// TriG - Turtle with named graphs (for CONSTRUCT/DESCRIBE)
    TriG,
    /// Notation3 - superset of Turtle (for CONSTRUCT/DESCRIBE)
    N3,
    /// N-Quads - N-Triples with named graphs (for CONSTRUCT/DESCRIBE)
    NQuads,
}

/// SPARQL query types supported by Rudof.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum QueryType {
    /// SELECT - returns tabular results (variable bindings)
    Select,
    /// CONSTRUCT - returns an RDF graph constructed from query results
    Construct,
    /// ASK - returns a boolean (true/false) result
    Ask,
    /// DESCRIBE - returns an RDF graph describing resources
    Describe,
}

// ============================================================================
// ResultQueryFormat
// ============================================================================

impl Display for ResultQueryFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultQueryFormat::Internal => write!(dest, "internal"),
            ResultQueryFormat::Turtle => write!(dest, "turtle"),
            ResultQueryFormat::NTriples => write!(dest, "ntriples"),
            ResultQueryFormat::JsonLd => write!(dest, "json-ld"),
            ResultQueryFormat::RdfXml => write!(dest, "rdf-xml"),
            ResultQueryFormat::Csv => write!(dest, "csv"),
            ResultQueryFormat::TriG => write!(dest, "trig"),
            ResultQueryFormat::N3 => write!(dest, "n3"),
            ResultQueryFormat::NQuads => write!(dest, "nquads"),
        }
    }
}

impl From<ResultQueryFormat> for QueryResultFormat {
    fn from(format: ResultQueryFormat) -> Self {
        match format {
            ResultQueryFormat::Internal => QueryResultFormat::Turtle,
            ResultQueryFormat::Turtle => QueryResultFormat::Turtle,
            ResultQueryFormat::NTriples => QueryResultFormat::NTriples,
            ResultQueryFormat::JsonLd => QueryResultFormat::JsonLd,
            ResultQueryFormat::RdfXml => QueryResultFormat::RdfXml,
            ResultQueryFormat::Csv => QueryResultFormat::Csv,
            ResultQueryFormat::TriG => QueryResultFormat::TriG,
            ResultQueryFormat::N3 => QueryResultFormat::N3,
            ResultQueryFormat::NQuads => QueryResultFormat::NQuads,
        }
    }
}

impl FromStr for ResultQueryFormat {
    type Err = QueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ResultQueryFormat::Internal),
            "turtle" => Ok(ResultQueryFormat::Turtle),
            "ntriples" => Ok(ResultQueryFormat::NTriples),
            "json-ld" => Ok(ResultQueryFormat::JsonLd),
            "rdf-xml" => Ok(ResultQueryFormat::RdfXml),
            "csv" => Ok(ResultQueryFormat::Csv),
            "trig" => Ok(ResultQueryFormat::TriG),
            "n3" => Ok(ResultQueryFormat::N3),
            "nquads" => Ok(ResultQueryFormat::NQuads),
            other => Err(QueryError::UnsupportedResultQueryFormat {
                format: other.to_string(),
            }),
        }
    }
}

// ============================================================================
// QueryType
// ============================================================================

impl Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            QueryType::Select => "select",
            QueryType::Construct => "construct",
            QueryType::Ask => "ask",
            QueryType::Describe => "describe",
        };
        write!(f, "{s}")
    }
}

impl FromStr for QueryType {
    type Err = QueryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "select" => Ok(QueryType::Select),
            "construct" => Ok(QueryType::Construct),
            "ask" => Ok(QueryType::Ask),
            "describe" => Ok(QueryType::Describe),
            other => Err(QueryError::UnsupportedQueryType {
                query_type: other.to_string(),
            }),
        }
    }
}
