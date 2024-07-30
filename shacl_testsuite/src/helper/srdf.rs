use std::collections::HashSet;

use iri_s::IriS;
use srdf::SRDFBasic;
use srdf::SRDF;

use super::helper_error::HelperError;

pub(crate) fn get_objects_for<S: SRDF + SRDFBasic>(
    store: &S,
    subject: &S::Subject,
    predicate: &IriS,
) -> Result<HashSet<Term>, HelperError> {
    match store.objects_for_subject_predicate(subject, &S::iri_s2iri(predicate)) {
        Ok(terms) => Ok(terms),
        Err(_) => Err(HelperError::NoTripleFound),
    }
}

pub(crate) fn get_object_for<S: SRDF + SRDFBasic>(
    store: &S,
    subject: &S::Subject,
    predicate: &IriS,
) -> Result<Option<Term>, HelperError> {
    let objects = get_objects_for(store, subject, predicate)?;
    match objects.into_iter().nth(0) {
        Some(object) => Ok(Some(object)),
        None => Ok(None),
    }
}
