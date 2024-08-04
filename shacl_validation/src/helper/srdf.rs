use std::{collections::HashSet, path::Path, str::FromStr};

use oxiri::Iri;
use shacl_ast::{Schema, ShaclParser};
use srdf::{RDFFormat, SRDFGraph, SRDF};

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

pub(crate) fn get_objects_for<S: SRDF>(
    store: &S,
    subject: &S::Term,
    predicate: &S::IRI,
) -> Result<HashSet<S::Term>, SRDFError> {
    let subject = match S::term_as_subject(subject) {
        Some(subject) => subject,
        None => todo!(),
    };

    match store.objects_for_subject_predicate(&subject, predicate) {
        Ok(ans) => Ok(ans),
        Err(_) => Err(SRDFError::Srdf),
    }
}

pub fn load_shapes_graph(
    path: &Path,
    rdf_format: RDFFormat,
    base: Option<&str>,
) -> Result<Schema, SRDFError> {
    let rdf = SRDFGraph::from_path(
        path,
        &rdf_format,
        match base {
            Some(base) => Some(Iri::from_str(base)?),
            None => None,
        },
    )?;

    match ShaclParser::new(rdf).parse() {
        Ok(schema) => Ok(schema),
        Err(_) => Err(SRDFError::Srdf),
    }
}
