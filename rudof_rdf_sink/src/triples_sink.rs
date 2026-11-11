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

/// Two handles are equal when they share the same sink.
impl PartialEq for TriplesSink {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl fmt::Debug for TriplesSink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TriplesSink").finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CountingSink;

    #[test]
    fn handles_are_equal_only_when_they_share_the_sink() {
        let sink = Arc::new(Mutex::new(CountingSink::new()));
        let a = TriplesSink::new(sink.clone());
        assert_eq!(a, TriplesSink::new(sink));
        assert_ne!(a, TriplesSink::new(Arc::new(Mutex::new(CountingSink::new()))));
    }
}
