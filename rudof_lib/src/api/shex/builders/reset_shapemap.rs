use crate::{Rudof, api::shex::ShExOperations};

/// Builder for `reset_shapemap` operation.
///
/// Provides a fluent interface for configuring and executing shape map reset
/// operations.
pub struct ResetShapemapBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetShapemapBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_shapemap()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the shape map reset operation.
    pub fn execute(self) {
        <Rudof as ShExOperations>::reset_shapemap(self.rudof)
    }
}
