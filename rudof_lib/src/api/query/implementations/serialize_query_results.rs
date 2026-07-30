use rudof_rdf::rdf_core::query::{TableFormat, TableOptions};

use crate::{
    Result, Rudof,
    errors::{QueryError, RudofError},
    formats::ResultQueryFormat,
    types::QueryResult,
};
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
            ResultQueryFormat::Internal => results
                .write_table(writer, &TableFormat::default(), &TableOptions::default())
                .map_err(|error| QueryError::FailedSerializingQueryResults {
                    error: error.to_string(),
                })?,
            ResultQueryFormat::Csv => results
                .write_table(writer, &TableFormat::Csv, &TableOptions::default())
                .map_err(|error| QueryError::FailedSerializingQueryResults {
                    error: error.to_string(),
                })?,
            ResultQueryFormat::Markdown => results
                .write_table(writer, &TableFormat::Markdown, &TableOptions::default())
                .map_err(|error| QueryError::FailedSerializingQueryResults {
                    error: error.to_string(),
                })?,
            ResultQueryFormat::Json => {
                serde_json::to_writer(writer, results).map_err(|error| QueryError::FailedSerializingQueryResults {
                    error: error.to_string(),
                })?
            },
            _ => Err(RudofError::UnsupportedResultQueryFormatSelect {
                format: result_query_format.to_string(),
                formats: [
                    ResultQueryFormat::Internal,
                    ResultQueryFormat::Csv,
                    ResultQueryFormat::Markdown,
                    ResultQueryFormat::Json,
                ]
                .iter()
                .map(|a| a.to_string())
                .collect(),
            })?,
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
