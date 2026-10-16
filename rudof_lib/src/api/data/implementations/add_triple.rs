use crate::{Result, Rudof, api::data::implementations::add_triples, formats::IriNormalizationMode};

/// Adds a triple to the current RDF data.
pub fn add_triple(
    rudof: &mut Rudof,
    subject: &str,
    predicate: &str,
    object: &str,
    iri_mode: IriNormalizationMode,
) -> Result<()> {
    add_triples(rudof, &[(subject, predicate, object)], iri_mode)
}
