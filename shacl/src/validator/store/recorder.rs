//! A view of a store that records what the validator reads from it.
//!
//! Every lookup the native engine makes ends up in
//! [`NeighsRDF::triples_matching`]. [`ReadRecorder`] forwards each one to the
//! wrapped store and, when a [`TriplesSink`] is installed, pushes every triple
//! it hands back into the sink. Without a sink it is a plain pass-through.
//!
//! SPARQL queries ([`QueryRDF`]) are forwarded but not recorded: their
//! answers are bindings, not triples.

use oxrdf::Triple as OxTriple;
use prefixmap::{PrefixMap, PrefixMapError};
use rudof_iri::IriS;
#[cfg(feature = "sparql")]
use rudof_rdf::rdf_core::query::{QueryRDF, QueryResultFormat, QuerySolutions};
use rudof_rdf::rdf_core::{Matcher, NeighsRDF, Rdf};
use rudof_rdf_sink::{RdfSinkError, TriplesSink};
use std::fmt::{self, Debug, Display};

/// See the [module documentation](self).
#[derive(Debug)]
pub struct ReadRecorder<S> {
    inner: S,
    sink: Option<TriplesSink>,
}

/// An error of the wrapped store, or of the sink refusing a triple.
#[derive(Debug)]
pub enum ReadRecorderError<E> {
    Store(E),
    Sink(String),
}

impl<E: Display> Display for ReadRecorderError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReadRecorderError::Store(e) => write!(f, "{e}"),
            ReadRecorderError::Sink(e) => write!(f, "Failed to record a triple read by the validator: {e}"),
        }
    }
}

impl<E> From<RdfSinkError> for ReadRecorderError<E> {
    fn from(e: RdfSinkError) -> Self {
        ReadRecorderError::Sink(e.to_string())
    }
}

impl<S> ReadRecorder<S> {
    pub fn new(inner: S) -> Self {
        Self { inner, sink: None }
    }

    pub fn inner(&self) -> &S {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    pub fn into_inner(self) -> S {
        self.inner
    }

    /// Installs the sink that receives the triples read from now on, or
    /// removes it with `None`.
    pub fn set_sink(&mut self, sink: Option<TriplesSink>) {
        self.sink = sink;
    }

    pub fn is_recording(&self) -> bool {
        self.sink.is_some()
    }

    fn record<'a, E>(&self, triples: impl IntoIterator<Item = &'a OxTriple>) -> Result<(), ReadRecorderError<E>> {
        let Some(sink) = &self.sink else {
            return Ok(());
        };
        let mut sink = sink.0.lock().map_err(|e| ReadRecorderError::Sink(e.to_string()))?;
        for triple in triples {
            sink.push(triple.clone())?;
        }
        Ok(())
    }
}

impl<S> From<S> for ReadRecorder<S> {
    fn from(inner: S) -> Self {
        Self::new(inner)
    }
}

impl<S: Rdf> Rdf for ReadRecorder<S> {
    type IRI = S::IRI;
    type BNode = S::BNode;
    type Literal = S::Literal;
    type Subject = S::Subject;
    type Term = S::Term;
    type Triple = S::Triple;
    type Err = ReadRecorderError<S::Err>;

    fn prefixmap(&self) -> Option<PrefixMap> {
        self.inner.prefixmap()
    }

    fn qualify_iri(&self, iri: &Self::IRI) -> String {
        self.inner.qualify_iri(iri)
    }

    fn qualify_subject(&self, subj: &Self::Subject) -> String {
        self.inner.qualify_subject(subj)
    }

    fn qualify_term(&self, term: &Self::Term) -> String {
        self.inner.qualify_term(term)
    }

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        self.inner.resolve_prefix_local(prefix, local)
    }
}

impl<S: NeighsRDF<Triple = OxTriple>> NeighsRDF for ReadRecorder<S> {
    fn triples(&self) -> Result<impl Iterator<Item = Self::Triple>, Self::Err> {
        let triples: Vec<OxTriple> = self.inner.triples().map_err(ReadRecorderError::Store)?.collect();
        self.record(&triples)?;
        Ok(triples.into_iter())
    }

