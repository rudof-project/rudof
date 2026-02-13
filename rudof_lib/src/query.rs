use crate::{
    InputSpec, RdfData, Rudof, RudofError, query_result_format::ResultQueryFormat,
    query_type::QueryType,
};
use rdf::rdf_core::query::{QueryResultFormat, QuerySolutions};
use std::io::Write;
use tracing::trace;

#[allow(clippy::too_many_arguments)]
pub fn execute_query<W: Write>(
    rudof: &mut Rudof,
    query: &InputSpec,
    query_type: &QueryType,
    result_query_format: &ResultQueryFormat,
    writer: &mut W,
) -> Result<(), RudofError> {
    let mut reader = query
        .open_read(None, "Query")
        .map_err(|e| RudofError::ReadError {
            error: format!("Failed to open query: {}", e),
        })?;
    match query_type {
        QueryType::Select => {
            trace!("Running SELECT query");
            let results = rudof.run_query_select(&mut reader)?;
            show_results(writer, &results, result_query_format)?;
        }
        QueryType::Construct => {
            let query_format = cnv_query_format(result_query_format);
            let str = rudof.run_query_construct(&mut reader, &query_format)?;
            writeln!(writer, "{str}")?;
        }
        QueryType::Ask => {
            // let bool = rudof.run_query_ask(&mut reader)?;
            // writeln!(writer, "{bool}")?;
            return Err(RudofError::NotImplemented {
                msg: "ASK queries".to_string(),
            });
        }
        QueryType::Describe => {
            return Err(RudofError::NotImplemented {
                msg: "DESCRIBE queries".to_string(),
            });
        }
    }
    Ok(())
}

fn show_results(
    writer: &mut dyn Write,
    results: &QuerySolutions<RdfData>,
    result_query_format: &ResultQueryFormat,
) -> Result<(), RudofError> {
    match result_query_format {
        ResultQueryFormat::Internal => {
            results
                .write_table(writer)
                .map_err(|e| RudofError::QueryError {
                    str: "write_table".to_string(),
                    error: format!("{}", e),
                })?;
        }
        _ => {
            todo!()
        }
    }
    Ok(())
}

fn cnv_query_format(format: &ResultQueryFormat) -> QueryResultFormat {
    match format {
        ResultQueryFormat::Internal => QueryResultFormat::Turtle,
        ResultQueryFormat::NTriples => QueryResultFormat::NTriples,
        ResultQueryFormat::JsonLd => QueryResultFormat::JsonLd,
        ResultQueryFormat::RdfXml => QueryResultFormat::RdfXml,
        ResultQueryFormat::Csv => QueryResultFormat::Csv,
        ResultQueryFormat::TriG => QueryResultFormat::TriG,
        ResultQueryFormat::N3 => QueryResultFormat::N3,
        ResultQueryFormat::NQuads => QueryResultFormat::NQuads,
        ResultQueryFormat::Turtle => QueryResultFormat::Turtle,
    }
}

// Detect query type from SPARQL string
pub fn detect_query_type(query: &str) -> Option<String> {
    let query_upper = query.trim().to_uppercase();

    let query_cleaned: String = query_upper
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join(" ");

    if query_cleaned.contains("SELECT") {
        Some("select".to_string())
    } else if query_cleaned.contains("CONSTRUCT") {
        Some("construct".to_string())
    } else if query_cleaned.contains("ASK") {
        Some("ask".to_string())
    } else if query_cleaned.contains("DESCRIBE") {
        Some("describe".to_string())
    } else {
        None
    }
}
