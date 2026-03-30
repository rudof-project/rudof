use crate::{Result, Rudof, errors::QueryError, formats::ResultQueryFormat, types::QueryResult};
use std::io;

pub fn serialize_query_results<W: io::Write>(
    rudof: &Rudof,
    result_query_format: Option<&ResultQueryFormat>,
    writer: &mut W,
) -> Result<()> {
    let query_results = rudof
        .query_results
        .as_ref()
        .ok_or(QueryError::NoQueryResultsAvailable)?;

    let result_query_format = result_query_format.copied().unwrap_or(ResultQueryFormat::Internal);

    match query_results {
        QueryResult::Select(results) => match result_query_format {
            ResultQueryFormat::Internal => {
                results
                    .write_table(writer)
                    .map_err(|error| QueryError::FailedSerializingQueryResults {
                        error: error.to_string(),
                    })?
            },
            _ => todo!("Implement serialization of SELECT query results in formats other than Internal"),
        },
        QueryResult::Construct(results) => {
            writeln!(writer, "{}", results).map_err(|error| QueryError::FailedSerializingQueryResults {
                error: error.to_string(),
            })?
        },
        QueryResult::Ask(result) => {
            writeln!(writer, "{}", result).map_err(|error| QueryError::FailedSerializingQueryResults {
                error: error.to_string(),
            })?
        },
    }

    Ok(())
}
