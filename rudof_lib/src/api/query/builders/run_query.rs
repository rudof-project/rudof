use crate::{Result, Rudof, api::query::QueryOperations, formats::ResultQueryFormat};

/// Builder for `run_query` operation.
///
/// Provides a fluent interface for configuring and executing query operations
/// with optional parameters.
pub struct RunQueryBuilder<'a> {
    rudof: &'a mut Rudof,
    result_query_format: Option<&'a ResultQueryFormat>,
}

impl<'a> RunQueryBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::run_query()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof) -> Self {
        Self {
            rudof,
            result_query_format: None,
        }
    }

    /// Sets the desired format for the query results.
    ///
    /// # Arguments
    /// * `format` - The format to serialize query results.
    pub fn with_result_query_format(mut self, format: &'a ResultQueryFormat) -> Self {
        self.result_query_format = Some(format);
        self
    }

    /// Executes the query operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as QueryOperations>::run_query(self.rudof, self.result_query_format)
    }
}
