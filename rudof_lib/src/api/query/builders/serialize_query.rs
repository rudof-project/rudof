use crate::{Result, Rudof, api::query::QueryOperations};
use std::io;

/// Builder for `serialize_query` operation.
///
/// Provides a fluent interface for configuring and executing query serialization
/// operations.
pub struct SerializeQueryBuilder<'a, W: io::Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
}

impl<'a, W: io::Write> SerializeQueryBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::serialize_query()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self { rudof, writer }
    }

    /// Executes the query serialization operation.
    ///
    /// # Errors
    ///
    /// Returns an error if no query is loaded or serialization fails.
    pub fn execute(self) -> Result<()> {
        <Rudof as QueryOperations>::serialize_query(self.rudof, self.writer)
    }
}
