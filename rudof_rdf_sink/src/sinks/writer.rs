use oxrdf::{GraphNameRef, Triple};
use oxttl::nquads::LowLevelNQuadsSerializer;
use oxttl::ntriples::LowLevelNTriplesSerializer;
use oxttl::trig::LowLevelTriGSerializer;
use oxttl::turtle::LowLevelTurtleSerializer;
use oxttl::{NQuadsSerializer, NTriplesSerializer, TriGSerializer, TurtleSerializer};
use prefixmap::PrefixMap;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::RDFFormat;
use std::io::Write;

use crate::{RdfSink, RdfSinkError, SinkReport};

/// Streams the triples straight out, one at a time.
///
/// N-Triples unless [`with_format`](WriterSink::with_format) says otherwise.
pub struct WriterSink<W> {
    writer: W,
    format: StreamFormat,
    serializer: Option<Serializer>,
    prefixes: PrefixMap,
    base: Option<IriS>,
    triples: u64,
    finished: bool,
}

impl<W> WriterSink<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            format: StreamFormat::NTriples,
            serializer: None,
            prefixes: PrefixMap::new(),
            base: None,
            triples: 0,
            finished: false,
        }
    }

    /// Writes `format` instead.
    ///
    /// # Errors
    ///
    /// [`RdfSinkError::NotStreamable`] for RDF/XML, JSON-LD and N3: the first
    /// two have no serializer that can be handed one triple at a time without
    /// taking the writer away from the sink, and N3 has no serializer at all.
    pub fn with_format(mut self, format: &RDFFormat) -> Result<Self, RdfSinkError> {
        self.format = StreamFormat::of(format)?;
        Ok(self)
    }

    pub fn into_writer(self) -> W {
        self.writer
    }

    /// Builds the serializer, declarations and all.
    fn new_serializer(&self) -> Serializer {
        match self.format {
            StreamFormat::NTriples => Serializer::NTriples(NTriplesSerializer::new().low_level()),
            StreamFormat::NQuads => Serializer::NQuads(NQuadsSerializer::new().low_level()),
            StreamFormat::Turtle => {
                let mut serializer = TurtleSerializer::new();
                if let Some(base) = &self.base {
                    serializer = declare_base(serializer, base, TurtleSerializer::with_base_iri);
                }
                for (alias, iri) in self.prefixes.iter() {
                    serializer = declare_prefix(serializer, alias, iri, TurtleSerializer::with_prefix);
                }
                Serializer::Turtle(serializer.low_level())
            },
            StreamFormat::TriG => {
                let mut serializer = TriGSerializer::new();
                if let Some(base) = &self.base {
                    serializer = declare_base(serializer, base, TriGSerializer::with_base_iri);
                }
                for (alias, iri) in self.prefixes.iter() {
                    serializer = declare_prefix(serializer, alias, iri, TriGSerializer::with_prefix);
                }
                Serializer::TriG(serializer.low_level())
            },
        }
    }

    /// Whether `what` can still be declared: the sink has to be open, and the
    /// prelude not written yet.
    fn ensure_declarable(&self, what: &'static str) -> Result<(), RdfSinkError> {
        if self.finished {
            return Err(RdfSinkError::Finished);
        }

        if self.serializer.is_some() && self.format.has_prelude() {
            return Err(RdfSinkError::DeclaredTooLate { what });
        }
        Ok(())
    }
}

/// Adds `base` to the serializer, leaving it as it was if oxttl rejects the
/// IRI.
fn declare_base<S: Clone>(
    serializer: S,
    base: &IriS,
    add: impl FnOnce(S, String) -> Result<S, oxrdf::IriParseError>,
) -> S {
    let fallback = serializer.clone();
    add(serializer, base.as_str().to_string()).unwrap_or_else(|error| {
        tracing::warn!(%base, %error, "base IRI left out of the serialized output");
        fallback
    })
}

/// The same for one prefix.
fn declare_prefix<S: Clone>(
    serializer: S,
    alias: &str,
    iri: &IriS,
    add: impl FnOnce(S, String, String) -> Result<S, oxrdf::IriParseError>,
) -> S {
    let fallback = serializer.clone();
    add(serializer, alias.to_string(), iri.as_str().to_string()).unwrap_or_else(|error| {
        tracing::warn!(alias, %iri, %error, "prefix left out of the serialized output");
        fallback
    })
}

impl<W: Write> RdfSink for WriterSink<W> {
    /// Declared in the output's prelude, if the format has one (Turtle and
    /// TriG) and the first triple has not gone out yet.
    fn declare_prefix(&mut self, alias: &str, iri: &IriS) -> Result<(), RdfSinkError> {
        self.ensure_declarable("prefix")?;
        self.prefixes.add_prefix(alias, iri.clone());
        Ok(())
    }

