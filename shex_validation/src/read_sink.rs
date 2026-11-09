use std::fmt;
use std::sync::{Arc, Mutex};

use rudof_rdf_sink::RdfSink;

/// A sink that receives every triple the validator reads from the data.
///
/// Shared, because the caller keeps a handle to read the sink back once the
/// validation is over.
#[derive(Clone)]
pub struct ReadSink(pub Arc<Mutex<dyn RdfSink + Send>>);

impl ReadSink {
    pub fn new<S: RdfSink + Send + 'static>(sink: Arc<Mutex<S>>) -> Self {
        ReadSink(sink)
    }
}

impl fmt::Debug for ReadSink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ReadSink").finish_non_exhaustive()
    }
}
