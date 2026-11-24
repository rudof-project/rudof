use crate::api::shacl::implementations::ShaclSinks;
use crate::{
    Result, Rudof,
    api::shacl::ShaclOperations,
    formats::{ShaclFetchStrategy, ShaclValidationMode},
};
use rudof_rdf_sink::TriplesSink;

/// Builder for `validate_shacl` operation.
///
/// Provides a fluent interface for configuring and executing SHACL validation
/// operations with optional parameters.
pub struct ValidateShaclBuilder<'a> {
    rudof: &'a mut Rudof,
    mode: Option<&'a ShaclValidationMode>,
    fetch_strategy: Option<ShaclFetchStrategy>,
    sinks: ShaclSinks,
}

impl<'a> ValidateShaclBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::validate_shacl()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self {
            rudof,
            mode: None,
            fetch_strategy: None,
            sinks: ShaclSinks::default(),
        }
    }

    /// Sets the validation mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The validation mode to use
    pub fn with_shacl_validation_mode(mut self, mode: &'a ShaclValidationMode) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Sets how the data of a SPARQL endpoint is fetched, overriding the
    /// configured `[shacl] fetch_strategy` for this validation. No effect on
    /// data in memory.
    ///
    /// # Arguments
    ///
    /// * `strategy` - `OnDemand` fetches each focus node's data when the
    ///   validation needs it; `ByShape` fetches the data shape by shape
    ///   before validating, many nodes per query
    pub fn with_fetch_strategy(mut self, strategy: ShaclFetchStrategy) -> Self {
        self.fetch_strategy = Some(strategy);
        self
    }

    /// Pushes every triple the validation reads into `read_sink`, for this
    /// run only. Needs the native validation mode.
    pub fn with_read_sink(mut self, read_sink: TriplesSink) -> Self {
        self.sinks.read = Some(read_sink);
        self
    }

    /// Pushes the valid subgraph into `valid_sink` once the validation ends,
    /// for this run only: the triples read while validating the conformant
    /// `(focus node, shape)` pairs of the shapes with targets, following the
    /// conformant pairs their constraints rely on (e.g. through `sh:node`).
    /// Triples only read while checking pairs that turned out non-conformant
    /// are left out. Needs the native validation mode.
    pub fn with_valid_sink(mut self, valid_sink: TriplesSink) -> Self {
        self.sinks.valid = Some(valid_sink);
        self
    }

    /// Pushes the invalid subgraph into `invalid_sink` once the validation
    /// ends, for this run only: the triples the validation read that are not
    /// in the valid subgraph (see [`Self::with_valid_sink`]). Needs the native
    /// validation mode.
    pub fn with_invalid_sink(mut self, invalid_sink: TriplesSink) -> Self {
        self.sinks.invalid = Some(invalid_sink);
        self
    }

    /// Executes the SHACL validation operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        let no_sinks = self.sinks.read.is_none() && self.sinks.valid.is_none() && self.sinks.invalid.is_none();
        if self.fetch_strategy.is_none() && no_sinks {
            return <Rudof as ShaclOperations>::validate_shacl(self.rudof, self.mode);
        }
        crate::api::shacl::implementations::validate_shacl_with(self.rudof, self.mode, self.fetch_strategy, self.sinks)
    }
}
