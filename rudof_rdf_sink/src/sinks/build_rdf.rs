use oxrdf::{NamedNode as OxNamedNode, NamedOrBlankNode as OxSubject, Term as OxTerm, Triple};
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_impl::OxigraphInMemory;

use crate::{RdfSink, RdfSinkError, SinkReport};

/// Accumulates the triples in a graph, and hands the graph back.
///
/// Generic over the destination, so it is also how a caller writes into
/// whichever backend it configured: a store that implements [`BuildRDF`] *is*
/// its own native API.
pub struct BuildRdfSink<B> {
    graph: B,
    triples: u64,
    finished: bool,
}

/// The default sink: triples kept in memory.
pub type GraphSink = BuildRdfSink<OxigraphInMemory>;

impl<B> BuildRdfSink<B> {
    pub fn new(graph: B) -> Self {
        Self {
            graph,
            triples: 0,
            finished: false,
        }
    }

    pub fn graph(&self) -> &B {
        &self.graph
    }

    pub fn graph_mut(&mut self) -> &mut B {
        &mut self.graph
    }

    pub fn into_graph(self) -> B {
        self.graph
    }

    /// `Err(Finished)` once the sink is finished.
    fn ensure_open(&self) -> Result<(), RdfSinkError> {
        if self.finished {
            return Err(RdfSinkError::Finished);
        }
        Ok(())
    }
}

impl<B> RdfSink for BuildRdfSink<B>
where
    B: BuildRDF,
    OxSubject: Into<B::Subject>,
    OxNamedNode: Into<B::IRI>,
    OxTerm: Into<B::Term>,
{
    /// The graph takes a prefix at any point, so this only fails once the sink
    /// is finished.
    fn declare_prefix(&mut self, alias: &str, iri: &IriS) -> Result<(), RdfSinkError> {
        self.ensure_open()?;
        self.graph.add_prefix(alias, iri);
        Ok(())
    }

    fn declare_prefixes(&mut self, prefixes: PrefixMap) -> Result<(), RdfSinkError> {
        self.ensure_open()?;
        self.graph.merge_prefixes(prefixes);
        Ok(())
    }

    fn declare_base(&mut self, base: &IriS) -> Result<(), RdfSinkError> {
        self.ensure_open()?;
        self.graph.add_base(&Some(base.clone()));
        Ok(())
    }

    fn push(&mut self, triple: Triple) -> Result<(), RdfSinkError> {
        self.ensure_open()?;
        let Triple {
            subject,
            predicate,
            object,
        } = triple;
        self.graph
            .add_triple(subject, predicate, object)
            .map_err(|error| RdfSinkError::Store {
                error: error.to_string(),
            })?;
        self.triples += 1;
        Ok(())
    }

    /// No-op: the graph *is* the destination.
    fn flush(&mut self) -> Result<(), RdfSinkError> {
        Ok(())
    }

    fn finish(&mut self) -> Result<SinkReport, RdfSinkError> {
        self.ensure_open()?;
        self.finished = true;
        Ok(self.report())
    }

    fn report(&self) -> SinkReport {
        SinkReport { triples: self.triples }
    }
}

