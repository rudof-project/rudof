use crate::{QueryResultFormat, SRDFSparqlError};
use iri_s::IriS;
use sparesults::QuerySolution;

#[derive(Debug, Clone)]
pub(super) struct Client;

pub(super) fn sparql_client() -> Result<Client, SRDFSparqlError> {
    Ok(Client)
}
pub(super) fn sparql_client_construct_jsonld() -> Result<Client, SRDFSparqlError> {
    Ok(Client)
}
pub(super) fn sparql_client_construct_rdfxml() -> Result<Client, SRDFSparqlError> {
    Ok(Client)
}
pub(super) fn sparql_client_construct_turtle() -> Result<Client, SRDFSparqlError> {
    Ok(Client)
}

pub(super) fn make_sparql_query_select(_: &str, _: &Client, _: &IriS) -> Result<Vec<QuerySolution>, SRDFSparqlError> {
    Err(SRDFSparqlError::WASMError {
        fn_name: String::from("make_sparql_query_select"),
    })
}

pub(super) fn make_sparql_query_construct(
    _: &str,
    _: &Client,
    _: &IriS,
    _: &QueryResultFormat,
) -> Result<String, SRDFSparqlError> {
    Err(SRDFSparqlError::WASMError {
        fn_name: String::from("make_sparql_query_construct"),
    })
}
