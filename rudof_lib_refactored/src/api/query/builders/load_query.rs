use crate::{Rudof, Result, api::query::QueryOperations, formats::{InputSpec, QueryType}};

/// Builder for `load_query` operation.
///
/// Provides a fluent interface for configuring and executing query loading
/// operations.
pub struct LoadQueryBuilder<'a> {
    rudof: &'a mut Rudof,
    query: &'a InputSpec,
    query_type: &'a QueryType,
}

impl<'a> LoadQueryBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::load_query()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, query: &'a InputSpec, query_type: &'a QueryType) -> Self {
        Self {
            rudof,
            query,
            query_type,
        }
    }

    /// Executes the query loading operation.
    ///
    /// # Errors
    ///
    /// Returns an error if the query cannot be parsed or loaded.
    pub fn execute(self) -> Result<()> {
        <Rudof as QueryOperations>::load_query(self.rudof, self.query, self.query_type)
    }
}
