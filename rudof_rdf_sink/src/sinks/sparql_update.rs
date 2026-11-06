use oxrdf::Triple;
use rudof_rdf::rdf_impl::OxigraphEndpoint;
use std::fmt::Write as _;

use crate::{RdfSink, RdfSinkError, SinkReport};

/// Triples per `INSERT DATA` request, unless
/// [`with_batch_size`](SparqlUpdateSink::with_batch_size) says otherwise.
pub const DEFAULT_BATCH_TRIPLES: usize = 1000;

/// Sends the triples to a SPARQL endpoint, batched into `INSERT DATA`
/// requests.
///
/// One thing to know before pushing:
///
/// - **Batching splits blank node scope.** Each `INSERT DATA` mints fresh
///   labels for the blank nodes it mentions, so a blank node appearing in two
///   batches becomes two nodes in the store. A producer that emits blank node
///   structures (an RDF list, a SHACL result) has to keep each structure
///   within one batch, or use another sink.
pub struct SparqlUpdateSink {
    endpoint: OxigraphEndpoint,
    batch: Vec<Triple>,
    batch_size: usize,
    triples: u64,
    finished: bool,
}

impl SparqlUpdateSink {
    pub fn new(endpoint: OxigraphEndpoint) -> Self {
        Self {
            endpoint,
            batch: Vec::new(),
            batch_size: DEFAULT_BATCH_TRIPLES,
            triples: 0,
            finished: false,
        }
    }

    /// Sets how many triples go in one `INSERT DATA`. A size of zero is read
    /// as one.
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size.max(1);
        self
    }

    /// The `INSERT DATA` request that inserts `triples`, written in N-Triples
    /// syntax: absolute IRIs, so no `PREFIX` declaration has to match.
    fn insert_data(triples: &[Triple]) -> String {
        let mut update = String::from("INSERT DATA {\n");
        for triple in triples {
            let _ = writeln!(update, "{triple} .");
        }
        update.push_str("}\n");
        update
    }

    /// Sends the batch, if there is one, and empties it. A batch the endpoint
    /// rejected is kept, so it is not silently dropped.
    fn send_batch(&mut self) -> Result<(), RdfSinkError> {
        if self.batch.is_empty() {
            return Ok(());
        }
        let update = Self::insert_data(&self.batch);
        self.endpoint.update(&update).map_err(|error| RdfSinkError::Store {
            error: error.to_string(),
        })?;
        self.batch.clear();
        Ok(())
    }
}

impl RdfSink for SparqlUpdateSink {
    fn push(&mut self, triple: Triple) -> Result<(), RdfSinkError> {
        if self.finished {
            return Err(RdfSinkError::Finished);
        }
        self.batch.push(triple);
        self.triples += 1;
        if self.batch.len() >= self.batch_size {
            self.send_batch()?;
        }
        Ok(())
    }

    /// Sends the partial batch now.
    fn flush(&mut self) -> Result<(), RdfSinkError> {
        self.send_batch()
    }

    fn finish(&mut self) -> Result<SinkReport, RdfSinkError> {
        if self.finished {
            return Err(RdfSinkError::Finished);
        }
        self.send_batch()?;
        self.finished = true;
        Ok(self.report())
    }

    fn report(&self) -> SinkReport {
        SinkReport { triples: self.triples }
    }
}

impl Drop for SparqlUpdateSink {
    /// A producer that forgets [`finish`](RdfSink::finish) leaves a batch
    /// nobody will ever send, so it has to hear about it.
    fn drop(&mut self) {
        if !self.batch.is_empty() {
            tracing::warn!(
                triples = self.batch.len(),
                "SparqlUpdateSink dropped with triples that never reached the endpoint"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxrdf::{BlankNode, Literal, NamedNode};
    use prefixmap::PrefixMap;
    use rudof_iri::IriS;
    use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
    use rudof_rdf::rdf_impl::{EndpointStrategy, OxigraphInMemory, ReaderMode};
    use std::collections::HashSet;

    fn iri(name: &str) -> NamedNode {
        NamedNode::new_unchecked(format!("http://example.org/{name}"))
    }

    fn triples() -> Vec<Triple> {
        vec![
            Triple::new(iri("alice"), iri("knows"), iri("bob")),
            Triple::new(
                BlankNode::new_unchecked("b0"),
                iri("note"),
                Literal::new_simple_literal("a \"quoted\" line"),
            ),
        ]
    }

    /// An endpoint whose updates fail without touching the network: the
    /// `Dereference` strategy rejects an UPDATE outright.
    fn offline_sink() -> SparqlUpdateSink {
        let endpoint = OxigraphEndpoint::new(&IriS::new_unchecked("http://example.org/sparql"), &PrefixMap::new())
            .unwrap()
            .with_strategy(EndpointStrategy::Dereference);
        SparqlUpdateSink::new(endpoint)
    }

    #[test]
    fn insert_data_wraps_the_triples_in_n_triples_syntax() {
        let update = SparqlUpdateSink::insert_data(&triples());

        let body = update
            .strip_prefix("INSERT DATA {\n")
            .and_then(|rest| rest.strip_suffix("}\n"))
            .unwrap_or_else(|| panic!("not an INSERT DATA: {update}"));

        // The body is N-Triples, with the absolute IRIs an endpoint needs.
        assert!(body.contains("<http://example.org/alice>"), "{body}");
        let parsed = OxigraphInMemory::from_str(body, &RDFFormat::NTriples, None, &ReaderMode::Strict).unwrap();
        let got: HashSet<Triple> = parsed.triples().unwrap().collect();
        assert_eq!(got, triples().into_iter().collect::<HashSet<_>>());
    }

    #[test]
    fn insert_data_of_nothing_is_an_empty_insert() {
        assert_eq!(SparqlUpdateSink::insert_data(&[]), "INSERT DATA {\n}\n");
    }

    #[test]
    fn push_sends_once_the_batch_fills() {
        let mut sink = offline_sink().with_batch_size(2);

        // Nothing is sent while the batch has room, so nothing can fail yet.
        sink.push(triples()[0].clone()).unwrap();
        // The second push fills the batch, which sends it — and this endpoint
        // rejects the send.
        assert!(matches!(
            sink.push(triples()[1].clone()),
            Err(RdfSinkError::Store { .. })
        ));
        assert_eq!(sink.report().triples, 2);
    }

    #[test]
    fn flush_sends_a_partial_batch_and_an_empty_one_sends_nothing() {
        let mut sink = offline_sink();

        // No batch, no request.
        sink.flush().unwrap();

        sink.push(triples()[0].clone()).unwrap();
        assert!(matches!(sink.flush(), Err(RdfSinkError::Store { .. })));
    }

    #[test]
    fn finish_sends_the_tail() {
        let mut sink = offline_sink();
        sink.push(triples()[0].clone()).unwrap();

        assert!(matches!(sink.finish(), Err(RdfSinkError::Store { .. })));
    }

    #[test]
    fn an_empty_sink_finishes_and_then_refuses_triples() {
        let mut sink = offline_sink();
        assert_eq!(sink.finish().unwrap().triples, 0);

        assert!(matches!(sink.push(triples()[0].clone()), Err(RdfSinkError::Finished)));
        assert!(matches!(sink.finish(), Err(RdfSinkError::Finished)));
    }
}
