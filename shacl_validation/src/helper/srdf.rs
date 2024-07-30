use std::collections::HashSet;

use srdf::{SRDFBasic, SRDF};

use super::helper_error::SRDFError;
use super::term::Term;

pub(crate) fn get_object_for<'a, S: SRDF + SRDFBasic>(
    store: &'a S,
    subject: &'a Term,
    predicate: &'a S::IRI,
) -> Result<Option<Term>, SRDFError> {
    match get_objects_for(store, subject, predicate)?
        .into_iter()
        .nth(0)
    {
        Some(term) => Ok(Some(term)),
        None => Ok(None),
    }
}

pub(crate) fn get_objects_for<'a, S: SRDF + SRDFBasic>(
    store: &'a S,
    subject: &'a Term,
    predicate: &'a S::IRI,
) -> Result<HashSet<Term>, SRDFError> {
    let object = subject.into();
    let term = S::object_as_term(&object);

    let subject = match S::term_as_subject(&term) {
        Some(subject) => subject,
        None => todo!(),
    };

    match store.objects_for_subject_predicate(&subject, predicate) {
        Ok(res) => Ok(res
            .iter()
            .map(|object| S::term_as_object(&object).into())
            .collect()),
        Err(_) => Err(SRDFError::SRDF),
    }
}
