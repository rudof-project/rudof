use std::collections::HashSet;

use srdf::{matcher::Any, Query, RDFNode, Triple};

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
        Some(term) => Ok(Some(term.into())),
        None => Ok(None),
    }
}

pub(crate) fn get_objects_for<S: Query>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<HashSet<S::Term>, SRDFError> {
    let subject: S::Subject = match subject.clone().try_into() {
        Ok(subject) => subject,
        Err(_) => {
            return Err(SRDFError::SRDFTermAsSubject {
                subject: format!("{subject}"),
            })
        }
    };

    Ok(store
        .triples_matching(subject, predicate.clone(), Any)
        .map(Triple::into_object)
        .collect())
}

pub(crate) fn get_subjects_for<S: Query>(
    store: &S,
    predicate: &S::IRI,
    object: &S::Term,
) -> Result<HashSet<S::Term>, SRDFError> {
    match store.subjects_with_predicate_object(predicate, object) {
        Ok(ans) => Ok(ans.into_iter().map(Into::into).collect()),
        Err(e) => Err(SRDFError::SubjectsWithPredicateObject {
            predicate: format!("{predicate}"),
            object: format!("{object}"),
            error: format!("{e}"),
        }),
    }
}
