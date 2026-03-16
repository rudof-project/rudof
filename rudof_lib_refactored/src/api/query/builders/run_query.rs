use crate::{Rudof, Result, api::query::QueryOperations};

/// Builder for `run_query` operation.
///
/// Provides a fluent interface for configuring and executing query operations
/// with optional parameters.
pub struct RunQueryBuilder<'a> {
    rudof: &'a mut Rudof,
    endpoint: Option<&'a str>,
}

impl<'a> RunQueryBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::run_query()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self {
            rudof,
            endpoint: None,
        }
    }

    /// Sets the SPARQL endpoint to query.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The name or URL of the SPARQL endpoint
    pub fn with_endpoint(mut self, endpoint: &'a str) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    /// Executes the query operation with the configured parameters.
    ///
    /// If no endpoint is specified, the query is executed against the loaded RDF data.
    ///
    /// # Errors
    ///
    /// Returns an error if no query is loaded or query execution fails.
    pub fn execute(self) -> Result<()> {
        <Rudof as QueryOperations>::run_query(self.rudof, self.endpoint)
    }
}
