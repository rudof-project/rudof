use crate::{Rudof, Result, api::query::QueryOperations, formats::ResultQueryFormat};
use std::io;

/// Builder for `serialize_query_results` operation.
///
/// Provides a fluent interface for configuring and executing query results
/// serialization operations with optional parameters.
pub struct SerializeQueryResultsBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
    result_format: Option<&'a ResultQueryFormat>,
}

impl<'a, W: io::Write> SerializeQueryResultsBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_query_results()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self {
            rudof,
            writer,
            result_format: None,
        }
    }

    /// Sets the output format for query results.
    ///
    /// # Arguments
    ///
    /// * `result_format` - The format to use when serializing the results
    pub fn with_result_format(mut self, result_format: &'a ResultQueryFormat) -> Self {
        self.result_format = Some(result_format);
        self
    }

    /// Executes the query results serialization operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if no query results are available or serialization fails.
    pub fn execute(self) -> Result<()> {
        <Rudof as QueryOperations>::serialize_query_results(
            self.rudof,
            self.result_format,
            self.writer,
        )
    }
}
