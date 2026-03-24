use crate::{Rudof, Result, errors::DataError};

pub fn list_endpoints(rudof: &mut Rudof) -> Result<Vec<(String, String)>> {
    let data = rudof.data.as_mut().ok_or(DataError::NoDataLoaded)?;

    if !data.is_rdf() {
        Err(DataError::NoRdfDataLoaded)?
    }

    let endpoints = data
        .unwrap_rdf_mut()
        .endpoints()
        .iter()
        .map(|(name, endpoint)| (name.clone(), endpoint.iri().to_string()))
        .collect();

    Ok(endpoints)
}
