use std::collections::HashSet;

use srdf::model::rdf::Object;
use srdf::model::rdf::Predicate;
use srdf::model::rdf::Rdf;
use srdf::model::Triple;

use crate::helpers::helper_error::SRDFError;

pub(crate) fn get_object_for<R: Rdf>(
    store: &R,
    subject: &Object<R>,
    predicate: &Predicate<R>,
) -> Result<Option<Object<R>>, SRDFError> {
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
    subject: &Object<R>,
    predicate: &Predicate<R>,
) -> Result<HashSet<Object<R>>, SRDFError> {
    let subject = match subject.clone().try_into() {
        Ok(subject) => subject,
        Err(_) => {
            return Err(SRDFError::SRDFTermAsSubject {
                subject: format!("{subject}"),
            })
        }
    };

    let ans = match store.triples_matching(Some(&subject), Some(predicate), None) {
        Ok(triples) => Ok(triples.map(Triple::into_object).collect()),
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
    predicate: &Predicate<R>,
    object: &Object<R>,
) -> Result<HashSet<Object<R>>, SRDFError> {
    let ans = match store.triples_matching(None, Some(predicate), Some(object)) {
        Ok(triples) => Ok(triples.map(Triple::into_subject).map(Into::into).collect()),
        Err(e) => Err(SRDFError::SubjectsWithPredicateObject {
            predicate: format!("{predicate}"),
            object: format!("{object}"),
            error: format!("{e}"),
        }),
    };
    ans
}
