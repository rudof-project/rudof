use crate::{Result, Rudof, api::data::implementations::remove_triples, formats::IriNormalizationMode};

/// Removes a triple from the current RDF data.
pub fn remove_triple(
    rudof: &mut Rudof,
    subject: &str,
    predicate: &str,
    object: &str,
    iri_mode: IriNormalizationMode,
) -> Result<()> {
    remove_triples(rudof, &[(subject, predicate, object)], iri_mode)
}