    /// Records only the triple that answers the question, not every match.
    fn contains<Sm, P, O>(&self, subject: &Sm, predicate: &P, object: &O) -> Result<bool, Self::Err>
    where
        Sm: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        let first = self
            .inner
            .triples_matching(subject, predicate, object)
            .map_err(ReadRecorderError::Store)?
            .next();
        self.record(&first)?;
        Ok(first.is_some())
    }

    fn triples_matching<Sm, P, O>(
        &self,
        subject: &Sm,
        predicate: &P,
        object: &O,
    ) -> Result<impl Iterator<Item = Self::Triple> + '_, Self::Err>
    where
        Sm: Matcher<Self::Subject>,
        P: Matcher<Self::IRI>,
        O: Matcher<Self::Term>,
    {
        let triples: Vec<OxTriple> = self
            .inner
            .triples_matching(subject, predicate, object)
            .map_err(ReadRecorderError::Store)?
            .collect();
        self.record(&triples)?;
        Ok(triples.into_iter())
    }
}

#[cfg(feature = "sparql")]
impl<S: QueryRDF> QueryRDF for ReadRecorder<S> {
    fn query_construct(&self, query: &str, format: &QueryResultFormat) -> Result<String, Self::Err> {
        self.inner
            .query_construct(query, format)
            .map_err(ReadRecorderError::Store)
    }

    fn query_select(&self, query: &str) -> Result<QuerySolutions<Self>, Self::Err> {
        let solutions = self.inner.query_select(query).map_err(ReadRecorderError::Store)?;
        let pm = self.inner.prefixmap().unwrap_or_default();
        let mut converted = QuerySolutions::empty();
        converted.extend(solutions.iter().map(|s| s.convert(|t| t.clone())).collect(), pm);
        Ok(converted)
    }

    fn query_ask(&self, query: &str) -> Result<bool, Self::Err> {
        self.inner.query_ask(query).map_err(ReadRecorderError::Store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxrdf::NamedNode;
    use rudof_rdf::rdf_core::{Any, RDFFormat};
    use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
    use rudof_rdf_sink::{CountingSink, RdfSink};
    use std::sync::{Arc, Mutex};

    const DATA: &str = r#"
        prefix : <http://example.org/>
        :alice :name "Alice" ; :knows :bob, :carol .
    "#;

    fn ex(local: &str) -> NamedNode {
        NamedNode::new_unchecked(format!("http://example.org/{local}"))
    }

    fn recorder() -> (ReadRecorder<OxigraphInMemory>, Arc<Mutex<CountingSink>>) {
        let graph = OxigraphInMemory::from_str(DATA, &RDFFormat::Turtle, None, &ReaderMode::default()).unwrap();
        let sink = Arc::new(Mutex::new(CountingSink::new()));
        let mut recorder = ReadRecorder::new(graph);
        recorder.set_sink(Some(TriplesSink::new(sink.clone())));
        (recorder, sink)
    }

    fn reads(sink: &Arc<Mutex<CountingSink>>) -> u64 {
        sink.lock().unwrap().report().triples
    }

    #[test]
    fn records_every_triple_a_lookup_returns() {
        let (recorder, sink) = recorder();
        let alice: oxrdf::NamedOrBlankNode = ex("alice").into();
        let found = recorder.triples_matching(&alice, &ex("knows"), &Any).unwrap().count();
        assert_eq!(found, 2);
        assert_eq!(reads(&sink), 2);
    }

    #[test]
    fn contains_records_only_the_triple_that_answers() {
        let (recorder, sink) = recorder();
        assert!(recorder.contains(&Any, &ex("knows"), &Any).unwrap());
        assert_eq!(reads(&sink), 1);
        assert!(!recorder.contains(&Any, &ex("age"), &Any).unwrap());
        assert_eq!(reads(&sink), 1);
    }

    #[test]
    fn records_nothing_without_a_sink() {
        let (mut recorder, sink) = recorder();
        recorder.set_sink(None);
        assert!(!recorder.is_recording());
        assert_eq!(recorder.triples().unwrap().count(), 3);
        assert_eq!(reads(&sink), 0);
    }
}
