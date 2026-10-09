use crate::{
    Result, Rudof,
    errors::DataError,
    formats::IriNormalizationMode,
    utils::{check_data_is_mutable, parse_triple, rdf_data, rdf_data_mut},
};
use rudof_rdf::rdf_core::BuildRDF;

/// Adds a triple to the current RDF data.
pub fn add_triple(
    rudof: &mut Rudof,
    subject: &str,
    predicate: &str,
    object: &str,
    iri_mode: IriNormalizationMode,
) -> Result<()> {
    check_data_is_mutable(rdf_data(rudof)?)?;
    let (subject, predicate, object) = parse_triple(rudof, subject, predicate, object, iri_mode)?;

    rdf_data_mut(rudof)?
        .add_triple(subject, predicate, object)
        .map_err(|error| {
            Box::new(DataError::FailedTripleUpdate {
                operation: "add".to_string(),
                error: error.to_string(),
            })
        })?;
    Ok(())
}
