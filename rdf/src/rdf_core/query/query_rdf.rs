use crate::rdf_core::{Rdf, query::{QueryResultFormat, QuerySolutions}};

/// Trait for RDF graphs that support SPARQL query execution.
///
/// This trait extends [`Rdf`] with methods for executing SPARQL queries against
/// the RDF graph. SPARQL is the W3C standard query language for RDF, providing
/// pattern matching, filtering, and graph construction capabilities.
///
/// # Query Forms
///
/// SPARQL defines four query forms, three of which are supported by this trait:
///
/// - **SELECT**: Pattern matching that returns variable bindings as a table
/// - **CONSTRUCT**: Pattern matching that builds a new RDF graph from templates
/// - **ASK**: Boolean query testing whether a pattern exists in the graph
/// - **DESCRIBE** (not yet supported): Retrieves RDF data about resources
pub trait QueryRDF: Rdf {
    /// Executes a SPARQL SELECT query and returns variable bindings.
    ///
    /// SELECT queries match patterns in the RDF graph and return a table-like
    /// structure containing variable bindings for each match. This is the most
    /// common SPARQL query form, analogous to SELECT in SQL.
    ///
    /// # Arguments
    ///
    /// * `query` - A SPARQL SELECT query string
    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>, Self::Err>
    where
        Self: Sized;

    /// Executes a SPARQL CONSTRUCT query and returns a serialized RDF graph.
    ///
    /// CONSTRUCT queries match patterns in the RDF graph and use those matches
    /// to build a new RDF graph according to a template. The resulting graph
    /// is serialized to a string in the specified format.
    ///
    /// # Arguments
    ///
    /// * `query` - A SPARQL CONSTRUCT query string
    /// * `result_format` - The serialization format for the resulting RDF graph
    ///   (e.g., Turtle, RDF/XML, N-Triples)
    fn query_construct(
        &self,
        query: &str,
        result_format: &QueryResultFormat,
    ) -> Result<String, Self::Err>
    where
        Self: Sized;

    /// Executes a SPARQL ASK query and returns a boolean result.
    ///
    /// ASK queries test whether a pattern exists in the RDF graph. They return
    /// `true` if at least one solution to the query pattern exists, `false`
    /// otherwise. This is more efficient than SELECT when only existence needs
    /// to be checked.
    ///
    /// # Arguments
    ///
    /// * `query` - A SPARQL ASK query string 
    fn query_ask(&self, query: &str) -> Result<bool, Self::Err>;
}
