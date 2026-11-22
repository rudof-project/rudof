use crate::{Result, Rudof, api::shex::ShExOperations};
use rudof_rdf_sink::TriplesSink;

/// Builder for `validate_shex` operation.
///
/// Provides a fluent interface for configuring and executing ShEx validation
/// operations.
pub struct ValidateShexBuilder<'a> {
    rudof: &'a mut Rudof,
    read_sink: Option<TriplesSink>,
    valid_sink: Option<TriplesSink>,
    invalid_sink: Option<TriplesSink>,
}

impl<'a> ValidateShexBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::validate_shex()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self {
            rudof,
            read_sink: None,
            valid_sink: None,
            invalid_sink: None,
        }
    }

    /// Pushes every triple the validator reads into `read_sink`, for this run
    /// only.
    pub fn with_read_sink(mut self, read_sink: TriplesSink) -> Self {
        self.read_sink = Some(read_sink);
        self
    }

    /// Pushes the valid subgraph into `valid_sink` once the validation ends, for
    /// this run only: the triples matched by the conformant node/shape pairs,
    /// following the pairs their shape references rely on. Triples that were
    /// only read while checking pairs that turned out non-conformant, or that no
    /// triple constraint matched, are left out.
    pub fn with_valid_sink(mut self, valid_sink: TriplesSink) -> Self {
        self.valid_sink = Some(valid_sink);
        self
    }

    /// Pushes the invalid subgraph into `invalid_sink` once the validation ends,
    /// for this run only: the triples the validator read that are not in the
    /// valid subgraph (see [`Self::with_valid_sink`]).
    pub fn with_invalid_sink(mut self, invalid_sink: TriplesSink) -> Self {
        self.invalid_sink = Some(invalid_sink);
        self
    }

    /// Executes the ShEx validation operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::validate_shex(self.rudof, self.read_sink, self.valid_sink, self.invalid_sink)
    }
}
