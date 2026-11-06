use oxrdf::Triple;

use crate::{RdfSink, RdfSinkError, SinkReport};

/// Counts the triples and drops them.
#[derive(Debug, Clone, Copy, Default)]
pub struct CountingSink {
    triples: u64,
    finished: bool,
}

impl CountingSink {
    pub fn new() -> Self {
        Self::default()
    }

    /// `Err(Finished)` once the sink is finished.
    fn ensure_open(&self) -> Result<(), RdfSinkError> {
        if self.finished {
            return Err(RdfSinkError::Finished);
        }
        Ok(())
    }
}

impl RdfSink for CountingSink {
    /// Dropped: there is nothing here to declare it to.
    fn declare_prefix(&mut self, _alias: &str, _iri: &rudof_iri::IriS) -> Result<(), RdfSinkError> {
        self.ensure_open()
    }

    fn declare_prefixes(&mut self, _prefixes: prefixmap::PrefixMap) -> Result<(), RdfSinkError> {
        self.ensure_open()
    }

    /// Dropped, for the same reason.
    fn declare_base(&mut self, _base: &rudof_iri::IriS) -> Result<(), RdfSinkError> {
        self.ensure_open()
    }

    fn push(&mut self, _triple: Triple) -> Result<(), RdfSinkError> {
        self.ensure_open()?;
        self.triples += 1;
        Ok(())
    }

    /// No-op: nothing is held.
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

#[cfg(test)]
mod tests {
    use super::*;
    use oxrdf::{Literal, NamedNode};

    fn triple(object: &str) -> Triple {
        Triple::new(
            NamedNode::new_unchecked("http://example.org/alice"),
            NamedNode::new_unchecked("http://example.org/name"),
            Literal::new_simple_literal(object),
        )
    }

    #[test]
    fn counts_duplicates() {
        let mut sink = CountingSink::new();
        sink.push(triple("Alice")).unwrap();
        sink.push(triple("Alice")).unwrap();
        sink.push(triple("Bob")).unwrap();

        assert_eq!(sink.finish().unwrap().triples, 3);
    }

    #[test]
    fn report_tracks_the_count_mid_stream_and_agrees_with_finish() {
        let mut sink = CountingSink::new();
        assert_eq!(sink.report().triples, 0);
        sink.push(triple("Alice")).unwrap();
        assert_eq!(sink.report().triples, 1);
        assert_eq!(sink.finish().unwrap(), sink.report());
    }

    #[test]
    fn pushing_after_finish_fails() {
        let mut sink = CountingSink::new();
        sink.finish().unwrap();

        assert!(matches!(sink.push(triple("Alice")), Err(RdfSinkError::Finished)));
        assert!(matches!(sink.finish(), Err(RdfSinkError::Finished)));
    }
}
