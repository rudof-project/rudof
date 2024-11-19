use std::collections::HashSet;

use srdf::model::rdf::Rdf;
use srdf::model::rdf::TObjectRef;
use srdf::model::rdf::TPredicateRef;
use srdf::model::Triple;

use crate::helpers::helper_error::SRDFError;

pub(crate) fn get_object_for<R: Rdf>(
    store: &R,
    subject: &TObjectRef<R>,
    predicate: &TPredicateRef<R>,
) -> Result<Option<TObjectRef<R>>, SRDFError> {
    match get_objects_for(store, subject, predicate)?
        .into_iter()
        .next()
    {
        Some(term) => Ok(Some(term)),
        None => Ok(None),
    }
}

pub(crate) fn get_objects_for<R: Rdf>(
    store: &R,
    subject: &TObjectRef<R>,
    predicate: &TPredicateRef<R>,
) -> Result<HashSet<TObjectRef<R>>, SRDFError> {
    let subject = match subject.clone().try_into() {
        Ok(subject) => subject,
        Err(_) => {
            return Err(SRDFError::SRDFTermAsSubject {
                subject: format!("{subject}"),
            })
        }
    };

    let ans = match store.triples_matching(Some(&subject), Some(predicate), None) {
        Ok(triples) => Ok(triples.map(Triple::object).collect()),
        Err(e) => Err(SRDFError::ObjectsWithSubjectPredicate {
            predicate: format!("{predicate}"),
            subject: format!("{subject}"),
            error: format!("{e}"),
        }),
    };

    ans
}

pub(crate) fn get_subjects_for<R: Rdf>(
    store: &R,
    predicate: &TPredicateRef<R>,
    object: &TObjectRef<R>,
) -> Result<HashSet<TObjectRef<R>>, SRDFError> {
    let ans = match store.triples_matching(None, Some(predicate), Some(object)) {
        Ok(triples) => Ok(triples.map(Triple::subject).map(Into::into).collect()),
        Err(e) => Err(SRDFError::SubjectsWithPredicateObject {
            predicate: format!("{predicate}"),
            object: format!("{object}"),
            error: format!("{e}"),
        }),
    };
    ans
}
