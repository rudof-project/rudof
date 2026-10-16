use crate::{Result, errors::DataError};
use rudof_iri::IriS;
use rudof_rdf::rdf_core::{Rdf, term::Object, term::Triple as TripleTrait};
use sparql_service::RdfData;
use std::{fmt::Display, iter::FusedIterator, vec::IntoIter};

/// A triple of the loaded RDF data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Triple {
    pub subject: Object,
    pub predicate: IriS,
    pub object: Object,
}

impl Display for Triple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}> {} .", self.subject, self.predicate, self.object)
    }
}

/// An iterator over the triples of the loaded RDF data matching a pattern, returned by
/// [`DataOperations::triples`](crate::api::data::DataOperations::triples).
///
/// Borrows the data, so it cannot be modified while its triples are being walked.
#[derive(Debug)]
pub struct Triples<'a> {
    /// The data the triples come from, borrowed for as long as they are iterated.
    _rdf: &'a RdfData,
    triples: IntoIter<<RdfData as Rdf>::Triple>,
    finished: bool,
}

impl<'a> Triples<'a> {
    pub(crate) fn new(rdf: &'a RdfData, triples: Vec<<RdfData as Rdf>::Triple>) -> Self {
        Self {
            _rdf: rdf,
            triples: triples.into_iter(),
            finished: false,
        }
    }
}

impl Iterator for Triples<'_> {
    type Item = Result<Triple>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let triple = self.triples.next()?;
        match to_triple(triple) {
            Ok(triple) => Some(Ok(triple)),
            Err(e) => {
                self.finished = true;
                Some(Err(Box::new(e).into()))
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.triples.size_hint()
    }
}

impl FusedIterator for Triples<'_> {}

fn to_triple(triple: <RdfData as Rdf>::Triple) -> std::result::Result<Triple, DataError> {
    let (subject, predicate, object) = triple.into_components();
    let qualification = |e: rudof_rdf::rdf_core::RDFError| DataError::FailedQualification { error: e.to_string() };
    Ok(Triple {
        subject: RdfData::term_as_object(&RdfData::subject_as_term(&subject)).map_err(qualification)?,
        predicate: predicate.into(),
        object: RdfData::term_as_object(&object).map_err(qualification)?,
    })
}