    fn declare_prefixes(&mut self, prefixes: PrefixMap) -> Result<(), RdfSinkError> {
        self.ensure_declarable("prefixes")?;
        self.prefixes.merge(prefixes);
        Ok(())
    }

    /// Declared like a prefix, and under the same conditions.
    fn declare_base(&mut self, base: &IriS) -> Result<(), RdfSinkError> {
        self.ensure_declarable("base IRI")?;
        self.base = Some(base.clone());
        Ok(())
    }

    fn push(&mut self, triple: Triple) -> Result<(), RdfSinkError> {
        if self.finished {
            return Err(RdfSinkError::Finished);
        }
        if self.serializer.is_none() {
            self.serializer = Some(self.new_serializer());
        }
        let serializer = self.serializer.as_mut().expect("serializer just built");
        serializer.serialize(&triple, &mut self.writer)?;
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
        if let Some(serializer) = self.serializer.as_mut() {
            serializer.finish(&mut self.writer)?;
        }
        self.writer.flush()?;
        self.finished = true;
        Ok(self.report())
    }

    fn report(&self) -> SinkReport {
        SinkReport { triples: self.triples }
    }
}

/// The formats a triple can be handed to one at a time, keeping the writer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StreamFormat {
    NTriples,
    NQuads,
    Turtle,
    TriG,
}

impl StreamFormat {
    /// Whether the format opens the document with the prefix and base
    /// declarations, which is what makes a later one too late.
    fn has_prelude(self) -> bool {
        matches!(self, Self::Turtle | Self::TriG)
    }

    fn of(format: &RDFFormat) -> Result<Self, RdfSinkError> {
        match format {
            RDFFormat::NTriples => Ok(Self::NTriples),
            RDFFormat::NQuads => Ok(Self::NQuads),
            RDFFormat::Turtle => Ok(Self::Turtle),
            RDFFormat::TriG => Ok(Self::TriG),
            RDFFormat::Rdfxml | RDFFormat::JsonLd | RDFFormat::N3 => {
                Err(RdfSinkError::NotStreamable { format: *format })
            },
        }
    }
}

/// One of oxttl's low-level serializers, which take the writer per call rather
/// than owning it.
enum Serializer {
    NTriples(LowLevelNTriplesSerializer),
    NQuads(LowLevelNQuadsSerializer),
    Turtle(LowLevelTurtleSerializer),
    TriG(LowLevelTriGSerializer),
}

impl Serializer {
    fn serialize(&mut self, triple: &Triple, writer: impl Write) -> std::io::Result<()> {
        match self {
            Self::NTriples(serializer) => serializer.serialize_triple(triple, writer),
            Self::Turtle(serializer) => serializer.serialize_triple(triple, writer),
            Self::NQuads(serializer) => {
                serializer.serialize_quad(triple.as_ref().in_graph(GraphNameRef::DefaultGraph), writer)
            },
            Self::TriG(serializer) => {
                serializer.serialize_quad(triple.as_ref().in_graph(GraphNameRef::DefaultGraph), writer)
            },
        }
    }

