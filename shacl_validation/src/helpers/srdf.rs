use std::collections::HashSet;

use srdf::{matcher::Any, Object, Query, RDFNode, SHACLPath, Triple};

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

    let triples = store
        .triples_matching(subject, predicate.clone(), Any)
        .map_err(|e| SRDFError::Srdf {
            error: e.to_string(),
        })?
        .map(Triple::into_object)
        .collect();

    Ok(triples)
}

pub(crate) fn get_subjects_for<S: Query>(
    store: &S,
    predicate: &S::IRI,
    object: &S::Term,
) -> Result<HashSet<S::Term>, SRDFError> {
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

pub(crate) fn get_path_for<S: Query>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<Option<SHACLPath>, SRDFError> {
    match get_objects_for(store, subject, predicate)?
        .into_iter()
        .next()
    {
        Some(term) => {
            let obj: Object = term.into();
            match obj {
                Object::Iri(iri_s) => Ok(Some(SHACLPath::iri(iri_s))),
                Object::BlankNode(_) => todo!(),
                Object::Literal(literal) => Err(SRDFError::SHACLUnexpectedLiteral {
                    lit: literal.to_string(),
                }),
            }
        }
        None => Ok(None),
    }
}
