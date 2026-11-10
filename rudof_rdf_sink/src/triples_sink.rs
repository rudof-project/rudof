use std::fmt;
use std::sync::{Arc, Mutex};

use crate::RdfSink;

/// A sink shared between the producer that pushes triples into it and the
/// caller, who keeps a handle to read it back once the producer is done.
#[derive(Clone)]
pub struct TriplesSink(pub Arc<Mutex<dyn RdfSink + Send>>);

impl TriplesSink {
    pub fn new<S: RdfSink + Send + 'static>(sink: Arc<Mutex<S>>) -> Self {
        TriplesSink(sink)
    }
}

impl fmt::Debug for TriplesSink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TriplesSink").finish_non_exhaustive()
    }
}
