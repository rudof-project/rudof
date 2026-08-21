use crate::{
    Result, Rudof,
    errors::DataError,
    formats::{DataFormat, DataReaderMode},
    types::Data,
};
use reqwest::blocking::ClientBuilder;
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderValue, USER_AGENT};
use sparql_service::RdfData;

/// Accept header advertising every RDF serialization Rudof can parse, in
/// order of preference, so a Linked Data server can content-negotiate a
/// format we can actually read.
const RDF_ACCEPT: &str = "text/turtle, application/rdf+xml;q=0.9, application/ld+json;q=0.8, \
                          application/trig;q=0.7, application/n-quads;q=0.6, application/n-triples;q=0.5, \
                          text/n3;q=0.4, */*;q=0.1";

/// Dereferences `uri` over HTTP(S), content-negotiating for an RDF
/// serialization, following redirects, and merging the resulting triples
/// into the current RDF data.
pub fn dereference(
    rudof: &mut Rudof,
    uri: &str,
    reader_mode: Option<&DataReaderMode>,
    merge: Option<bool>,
) -> Result<()> {
    let reader_mode = reader_mode.copied().unwrap_or_default();
    let merge = merge.unwrap_or(true);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static(RDF_ACCEPT));
    headers.insert(USER_AGENT, HeaderValue::from_static("rudof"));

    let client = ClientBuilder::new()
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|error| {
            Box::new(DataError::DereferenceError {
                uri: uri.to_string(),
                error: format!("failed to build HTTP client: {error}"),
            })
        })?;

    let response = client.get(uri).send().map_err(|error| {
        Box::new(DataError::DereferenceError {
            uri: uri.to_string(),
            error: error.to_string(),
        })
    })?;

    let final_url = response.url().to_string();

    if !response.status().is_success() {
        return Err(Box::new(DataError::DereferenceHttpStatus {
            uri: uri.to_string(),
            status: response.status().to_string(),
        })
        .into());
    }

    let data_format = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(DataFormat::from_mime_type)
        .unwrap_or_default();

    let body = response.text().map_err(|error| {
        Box::new(DataError::DereferenceError {
            uri: uri.to_string(),
            error: error.to_string(),
        })
    })?;

    if !merge || rudof.data.is_none() || matches!(rudof.data, Some(ref data) if data.is_pg()) {
        let rdf_data = RdfData::new()
            .with_rdf_data_config(rudof.config.rdf_data())
            .map_err(|error| {
                Box::new(DataError::RdfDataConfig {
                    error: error.to_string(),
                })
            })?;
        rudof.data = Some(Data::RDFData(Box::new(rdf_data)));
    }

    rudof
        .data
        .as_mut()
        .unwrap()
        .unwrap_rdf_mut()
        .merge_from_reader(
            &mut body.as_bytes(),
            &final_url,
            &data_format.try_into()?,
            Some(final_url.as_str()),
            &reader_mode.into(),
        )
        .map_err(|error| {
            Box::new(DataError::FailedParsingRdfData {
                source_name: final_url.clone(),
                format: data_format.to_string(),
                base: final_url,
                reader_mode: reader_mode.to_string(),
                error: error.to_string(),
            })
        })?;

    Ok(())
}
