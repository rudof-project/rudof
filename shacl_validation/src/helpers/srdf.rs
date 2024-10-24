use std::collections::HashSet;

use iri_s::IriS;
use srdf::SRDF;

use super::helper_error::SRDFError;

pub(crate) fn get_object_for<S: SRDF>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<Option<S::Term>, SRDFError> {
    match get_objects_for(store, subject, predicate)?
        .into_iter()
        .next()
    {
        Some(term) => Ok(Some(term)),
        None => Ok(None),
    }
}

pub(crate) fn get_object_iri_for<S: SRDF>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<Option<IriS>, SRDFError> {
    match get_objects_for(store, subject, predicate)?
        .into_iter()
        .next()
    {
        Some(term) => {
            if let Some(iri) = S::term_as_iri(&term) {
                let iri = S::iri2iri_s(&iri);
                Ok(Some(iri))
            } else {
                Err(SRDFError::Srdf {
                    error: "Expected object to be an IRI".to_string(),
                })
                // Error as non IRI ?f
            }
        }
        None => Ok(None),
    }
}

pub(crate) fn get_objects_for<S: SRDF>(
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

pub(crate) fn get_objects_iris_for<S: SRDF>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<HashSet<IriS>, SRDFError> {
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
        .and_then(terms_to_iris::<S>)
}

fn terms_to_iris<S: SRDF>(terms: HashSet<S::Term>) -> Result<HashSet<IriS>, SRDFError> {
    // TODO: I would prefer a more declarative way to do this avoiding the creation of a new HashSet...
    let mut result = HashSet::new();
    for term in terms {
        let iri = term_to_iris::<S>(&term)?;
        result.insert(iri);
    }
    Ok(result)
}

fn term_to_iris<S: SRDF>(term: &S::Term) -> Result<IriS, SRDFError> {
    if let Some(iri_s) = S::term_as_iri_s(&term) {
        Ok(iri_s)
    } else {
        Err(SRDFError::Srdf {
            error: format!("Expected IRI but found term: {term}"),
        })
    }
}
pub(crate) fn get_subjects_for<S: SRDF>(
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
