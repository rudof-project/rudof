use std::io::Read;

use crate::{
    Result, Rudof,
    errors::QueryError,
    formats::{InputSpec, QueryType},
};
use rudof_rdf::rdf_core::query::SparqlQuery;

pub fn load_query(rudof: &mut Rudof, query: &InputSpec, _query_type: Option<&QueryType>) -> Result<()> {
    let mut query_reader = query
        .open_read(None, "SPARQL query")
        .map_err(|error| QueryError::DataSourceSpec {
            message: format!("Failed to open data source '{}': {error}", query.source_name()),
        })?;

    let mut query_string = String::new();
    query_reader
        .read_to_string(&mut query_string)
        .map_err(|error| QueryError::DataSourceSpec {
            message: format!("Failed to read data source '{}': {error}", query.source_name()),
        })?;

    let query = SparqlQuery::new(&query_string).map_err(|error| QueryError::FailedParsingQuery {
        source_name: query.source_name().to_string(),
        error: error.to_string(),
    })?;

    rudof.query = Some(query);

    Ok(())
}
