use std::fmt::Display;

/// Represents a SPARQL query
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SparqlQuery {
    source: String,
    query: spargebra::Query,
}

impl SparqlQuery {
    /// Creates a new `SparqlQuery` from a query string
    pub fn new(source: &str) -> Result<Self, spargebra::SparqlSyntaxError> {
        let query = spargebra::SparqlParser::new().parse_query(source)?;
        Ok(SparqlQuery {
            source: source.to_string(),
            query,
        })
    }

    /// Returns the SPARQL query string
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn serialize(&self) -> String {
        self.query.to_string()
    }

    pub fn is_select(&self) -> bool {
        matches!(self.query, spargebra::Query::Select { .. })
    }

    pub fn is_construct(&self) -> bool {
        matches!(self.query, spargebra::Query::Construct { .. })
    }

    pub fn is_ask(&self) -> bool {
        matches!(self.query, spargebra::Query::Ask { .. })
    }

    pub fn is_describe(&self) -> bool {
        matches!(self.query, spargebra::Query::Describe { .. })
    }
}

impl Display for SparqlQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.serialize())
    }
}
