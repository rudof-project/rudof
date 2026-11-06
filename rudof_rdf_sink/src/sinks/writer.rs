use oxrdf::Triple;
use std::io::Write;

use crate::{RdfSink, RdfSinkError, SinkReport};

/// Streams the triples straight out as N-Triples, one line each.
pub struct WriterSink<W> {
    writer: W,
    triples: u64,
    finished: bool,
}

impl<W> WriterSink<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            triples: 0,
            finished: false,
        }
    }

    pub fn into_writer(self) -> W {
        self.writer
    }
}

impl<W: Write> RdfSink for WriterSink<W> {
    fn push(&mut self, triple: Triple) -> Result<(), RdfSinkError> {
        if self.finished {
            return Err(RdfSinkError::Finished);
        }
        writeln!(self.writer, "{triple} .")?;
        self.triples += 1;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), RdfSinkError> {
        self.writer.flush()?;
        Ok(())
    }

    fn finish(&mut self) -> Result<SinkReport, RdfSinkError> {
        if self.finished {
            return Err(RdfSinkError::Finished);
        }
        self.writer.flush()?;
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
    use oxrdf::{BlankNode, Literal, NamedNode, Term};
    use rudof_rdf::rdf_core::{NeighsRDF, RDFFormat};
    use rudof_rdf::rdf_impl::{OxigraphInMemory, ReaderMode};
    use std::collections::HashSet;

    /// A writer that only hands its bytes over on `flush`, so a test can tell
    /// a written triple from a flushed one.
    #[derive(Default)]
    struct FlushOnly {
        flushed: Vec<u8>,
        pending: Vec<u8>,
    }

    impl Write for FlushOnly {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.pending.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.flushed.append(&mut self.pending);
            Ok(())
        }
    }

    fn iri(name: &str) -> NamedNode {
        NamedNode::new_unchecked(format!("http://example.org/{name}"))
    }

    /// One of each: a literal needing escapes, a blank node, an IRI object and
    /// an RDF 1.2 triple term.
    fn triples() -> Vec<Triple> {
        vec![
            Triple::new(
                iri("alice"),
                iri("note"),
                Literal::new_simple_literal("a \"quoted\" line\nand a \\ backslash"),
            ),
            Triple::new(BlankNode::new_unchecked("b0"), iri("knows"), iri("bob")),
            Triple::new(iri("bob"), iri("knows"), iri("alice")),
            Triple::new(
                iri("claim"),
                iri("says"),
                Term::Triple(Box::new(Triple::new(iri("alice"), iri("knows"), iri("bob")))),
            ),
        ]
    }

    fn written(triples: &[Triple]) -> String {
        let mut sink = WriterSink::new(Vec::new());
        for triple in triples {
            sink.push(triple.clone()).unwrap();
        }
        sink.finish().unwrap();
        String::from_utf8(sink.into_writer()).unwrap()
    }

    #[test]
    fn the_n_triples_parse_back_to_the_same_triples() {
        let pushed = triples();
        let parsed =
            OxigraphInMemory::from_str(&written(&pushed), &RDFFormat::NTriples, None, &ReaderMode::Strict).unwrap();

        let got: HashSet<Triple> = parsed.triples().unwrap().collect();
        assert_eq!(got, pushed.into_iter().collect::<HashSet<_>>());
    }

    #[test]
    fn a_push_reaches_the_writer_without_waiting_for_finish() {
        let mut sink = WriterSink::new(Vec::new());
        sink.push(triples()[2].clone()).unwrap();

        let out = String::from_utf8(sink.into_writer()).unwrap();
        assert_eq!(
            out,
            "<http://example.org/bob> <http://example.org/knows> <http://example.org/alice> .\n"
        );
    }

    #[test]
    fn finish_flushes_the_writer() {
        let mut sink = WriterSink::new(FlushOnly::default());
        sink.push(triples()[2].clone()).unwrap();
        assert!(sink.writer.flushed.is_empty());

        sink.finish().unwrap();
        assert!(!sink.into_writer().flushed.is_empty());
    }

    #[test]
    fn flush_flushes_the_writer_without_ending_the_stream() {
        let mut sink = WriterSink::new(FlushOnly::default());
        sink.push(triples()[2].clone()).unwrap();
        sink.flush().unwrap();
        assert!(!sink.writer.flushed.is_empty());

        // Still takes triples.
        sink.push(triples()[1].clone()).unwrap();
        assert_eq!(sink.finish().unwrap().triples, 2);
    }

    #[test]
    fn pushing_after_finish_fails() {
        let mut sink = WriterSink::new(Vec::new());
        sink.finish().unwrap();

        assert!(matches!(sink.push(triples()[2].clone()), Err(RdfSinkError::Finished)));
        assert!(matches!(sink.finish(), Err(RdfSinkError::Finished)));
    }
}
