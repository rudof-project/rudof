use crate::{Rudof, api::query::QueryOperations};

/// Builder for `reset_query_results` operation.
///
/// Provides a fluent interface for configuring and executing query results reset
/// operations.
pub struct ResetQueryResultsBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetQueryResultsBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_query_results()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the query results reset operation.
    pub fn execute(self) {
        <Rudof as QueryOperations>::reset_query_results(self.rudof)
    }
}
