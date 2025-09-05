/*use srdf::RDFNodeParse;
use srdf::{FocusRDF, NeighsRDF, RDFNode, SHACLPath, Triple, matcher::Any, shacl_path_parse};
use std::collections::HashSet;
use tracing::debug;

use super::helper_error::SRDFError;*/

// TODO: Remove the following functions which are implemented in SRDF
/*pub(crate) fn get_object_for<S: NeighsRDF>(
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

pub(crate) fn get_objects_for_shacl_path<S: NeighsRDF>(
    store: &S,
    subject: &S::Term,
    path: &SHACLPath,
) -> Result<HashSet<S::Term>, SRDFError> {
    match path {
        SHACLPath::Predicate { pred } => {
            let pred: S::IRI = pred.clone().into();
            get_objects_for(store, subject, &pred)
        }
        SHACLPath::Alternative { paths } => {
            let mut all_objects = HashSet::new();
            for path in paths {
                let objects = get_objects_for_shacl_path(store, subject, path)?;
                all_objects.extend(objects);
            }
            Ok(all_objects)
        }
        SHACLPath::Sequence { paths } => match paths.as_slice() {
            [] => Ok(HashSet::from([subject.clone()])),
            [first, rest @ ..] => {
                let first_objects = get_objects_for_shacl_path(store, subject, first)?;
                let mut all_objects = HashSet::new();
                for obj in first_objects {
                    let intermediate_objects = get_objects_for_shacl_path(
                        store,
                        &obj,
                        &SHACLPath::Sequence {
                            paths: rest.to_vec(),
                        },
                    )?;
                    all_objects.extend(intermediate_objects);
                }
                Ok(all_objects)
            }
        },
        SHACLPath::Inverse { path } => {
            let objects = get_subjects_for(store, &path.pred().unwrap().clone().into(), subject)?;
            Ok(objects)
        }
        SHACLPath::ZeroOrMore { path } => {
            let mut all_objects = HashSet::new();
            all_objects.insert(subject.clone());

            let mut to_process = vec![subject.clone()];
            while let Some(current) = to_process.pop() {
                let next_objects = get_objects_for_shacl_path(store, &current, path)?;
                for obj in next_objects {
                    if all_objects.insert(obj.clone()) {
                        to_process.push(obj);
                    }
                }
            }
            Ok(all_objects)
        }
        SHACLPath::OneOrMore { path } => {
            let mut all_objects = HashSet::new();
            let first_objects = get_objects_for_shacl_path(store, subject, path)?;
            all_objects.extend(first_objects.clone());

            let mut to_process: Vec<S::Term> = first_objects.into_iter().collect();
            while let Some(current) = to_process.pop() {
                let next_objects = get_objects_for_shacl_path(store, &current, path)?;
                for obj in next_objects {
                    if all_objects.insert(obj.clone()) {
                        to_process.push(obj);
                    }
                }
            }
            Ok(all_objects)
        }
        SHACLPath::ZeroOrOne { path } => {
            let mut all_objects = HashSet::new();
            all_objects.insert(subject.clone());
            let next_objects = get_objects_for_shacl_path(store, subject, path)?;
            all_objects.extend(next_objects);
            Ok(all_objects)
        }
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
                "Error obtaining objects for subject {} and predicate {}: {e}",
                subject_str, predicate_str
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

pub(crate) fn get_path_for<R>(
    rdf: &mut R,
    subject: &R::Term,
    predicate: &R::IRI,
) -> Result<Option<SHACLPath>, SRDFError>
where
    R: FocusRDF,
{
    match get_objects_for(rdf, subject, predicate)?.into_iter().next() {
        Some(term) => match shacl_path_parse::<R>(term.clone()).parse_impl(rdf) {
            Ok(path) => Ok(Some(path)),
            Err(e) => {
                debug!("Error parsing PATH from report...{e}");
                Ok(None)
            }
        },
        None => Ok(None),
    }
    /*match get_objects_for(store, subject, predicate)?
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
    }*/
}
*/
