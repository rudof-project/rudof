use crate::{
    Result, Rudof,
    api::data::staging::journal_removed,
    errors::DataError,
    formats::IriNormalizationMode,
    utils::{check_data_is_mutable, parse_triples, rdf_data, rdf_data_mut_unjournaled},
};
use oxrdf::Triple as OxTriple;
use rudof_rdf::rdf_core::{AnyOr, BuildRDF, NeighsRDF};

/// Removes several triples from the current RDF data in one operation.
pub fn remove_triples(rudof: &mut Rudof, triples: &[(&str, &str, &str)], iri_mode: IriNormalizationMode) -> Result<()> {
    check_data_is_mutable(rdf_data(rudof)?)?;
    let triples = parse_triples(rudof, triples, iri_mode)?;
    let removed = apply_removals(rudof, triples)?;
    journal_removed(rudof, removed);
    Ok(())
}

/// Removes `triples` from the loaded RDF data, returning the ones that were in it
/// (the ones whose removal is an actual change, and so the ones a rollback has to undo).
///
/// A triple the data does not hold is left alone instead of being removed, as in
/// [`apply_additions`](super::apply_additions).
pub(crate) fn apply_removals(rudof: &mut Rudof, triples: Vec<OxTriple>) -> Result<Vec<OxTriple>> {
    let rdf = rdf_data_mut_unjournaled(rudof)?;
    let mut removed = Vec::new();

    for triple in triples {
        let present = rdf
            .contains(
                &AnyOr::new(Some(triple.subject.clone())),
                &AnyOr::new(Some(triple.predicate.clone())),
                &AnyOr::new(Some(triple.object.clone())),
            )
            .map_err(|error| {
                Box::new(DataError::FailedTripleUpdate {
                    operation: "remove".to_string(),
                    error: error.to_string(),
                })
            })?;
        if !present {
            continue;
        }

        rdf.remove_triple(triple.subject.clone(), triple.predicate.clone(), triple.object.clone())
            .map_err(|error| {
                Box::new(DataError::FailedTripleUpdate {
                    operation: "remove".to_string(),
                    error: error.to_string(),
                })
            })?;
        removed.push(triple);
    }

    Ok(removed)
}
