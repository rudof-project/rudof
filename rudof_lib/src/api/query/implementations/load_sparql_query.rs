use std::io::Read;

use crate::{
    Result, Rudof,
    errors::QueryError,
    formats::{InputSpec, QueryType},
    utils::{PrefixDirective, default_prefix_header},
};
use rudof_rdf::rdf_core::query::SparqlQuery;

pub fn load_sparql_query(rudof: &mut Rudof, query: &InputSpec, _query_type: Option<&QueryType>) -> Result<()> {
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

    let header = default_prefix_header(rudof, &query_string, PrefixDirective::Sparql);
    let prefixed_query_string = format!("{header}{query_string}");

    let parsed_query = SparqlQuery::new(&prefixed_query_string).map_err(|error| QueryError::FailedParsingQuery {
        source_name: query.source_name().to_string(),
        error: error.to_string(),
    })?;

    rudof.sparql_query = Some(parsed_query);

    Ok(())
}
