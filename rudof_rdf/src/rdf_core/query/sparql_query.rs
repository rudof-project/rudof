use spargebra::{Query, SparqlParser, SparqlSyntaxError};
use std::fmt::Display;

/// Represents a parsed and validated SPARQL query.
///
/// This type wraps a SPARQL query string along with its parsed abstract syntax
/// tree (AST). It provides methods for query introspection, serialization, and
/// type checking.
///
/// # Parsing
///
/// Queries are parsed using the `spargebra` SPARQL parser, which validates
/// syntax according to the SPARQL 1.1 specification. Invalid queries will
/// fail at construction time with a [`SparqlSyntaxError`].
///
/// # Preserved Information
///
/// Both the original query string and the parsed AST are preserved, allowing:
/// - Access to the exact original query text
/// - Structured access to the parsed query components
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SparqlQuery {
    /// The original SPARQL query string as provided.
    source: String,
    /// The parsed abstract syntax tree of the query.
    query: Query,
}

impl SparqlQuery {
    /// Creates a new SPARQL query by parsing and validating the input string.
    ///
    /// This constructor parses the query string according to SPARQL 1.1 syntax
    /// rules and validates its structure. If parsing succeeds, both the original
    /// string and the parsed AST are stored for later use.
    ///
    /// # Arguments
    ///
    /// * `source` - A SPARQL query string
    pub fn new(source: &str) -> Result<Self, SparqlSyntaxError> {
        let query = SparqlParser::new().parse_query(source)?;
        Ok(SparqlQuery {
            source: source.to_string(),
            query,
        })
    }

    /// Returns the original SPARQL query string
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Serializes the query to its canonical SPARQL string form
    ///
    /// Unlike [`source()`](Self::source), this returns a canonicalized version
    /// that may differ from the original in formatting but is semantically
    /// equivalent.
    pub fn serialize(&self) -> String {
        self.query.to_string()
    }

    /// Checks if this is a SELECT query.
    pub fn is_select(&self) -> bool {
        matches!(self.query, Query::Select { .. })
    }

    /// Checks if this is a CONSTRUCT query.
    pub fn is_construct(&self) -> bool {
        matches!(self.query, Query::Construct { .. })
    }

    /// Checks if this is an ASK query.
    pub fn is_ask(&self) -> bool {
        matches!(self.query, Query::Ask { .. })
    }

    /// Checks if this is a DESCRIBE query.
    pub fn is_describe(&self) -> bool {
        matches!(self.query, Query::Describe { .. })
    }
}

impl Display for SparqlQuery {
    /// Formats the query using its canonical serialized form.
    ///
    /// This delegates to [`serialize()`](Self::serialize), producing a
    /// normalized representation of the query suitable for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}
