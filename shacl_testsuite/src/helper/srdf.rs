use std::collections::HashSet;

use srdf::{Rdf, SRDF};

use super::helper_error::HelperError;

pub(crate) fn get_object_for<S: SRDF + Rdf>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<Option<S::Term>, HelperError> {
    match get_objects_for(store, subject, predicate)?
        .into_iter()
        .next()
    {
        Some(term) => Ok(Some(term)),
        None => Ok(None),
    }
}

pub(crate) fn get_objects_for<S: SRDF + Rdf>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<HashSet<S::Term>, HelperError> {
    let subject = match S::term_as_subject(subject) {
        Some(subject) => subject,
        None => todo!(),
    };

    match store.objects_for_subject_predicate(&subject, predicate) {
        Ok(ans) => Ok(ans),
        Err(_) => Err(HelperError::NoTripleFound),
    }
}
