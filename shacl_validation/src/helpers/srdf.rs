use std::collections::HashSet;

use srdf::{matcher::Any, NeighsRDF, Object, RDFNode, SHACLPath, Triple};

use super::helper_error::SRDFError;

pub(crate) fn get_object_for<S: NeighsRDF>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<Option<RDFNode>, SRDFError> {
    match get_objects_for(store, subject, predicate)?
        .into_iter()
        .next()
    {
        Some(term) => {
            let obj = S::term_as_object(&term)?;
            Ok(Some(obj))
        }
        None => Ok(None),
    }
}

pub(crate) fn get_objects_for<S: NeighsRDF>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<HashSet<S::Term>, SRDFError> {
    let subject: S::Subject = match S::term_as_subject(subject) {
        Ok(subject) => subject,
        Err(_) => {
            return Err(SRDFError::SRDFTermAsSubject {
                subject: format!("{subject}"),
            });
        }
    };
    let subject_str = format!("{subject}");
    let predicate_str = format!("{predicate}");
    let triples = store
        .triples_matching(subject, predicate.clone(), Any)
        .map_err(|e| SRDFError::Srdf {
            error: format!(
                "Error obtaining objects for subject {} and predicate {}: {}",
                subject_str,
                predicate_str,
                e.to_string()
            ),
        })?
        .map(Triple::into_object)
        .collect();

    Ok(triples)
}

pub(crate) fn get_subjects_for<S: NeighsRDF>(
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

pub(crate) fn get_path_for<S: NeighsRDF>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<Option<SHACLPath>, SRDFError> {
    match get_objects_for(store, subject, predicate)?
        .into_iter()
        .next()
    {
        Some(term) => {
            let obj: Object = S::term_as_object(&term)?;
            match obj {
                Object::Iri(iri_s) => Ok(Some(SHACLPath::iri(iri_s))),
                Object::BlankNode(_) => todo!(),
                Object::Literal(literal) => Err(SRDFError::SHACLUnexpectedLiteral {
                    lit: literal.to_string(),
                }),
                Object::Triple { .. } => todo!(),
            }
        }
        None => Ok(None),
    }
}
