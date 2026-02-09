use crate::{QueryResultFormat, QuerySolutions, Rdf};

/// Represents RDF that supports SPARQL-like queries
pub trait QueryRDF: Rdf {
    /// SPARQL SELECT query
    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>, Self::Err>
    where
        Self: Sized;

    /// SPARQL CONSTRUCT query
    fn query_construct(&self, query: &str, result_format: &QueryResultFormat) -> Result<String, Self::Err>
    where
        Self: Sized;

    /// SPARQL ASK query
    fn query_ask(&self, query: &str) -> Result<bool, Self::Err>;
}
