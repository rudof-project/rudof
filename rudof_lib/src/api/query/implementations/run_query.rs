use rudof_rdf::rdf_core::query::QueryRDF;

use crate::{
    Result, Rudof,
    errors::{DataError, QueryError},
    formats::{QueryType, ResultQueryFormat},
    types::QueryResult,
    utils::detect_query_type,
};

pub fn run_query(rudof: &mut Rudof, result_query_format: Option<&ResultQueryFormat>) -> Result<()> {
    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoRdfDataLoaded))?;
    if !data.is_rdf() {
        return Err(Box::new(DataError::NoRdfDataLoaded).into());
    }

    let query = rudof.query.as_ref().ok_or(QueryError::NoQueryLoaded)?;
    let query_type = detect_query_type(query);
    let result_query_format = result_query_format.copied().unwrap_or(ResultQueryFormat::Turtle);

    data.unwrap_rdf_mut()
        .check_store()
        .map_err(|e| QueryError::FailedInitializingQueryStore { error: e.to_string() })?;

    match query_type {
        QueryType::Select => {
            let results = data
                .unwrap_rdf_mut()
                .query_select(&query.serialize())
                .map_err(|error| QueryError::FailedExecutingQuery {
                    query_type: "select".to_string(),
                    error: error.to_string(),
                })?;

            rudof.query_results = Some(QueryResult::Select(results));
        },
        QueryType::Construct => {
            let results = data
                .unwrap_rdf_mut()
                .query_construct(&query.serialize(), &result_query_format.into())
                .map_err(|error| QueryError::FailedExecutingQuery {
                    query_type: "construct".to_string(),
                    error: error.to_string(),
                })?;

            rudof.query_results = Some(QueryResult::Construct(results));
        },
        QueryType::Ask => {
            let results = data.unwrap_rdf_mut().query_ask(&query.serialize()).map_err(|error| {
                QueryError::FailedExecutingQuery {
                    query_type: "ask".to_string(),
                    error: error.to_string(),
                }
            })?;

            rudof.query_results = Some(QueryResult::Ask(results));
        },
        QueryType::Describe => {
            todo!("Implement DESCRIBE query execution")
        },
    }

    Ok(())
}
