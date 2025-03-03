use std::collections::HashSet;

use srdf::{matcher::Any, Query, RDFNode, Triple};

use super::helper_error::SRDFError;

pub(crate) fn get_object_for<Q: Query>(
    store: &Q,
    subject: &Q::Term,
    predicate: &Q::IRI,
) -> Result<Option<RDFNode>, SRDFError> {
    match get_objects_for(store, subject, predicate)?
        .into_iter()
        .next()
    {
        Some(term) => Ok(Some(term.into())),
        None => Ok(None),
    }
}

pub(crate) fn get_objects_for<Q: Query>(
    store: &Q,
    subject: &Q::Term,
    predicate: &Q::IRI,
) -> Result<HashSet<Q::Term>, SRDFError> {
    let subject: Q::Subject = match subject.clone().try_into() {
        Ok(subject) => subject,
        Err(_) => {
            return Err(SRDFError::SRDFTermAsSubject {
                subject: format!("{subject}"),
            })
        }
    };

    let triples = store
        .triples_matching(subject, predicate.clone(), Any)
        .map_err(|e| SRDFError::Srdf {
            error: e.to_string(),
        })?
        .map(Triple::into_object)
        .collect();

    Ok(triples)
}

pub(crate) fn get_subjects_for<Q: Query>(
    store: &Q,
    predicate: &Q::IRI,
    object: &Q::Term,
) -> Result<HashSet<Q::Term>, SRDFError> {
    let values = store
        .triples_matching(Any, predicate.clone(), object.clone())
        .map_err(|e| SRDFError::Srdf {
            error: e.to_string(),
        })?
        .map(Triple::into_subject)
        .map(Into::into)
        .collect();
    Ok(values)
}