#[cfg(all(feature = "sparql-update", not(target_family = "wasm")))]
impl<B> BuildRdfSink<B>
where
    B: rudof_rdf::rdf_core::NeighsRDF + rudof_rdf::rdf_core::Rdf<Triple = Triple>,
{
    /// Sends every triple of the graph to `endpoint`, `batch_size` triples per
    /// `INSERT DATA`, and reports what went out.
    ///
    /// `usize::MAX` sends the whole graph in one request, which is the only
    /// way to keep every blank node in one scope — at the cost of a body the
    /// endpoint may refuse for being too large. [`DEFAULT_BATCH_TRIPLES`] is
    /// the safe choice for a graph of unknown size.
    ///
    /// [`DEFAULT_BATCH_TRIPLES`]: crate::DEFAULT_BATCH_TRIPLES
    pub fn upload_to(
        &self,
        endpoint: &rudof_rdf::rdf_impl::OxigraphEndpoint,
        batch_size: usize,
    ) -> Result<SinkReport, RdfSinkError> {
        let triples = self.graph.triples().map_err(|error| RdfSinkError::Store {
            error: error.to_string(),
        })?;
        let mut sink = crate::SparqlUpdateSink::new(endpoint.clone()).with_batch_size(batch_size);
        for triple in triples {
            sink.push(triple)?;
        }
        sink.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxrdf::{Literal as OxLiteral, NamedNode};
    use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
    use std::collections::HashSet;

    fn iri(name: &str) -> NamedNode {
        NamedNode::new_unchecked(format!("http://example.org/{name}"))
    }

    fn triple(subject: &str, predicate: &str, object: &str) -> Triple {
        Triple::new(iri(subject), iri(predicate), OxLiteral::new_simple_literal(object))
    }

    fn graph_sink() -> GraphSink {
        BuildRdfSink::new(OxigraphInMemory::empty())
    }

    fn turtle(sink: &GraphSink) -> String {
        let mut out = Vec::new();
        sink.graph().serialize(&RDFFormat::Turtle, &mut out).unwrap();
        String::from_utf8(out).unwrap()
    }

    #[test]
    fn pushed_triples_are_in_the_graph() {
        let mut sink = graph_sink();
        sink.push(triple("alice", "name", "Alice")).unwrap();
        sink.push(triple("bob", "name", "Bob")).unwrap();

        let got: HashSet<Triple> = sink.graph().triples().unwrap().collect();
        assert_eq!(
            got,
            HashSet::from([triple("alice", "name", "Alice"), triple("bob", "name", "Bob")])
        );
    }

    #[test]
    fn the_report_counts_duplicates() {
        let mut sink = graph_sink();
        for _ in 0..3 {
            sink.push(triple("alice", "name", "Alice")).unwrap();
        }

        // Three pushes, one triple in the graph.
        assert_eq!(sink.finish().unwrap().triples, 3);
        assert_eq!(sink.graph().triples().unwrap().count(), 1);
    }

    #[test]
    fn report_tracks_the_count_mid_stream_and_agrees_with_finish() {
        let mut sink = graph_sink();
        assert_eq!(sink.report().triples, 0);
        sink.push(triple("alice", "name", "Alice")).unwrap();
        assert_eq!(sink.report().triples, 1);
        sink.push(triple("bob", "name", "Bob")).unwrap();
        assert_eq!(sink.report().triples, 2);
        assert_eq!(sink.finish().unwrap(), sink.report());
    }

    #[test]
    fn declared_prefixes_come_out_in_the_turtle() {
        let mut sink = graph_sink();
        sink.declare_prefix("ex", &IriS::new_unchecked("http://example.org/"))
            .unwrap();
        sink.push(triple("alice", "name", "Alice")).unwrap();

        let turtle = turtle(&sink);
        assert!(turtle.contains("@prefix ex: <http://example.org/>"), "{turtle}");
        assert!(turtle.contains("ex:alice"), "{turtle}");
    }

    #[test]
    fn declare_prefixes_lands_the_whole_map() {
        let mut sink = graph_sink();
        let mut prefixes = PrefixMap::new();
        prefixes.add_prefix("ex", IriS::new_unchecked("http://example.org/"));
        prefixes.add_prefix("schema", IriS::new_unchecked("https://schema.org/"));
        sink.declare_prefixes(prefixes).unwrap();

        let prefixmap = sink.graph().prefixmap();
        assert_eq!(prefixmap.find("ex"), Some(&IriS::new_unchecked("http://example.org/")));
        assert_eq!(
            prefixmap.find("schema"),
            Some(&IriS::new_unchecked("https://schema.org/"))
        );
    }

    #[test]
    fn pushing_after_finish_fails() {
        let mut sink = graph_sink();
        sink.finish().unwrap();

        assert!(matches!(
            sink.push(triple("alice", "name", "Alice")),
            Err(RdfSinkError::Finished)
        ));
        assert!(matches!(sink.finish(), Err(RdfSinkError::Finished)));
    }

    #[test]
    fn declaring_after_finish_says_the_sink_is_finished() {
        let mut sink = graph_sink();
        sink.finish().unwrap();

        assert!(matches!(
            sink.declare_prefix("ex", &IriS::new_unchecked("http://example.org/")),
            Err(RdfSinkError::Finished)
        ));
        assert!(matches!(
            sink.declare_base(&IriS::new_unchecked("http://example.org/")),
            Err(RdfSinkError::Finished)
        ));
    }
    #[cfg(all(feature = "sparql-update", not(target_family = "wasm")))]
    mod upload {
        use super::*;
        use rudof_rdf::rdf_impl::{EndpointStrategy, OxigraphEndpoint};

        /// An endpoint whose updates fail without touching the network: the
        /// `Dereference` strategy rejects an UPDATE outright.
        fn offline_endpoint() -> OxigraphEndpoint {
            OxigraphEndpoint::new(&IriS::new_unchecked("http://example.org/sparql"), &PrefixMap::new())
                .unwrap()
                .with_strategy(EndpointStrategy::Dereference)
                .clone()
        }

        #[test]
        fn an_empty_graph_sends_no_request() {
            let sink = graph_sink();

            assert_eq!(sink.upload_to(&offline_endpoint(), usize::MAX).unwrap().triples, 0);
        }

        #[test]
        fn a_graph_with_triples_is_sent_and_left_alone() {
            let mut sink = graph_sink();
            sink.push(triple("alice", "name", "Alice")).unwrap();
            sink.finish().unwrap();

            // The send is attempted — and this endpoint refuses it.
            assert!(matches!(
                sink.upload_to(&offline_endpoint(), usize::MAX),
                Err(RdfSinkError::Store { .. })
            ));
            // Uploading reads the graph; it does not consume it.
            assert_eq!(sink.graph().triples().unwrap().count(), 1);
        }
    }
}
