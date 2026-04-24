use crate::{Result, Rudof, errors::IriError, formats::QueryType};
use crossterm::terminal;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::query::SparqlQuery;
use std::{env, str::FromStr};
#[cfg(not(target_family = "wasm"))]
use url::Url;

pub fn get_base_iri(rudof: &mut Rudof, base_iri: Option<&str>) -> Result<IriS> {
    if let Some(base_iri) = base_iri {
        let base_iri = IriS::from_str(base_iri).map_err(|error| IriError::ParseError {
            iri: base_iri.to_string(),
            error: error.to_string(),
        })?;

        Ok(base_iri.clone())
    } else if let Some(base_iri) = rudof.config.shex_config().base.as_ref() {
        Ok(base_iri.clone())
    } else {
        #[cfg(target_family = "wasm")]
        return Err(RudofError::WASMError(
            "Base IRI must be provided in WASM environment".to_string(),
        ));
        #[cfg(not(target_family = "wasm"))]
        {
            let cwd = env::current_dir().map_err(|e| IriError::PathConversionError {
                path: ".".to_string(),
                error: format!("Error resolving base IRI. Failed to get current directory: {e}"),
            })?;

            let url = Url::from_directory_path(&cwd).map_err(|_| IriError::PathConversionError {
                path: cwd.to_string_lossy().to_string(),
                error: "Error resolving base IRI. Cannot convert current directory to a file URL".to_string(),
            })?;
            Ok(url.into())
        }
    }
}

const MAX_TERMINAL_WIDTH: usize = 100;
const DEFAULT_TERMINAL_WIDTH: usize = 80;

pub fn terminal_width() -> usize {
    if let Ok((cols, _)) = terminal::size() {
        sanitize_width(cols as usize)
    } else {
        DEFAULT_TERMINAL_WIDTH
    }
}

fn sanitize_width(width: usize) -> usize {
    match width {
        w if w > MAX_TERMINAL_WIDTH => MAX_TERMINAL_WIDTH,
        w if w < 40 => DEFAULT_TERMINAL_WIDTH,
        w => w,
    }
}

// Detect query type from SPARQL string
pub fn detect_query_type(query: &SparqlQuery) -> QueryType {
    if query.is_select() {
        QueryType::Select
    } else if query.is_construct() {
        QueryType::Construct
    } else if query.is_ask() {
        QueryType::Ask
    } else {
        QueryType::Describe
    }
}