    /// Closes the statement Turtle and TriG keep open. The line-per-triple
    /// formats have nothing to close.
    fn finish(&mut self, writer: impl Write) -> std::io::Result<()> {
        match self {
            Self::NTriples(_) | Self::NQuads(_) => Ok(()),
            Self::Turtle(serializer) => serializer.finish(writer),
            Self::TriG(serializer) => serializer.finish(writer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxrdf::{BlankNode, Literal, NamedNode, Term};
    use rudof_rdf::rdf_core::NeighsRDF;
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

    fn parsed(format: &RDFFormat, text: &str) -> HashSet<Triple> {
        OxigraphInMemory::from_str(text, format, None, &ReaderMode::Strict)
            .unwrap()
            .triples()
            .unwrap()
            .collect()
    }

    #[test]
    fn turtle_declares_the_prefixes_and_groups_by_subject() {
        let mut sink = WriterSink::new(Vec::new()).with_format(&RDFFormat::Turtle).unwrap();
        sink.declare_prefix("ex", &IriS::new_unchecked("http://example.org/"))
            .unwrap();
        sink.push(Triple::new(iri("alice"), iri("knows"), iri("bob"))).unwrap();
        sink.push(Triple::new(iri("alice"), iri("knows"), iri("carol")))
            .unwrap();
        sink.finish().unwrap();

        let text = String::from_utf8(sink.into_writer()).unwrap();
        assert!(text.contains("@prefix ex: <http://example.org/> ."), "{text}");
        assert!(text.contains("ex:alice ex:knows ex:bob"), "{text}");
        // The second object joins the first instead of repeating the subject.
        assert!(text.contains("ex:carol"), "{text}");
        assert_eq!(text.matches("ex:alice").count(), 1, "{text}");
    }

    #[test]
    fn n_triples_ignores_the_prefixes_it_has_no_syntax_for() {
        let mut sink = WriterSink::new(Vec::new());
        sink.declare_prefix("ex", &IriS::new_unchecked("http://example.org/"))
            .unwrap();
        sink.push(Triple::new(iri("alice"), iri("knows"), iri("bob"))).unwrap();
        sink.finish().unwrap();

        let text = String::from_utf8(sink.into_writer()).unwrap();
        assert_eq!(
            text,
            "<http://example.org/alice> <http://example.org/knows> <http://example.org/bob> .\n"
        );
    }

    #[test]
    fn a_prefix_declared_after_the_first_triple_is_reported() {
        let mut sink = WriterSink::new(Vec::new()).with_format(&RDFFormat::Turtle).unwrap();
        sink.push(Triple::new(iri("alice"), iri("knows"), iri("bob"))).unwrap();

        let too_late = sink.declare_prefix("ex", &IriS::new_unchecked("http://example.org/"));
        assert!(matches!(
            too_late,
            Err(RdfSinkError::DeclaredTooLate { what: "prefix" })
        ));

        // Nothing is corrupted by the refusal: the output is valid Turtle,
        // just unqualified.
        sink.finish().unwrap();
        let text = String::from_utf8(sink.into_writer()).unwrap();
        assert!(!text.contains("@prefix"), "{text}");
        assert_eq!(parsed(&RDFFormat::Turtle, &text).len(), 1, "{text}");
    }

    #[test]
    fn the_base_iri_and_the_whole_map_are_reported_too() {
        let mut sink = WriterSink::new(Vec::new()).with_format(&RDFFormat::Turtle).unwrap();
        sink.push(Triple::new(iri("alice"), iri("knows"), iri("bob"))).unwrap();

        assert!(matches!(
            sink.declare_base(&IriS::new_unchecked("http://example.org/")),
            Err(RdfSinkError::DeclaredTooLate { what: "base IRI" })
        ));
        assert!(matches!(
            sink.declare_prefixes(PrefixMap::new()),
            Err(RdfSinkError::DeclaredTooLate { what: "prefixes" })
        ));
    }

    #[test]
    fn declaring_after_finish_says_the_sink_is_finished() {
        let mut sink = WriterSink::new(Vec::new()).with_format(&RDFFormat::Turtle).unwrap();
        sink.finish().unwrap();

        assert!(matches!(
            sink.declare_prefix("ex", &IriS::new_unchecked("http://example.org/")),
            Err(RdfSinkError::Finished)
        ));
    }

    /// N-Triples has no prefix syntax, so a declaration is useless there.
    #[test]
    fn a_format_with_no_prefix_syntax_accepts_the_declaration_anyway() {
        let mut sink = WriterSink::new(Vec::new());
        sink.push(Triple::new(iri("alice"), iri("knows"), iri("bob"))).unwrap();

        assert!(
            sink.declare_prefix("ex", &IriS::new_unchecked("http://example.org/"))
                .is_ok()
        );
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
    fn a_flushed_n_triples_document_is_already_complete() {
        let mut sink = WriterSink::new(FlushOnly::default());
        sink.push(triples()[2].clone()).unwrap();
        sink.flush().unwrap();

        let flushed = String::from_utf8(sink.into_writer().flushed).unwrap();
        assert_eq!(parsed(&RDFFormat::NTriples, &flushed).len(), 1);
    }

    /// The other half of the contract the doc comment states: Turtle keeps the
    /// statement open, so a flush is not yet a readable document.
    #[test]
    fn a_flushed_turtle_document_is_complete_only_after_finish() {
        let mut sink = WriterSink::new(FlushOnly::default())
            .with_format(&RDFFormat::Turtle)
            .unwrap();
        sink.push(triples()[2].clone()).unwrap();
        sink.flush().unwrap();
        let mid_run = String::from_utf8(sink.writer.flushed.clone()).unwrap();
        assert!(
            OxigraphInMemory::from_str(&mid_run, &RDFFormat::Turtle, None, &ReaderMode::Strict).is_err(),
            "{mid_run}"
        );

        sink.finish().unwrap();
        let complete = String::from_utf8(sink.into_writer().flushed).unwrap();
        assert_eq!(parsed(&RDFFormat::Turtle, &complete).len(), 1, "{complete}");
    }

    #[test]
    fn pushing_after_finish_fails() {
        let mut sink = WriterSink::new(Vec::new());
        sink.finish().unwrap();

        assert!(matches!(sink.push(triples()[2].clone()), Err(RdfSinkError::Finished)));
        assert!(matches!(sink.finish(), Err(RdfSinkError::Finished)));
    }
}
