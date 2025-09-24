use spargebra::{Query, SparqlSyntaxError};
// use srdf::QueryRDF;
use std::str::FromStr;
use thiserror::Error;

// TODO: This code is just a stub for now, to be expanded later.
// The goal is to create a wrapper for SPARQL queries that doesn't require to parse them each time they are run

/// A SPARQL query with its source (for error reporting)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SparqlQuery {
    pub source: String,
    pub query: Query,
}

impl SparqlQuery {
    /*   pub fn run_query<RDF: QueryRDF>(&self, rdf: &RDF) -> Result<QueryResults, SparqlQueryError> {
        rdf.execute_query(&self.query)
            .map_err(|e| SparqlQueryError::ParseError { error: e })
    } */
}

impl FromStr for SparqlQuery {
    type Err = spargebra::SparqlSyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let query = s.parse()?;
        Ok(SparqlQuery {
            source: s.to_string(),
            query,
        })
    }
}

#[derive(Error, Debug)]
pub enum SparqlQueryError {
    #[error("Error parsing SPARQL query: {error}")]
    ParseError { error: SparqlSyntaxError },
}
