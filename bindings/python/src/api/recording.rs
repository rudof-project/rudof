//! The sinks of a validation that records the triples it reads and/or its
//! valid and invalid subgraphs, shared by `validate_shex` and `validate_shacl`.

use crate::formats::{ReadTriples, RecordedTriples};
use rudof_lib::Rudof;
use rudof_lib::errors::RudofError as CoreError;
use rudof_lib::types::{GraphSink, RdfSink, TriplesSink};
use rudof_rdf::rdf_impl::OxigraphInMemory;
use std::sync::{Arc, Mutex};

type SharedGraphSink = Arc<Mutex<GraphSink>>;

pub(crate) struct RecordingSinks {
    read: Option<SharedGraphSink>,
    valid: Option<SharedGraphSink>,
    invalid: Option<SharedGraphSink>,
}

fn generic(e: &dyn std::fmt::Display) -> CoreError {
    CoreError::Generic { error: e.to_string() }
}

impl RecordingSinks {
    /// A sink for the triples read with `read_triples`, and one for each
    /// subgraph with `subgraphs`, declaring the prefixes of the session.
    pub(crate) fn new(rudof: &Rudof, read_triples: bool, subgraphs: bool) -> Result<Self, CoreError> {
        let new_sink = |wanted: bool| -> Result<Option<SharedGraphSink>, CoreError> {
            if !wanted {
                return Ok(None);
            }
            let mut sink = GraphSink::new(OxigraphInMemory::new());
            sink.declare_prefixes(rudof.prefixes().execute())
                .map_err(|e| generic(&e))?;
            Ok(Some(Arc::new(Mutex::new(sink))))
        };
        Ok(RecordingSinks {
            read: new_sink(read_triples)?,
            valid: new_sink(subgraphs)?,
            invalid: new_sink(subgraphs)?,
        })
    }

    fn handle(sink: &Option<SharedGraphSink>) -> Option<TriplesSink> {
        sink.as_ref().map(|s| TriplesSink::new(s.clone()))
    }

    pub(crate) fn read(&self) -> Option<TriplesSink> {
        Self::handle(&self.read)
    }

    pub(crate) fn valid(&self) -> Option<TriplesSink> {
        Self::handle(&self.valid)
    }

    pub(crate) fn invalid(&self) -> Option<TriplesSink> {
        Self::handle(&self.invalid)
    }

    /// What the sinks received, once the validation is over.
    pub(crate) fn finish(self) -> Result<RecordedTriples, CoreError> {
        let finish = |sink: Option<SharedGraphSink>| -> Result<Option<(u64, OxigraphInMemory)>, CoreError> {
            let Some(sink) = sink else {
                return Ok(None);
            };
            let mut sink = Arc::try_unwrap(sink)
                .map_err(|_| generic(&"the sink is still shared"))?
                .into_inner()
                .map_err(|e| generic(&e))?;
            let reads = sink.finish().map_err(|e| generic(&e))?.triples;
            Ok(Some((reads, sink.into_graph())))
        };
        Ok(RecordedTriples {
            read: finish(self.read)?.map(|(reads, graph)| ReadTriples { reads, graph }),
            valid: finish(self.valid)?.map(|(_, graph)| graph),
            invalid: finish(self.invalid)?.map(|(_, graph)| graph),
        })
    }
}
