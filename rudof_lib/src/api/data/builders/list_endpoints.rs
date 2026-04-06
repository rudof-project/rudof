use crate::{Result, Rudof, api::data::DataOperations};

/// Builder for `list_endpoints` operation.
///
/// Provides a fluent interface for configuring and executing endpoint listing
/// operations.
pub struct ListEndpointsBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ListEndpointsBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::list_endpoints()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the endpoint listing operation.
    pub fn execute(self) -> Result<Vec<(String, String)>> {
        <Rudof as DataOperations>::list_endpoints(self.rudof)
    }
}
