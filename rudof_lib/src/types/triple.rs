use oxrdf::Triple as OxTriple;
use rudof_rdf::rdf_core::Rdf;
use sparql_service::RdfData;
use std::{fmt::Display, iter::FusedIterator, ops::Deref, vec::IntoIter};

/// A triple of the loaded RDF data.
///
/// Wraps the triple the graph holds and dereferences to it, so its three terms are read
/// as `triple.subject`, `triple.predicate` and `triple.object`. They are the terms of
/// the data itself rather than a rendering of them, so written out as text they come
/// back in the syntax [`add_triple`](crate::api::data::DataOperations::add_triple),
/// [`remove_triple`](crate::api::data::DataOperations::remove_triple) and the patterns
/// of [`triples`](crate::api::data::DataOperations::triples) read: a triple read from
/// the graph goes back into them unedited.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Triple(OxTriple);

impl Triple {
    /// The triple it wraps.
    pub fn into_inner(self) -> OxTriple {
        self.0
    }
}

impl Deref for Triple {
    type Target = OxTriple;

    fn deref(&self) -> &OxTriple {
        &self.0
    }
}

impl From<OxTriple> for Triple {
    fn from(triple: OxTriple) -> Self {
        Self(triple)
    }
}

impl From<Triple> for OxTriple {
    fn from(triple: Triple) -> Self {
        triple.0
    }
}

impl Display for Triple {
    /// The triple in N-Triples syntax, the final `.` included.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} .", self.0)
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
}

impl<'a> Triples<'a> {
    pub(crate) fn new(rdf: &'a RdfData, triples: Vec<<RdfData as Rdf>::Triple>) -> Self {
        Self {
            _rdf: rdf,
            triples: triples.into_iter(),
        }
    }
}

impl Iterator for Triples<'_> {
    type Item = Triple;

    fn next(&mut self) -> Option<Self::Item> {
        self.triples.next().map(Triple::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.triples.size_hint()
    }
}

impl ExactSizeIterator for Triples<'_> {}

impl FusedIterator for Triples<'_> {}
