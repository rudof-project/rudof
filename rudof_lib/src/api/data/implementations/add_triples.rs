use crate::{
    Result, Rudof,
    api::data::staging::journal_added,
    errors::DataError,
    formats::IriNormalizationMode,
    utils::{check_data_is_mutable, parse_triples, rdf_data, rdf_data_mut_unjournaled},
};
use oxrdf::Triple as OxTriple;
use rudof_rdf::rdf_core::{AnyOr, BuildRDF, NeighsRDF};

/// Adds several triples to the current RDF data in one operation.
pub fn add_triples(rudof: &mut Rudof, triples: &[(&str, &str, &str)], iri_mode: IriNormalizationMode) -> Result<()> {
    check_data_is_mutable(rdf_data(rudof)?)?;
    let triples = parse_triples(rudof, triples, iri_mode)?;
    let added = apply_additions(rudof, triples)?;
    journal_added(rudof, added);
    Ok(())
}

/// Adds `triples` to the loaded RDF data, returning the ones that were not in it
/// already (the ones whose addition is an actual change, and so the ones a rollback
/// has to undo).
///
/// A triple the data already holds is left alone instead of being inserted again: an
/// RDF graph is a set.
pub(crate) fn apply_additions(rudof: &mut Rudof, triples: Vec<OxTriple>) -> Result<Vec<OxTriple>> {
    let rdf = rdf_data_mut_unjournaled(rudof)?;
    let mut added = Vec::new();

    for triple in triples {
        let present = rdf
            .contains(
                &AnyOr::new(Some(triple.subject.clone())),
                &AnyOr::new(Some(triple.predicate.clone())),
                &AnyOr::new(Some(triple.object.clone())),
            )
            .map_err(|error| {
                Box::new(DataError::FailedTripleUpdate {
                    operation: "add".to_string(),
                    error: error.to_string(),
                })
            })?;
        if present {
            continue;
        }

        rdf.add_triple(triple.subject.clone(), triple.predicate.clone(), triple.object.clone())
            .map_err(|error| {
                Box::new(DataError::FailedTripleUpdate {
                    operation: "add".to_string(),
                    error: error.to_string(),
                })
            })?;
        added.push(triple);
    }

    Ok(added)
}
