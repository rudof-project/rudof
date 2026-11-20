use crate::{Result, Rudof, api::shex::ShExOperations, types::ShExPathMatch};

/// Builder for `evaluate_shex_path` operation.
///
/// Provides a fluent interface for evaluating a ShExPath against the loaded ShEx schema.
pub struct EvaluateShExPathBuilder<'a> {
    rudof: &'a Rudof,
    path: &'a str,
}

impl<'a> EvaluateShExPathBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::evaluate_shex_path()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, path: &'a str) -> Self {
        Self { rudof, path }
    }

    /// Executes the evaluation, returning the selected schema elements in order.
    pub fn execute(self) -> Result<Vec<ShExPathMatch>> {
        <Rudof as ShExOperations>::evaluate_shex_path(self.rudof, self.path)
    }
}
