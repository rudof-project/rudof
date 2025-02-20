use std::collections::HashSet;

use srdf::{Query, RDFNode};

use super::helper_error::SRDFError;

pub(crate) fn get_object_for<S: Query>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<Option<RDFNode>, SRDFError> {
    match get_objects_for(store, subject, predicate)?
        .into_iter()
        .next()
    {
        Some(term) => Ok(Some(S::term_as_object(&term))),
        None => Ok(None),
    }
}

pub(crate) fn get_objects_for<S: Query>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<HashSet<S::Term>, SRDFError> {
    let subject = match S::term_as_subject(subject) {
        Some(subject) => subject,
        None => {
            return Err(SRDFError::SRDFTermAsSubject {
                subject: format!("{subject}"),
            })
        }
    };

    store
        .objects_for_subject_predicate(&subject, predicate)
        .map_err(|e| SRDFError::ObjectsWithSubjectPredicate {
            predicate: format!("{predicate}"),
            subject: format!("{subject}"),
            error: format!("{e}"),
        })
}

pub(crate) fn get_subjects_for<S: Query>(
    store: &S,
    predicate: &S::IRI,
    object: &S::Term,
) -> Result<HashSet<S::Term>, SRDFError> {
    match store.subjects_with_predicate_object(predicate, object) {
        Ok(ans) => Ok(ans
            .into_iter()
            .map(|subject| S::subject_as_term(&subject))
            .collect()),
        Err(e) => Err(SRDFError::SubjectsWithPredicateObject {
            predicate: format!("{predicate}"),
            object: format!("{object}"),
            error: format!("{e}"),
        }),
    }
}
