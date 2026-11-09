use crate::{Result, Rudof, api::shex::ShExOperations};
use shex_validation::ReadSink;

/// Builder for `validate_shex` operation.
///
/// Provides a fluent interface for configuring and executing ShEx validation
/// operations.
pub struct ValidateShexBuilder<'a> {
    rudof: &'a mut Rudof,
    read_sink: Option<ReadSink>,
}

impl<'a> ValidateShexBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::validate_shex()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof, read_sink: None }
    }

    /// Pushes every triple the validator reads into `read_sink`, for this run
    /// only.
    pub fn with_read_sink(mut self, read_sink: ReadSink) -> Self {
        self.read_sink = Some(read_sink);
        self
    }

    /// Executes the ShEx validation operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::validate_shex(self.rudof, self.read_sink)
    }
}
