/// Represents a SPARQL query
pub struct SparqlQuery {
    source: String,
}

impl SparqlQuery {
    /// Creates a new `SparqlQuery` from a query string
    pub fn new(source: &str) -> Self {
        SparqlQuery {
            source: source.to_string(),
        }
    }

    /// Returns the SPARQL query string
    pub fn source(&self) -> &str {
        &self.source
    }
}
