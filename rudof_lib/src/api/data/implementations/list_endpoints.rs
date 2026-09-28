use crate::{Result, Rudof, errors::DataError};

pub fn list_endpoints(rudof: &mut Rudof) -> Result<Vec<(String, String)>> {
    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoDataLoaded))?;

    if !data.is_rdf() {
        Err(Box::new(DataError::NoRdfDataLoaded))?
    }

    // SPARQL endpoints are not available on wasm, so none can be registered.
    #[cfg(target_family = "wasm")]
    let endpoints = Vec::new();
    #[cfg(not(target_family = "wasm"))]
    let endpoints = data
        .unwrap_rdf_mut()
        .endpoints()
        .iter()
        .map(|(name, endpoint)| (name.clone(), endpoint.iri().to_string()))
        .collect();

    Ok(endpoints)
}
