use crate::{Rudof, api::query::QueryOperations};

/// Builder for `reset_sparql_query` operation.
///
/// Provides a fluent interface for configuring and executing query reset
/// operations.
pub struct ResetSparqlQueryBuilder<'a> {
    rudof: &'a mut Rudof,
}

impl<'a> ResetSparqlQueryBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::reset_sparql_query()` and should
    /// not be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self { rudof }
    }

    /// Executes the query reset operation.
    pub fn execute(self) {
        <Rudof as QueryOperations>::reset_sparql_query(self.rudof)
    }
}
