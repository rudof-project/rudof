use crate::{
    Result, Rudof,
    errors::DataError,
    formats::IriNormalizationMode,
    types::Triples,
    utils::{parse_predicate_iri, parse_subject, parse_term, rdf_data, resolution_prefixmap, session_base_iri},
};
use rudof_rdf::rdf_core::{AnyOr, NeighsRDF};

/// Returns an iterator over the triples of the current RDF data matching a pattern.
pub fn triples<'a>(
    rudof: &'a Rudof,
    subject: Option<&str>,
    predicate: Option<&str>,
    object: Option<&str>,
    iri_mode: IriNormalizationMode,
) -> Result<Triples<'a>> {
    let rdf = rdf_data(rudof)?;
    let prefixmap = resolution_prefixmap(rudof, rdf);
    let base = session_base_iri(rudof)?;

    let subject = subject
        .map(|subject| parse_subject(subject, base.as_ref(), &prefixmap, iri_mode))
        .transpose()?;
    let predicate = predicate
        .map(|predicate| parse_predicate_iri(predicate, &prefixmap))
        .transpose()?
        .map(Into::into);
    let object = object
        .map(|object| parse_term(object, base.as_ref(), &prefixmap, iri_mode))
        .transpose()?;

    let triples = rdf
        .triples_matching(&AnyOr::new(subject), &AnyOr::new(predicate), &AnyOr::new(object))
        .map_err(|error| {
            Box::new(DataError::FailedTripleRetrieval {
                error: error.to_string(),
            })
        })?
        .collect();

    Ok(Triples::new(rdf, triples))
}
