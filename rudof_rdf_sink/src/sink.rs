use oxrdf::Triple;
use prefixmap::PrefixMap;
use rudof_iri::IriS;

use crate::{RdfSinkError, SinkReport};

/// Where a producer of RDF triples puts them. The producer pushes; the caller
/// chose what happens to them.
pub trait RdfSink {
    /// Declares a namespace prefix. A sink whose destination has no syntax for
    /// one drops it and answers `Ok`: the declaration is useless there, which is not a
    /// failure.
    ///
    /// # Errors
    ///
    /// [`RdfSinkError::DeclaredTooLate`] when the sink *would* have used it but
    /// can no longer: a Turtle writer has already written its prelude.
    /// [`RdfSinkError::Finished`] after [`Self::finish`].
    fn declare_prefix(&mut self, _alias: &str, _iri: &IriS) -> Result<(), RdfSinkError> {
        Ok(())
    }

    /// Declares every prefix of `prefixes` at once, so a producer that already
    /// holds a prefix map hands it over whole.
    fn declare_prefixes(&mut self, prefixes: PrefixMap) -> Result<(), RdfSinkError> {
        for (alias, iri) in prefixes.iter() {
            self.declare_prefix(alias, iri)?;
        }
        Ok(())
    }

    /// Declares the base IRI, and fails for the same reasons as
    /// [`Self::declare_prefix`].
    fn declare_base(&mut self, _base: &IriS) -> Result<(), RdfSinkError> {
        Ok(())
    }

    /// Consumes one triple.
    fn push(&mut self, triple: Triple) -> Result<(), RdfSinkError>;

    /// Pushes out whatever is buffered, without ending the stream: everything
    /// pushed so far has reached the destination.
    fn flush(&mut self) -> Result<(), RdfSinkError> {
        Ok(())
    }

    /// Ends the stream and reports what went through. Does what [`Self::flush`]
    /// would have done first. Calling it twice, or pushing after it, fails with
    /// [`RdfSinkError::Finished`].
    fn finish(&mut self) -> Result<SinkReport, RdfSinkError>;

    /// What has gone through so far, without ending the stream.
    fn report(&self) -> SinkReport;
}
