use crate::{Rudof, api::query::QueryOperations};

/// Builder for `reset_query` operation.
///
/// Provides a fluent interface for configuring and executing query reset
/// operations.
pub struct ResetQueryBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetQueryBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_query()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the query reset operation.
    pub fn execute(self) {
        <Rudof as QueryOperations>::reset_query(self.rudof)
    }
}
