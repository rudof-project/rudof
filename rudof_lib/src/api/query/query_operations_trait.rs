use crate::{
    Result,
    api::query::implementations::{
        load_query, reset_query, reset_query_results, run_query, serialize_query, serialize_query_results,
    },
    formats::{InputSpec, QueryType, ResultQueryFormat},
};
use std::io;

/// Operations for executing SPARQL queries.
pub trait QueryOperations {
    /// Loads a SPARQL query from an input specification.
    ///
    /// # Arguments
    ///
    /// * `query` - Input specification defining the query source
    /// * `query_type` - Optional type of query (SELECT, CONSTRUCT, etc.).
    ///   If None, Rudof will attempt to auto-detect the query type.
    ///
    /// # Errors
    ///
    /// Returns an error if the query cannot be parsed or loaded.
    fn load_query(&mut self, query: &InputSpec, query_type: Option<&QueryType>) -> Result<()>;

    /// Serializes the currently loaded query to a writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The destination to write the serialized query to
    ///
    /// # Errors
    ///
    /// Returns an error if no query is loaded or serialization fails.
    fn serialize_query<W: io::Write>(&self, writer: &mut W) -> Result<()>;

    /// Resets the current query.
    fn reset_query(&mut self);

    /// Executes the currently loaded query.
    ///
    /// # Arguments
    ///
    /// * `result_query_format` - Optional format for the query results.
    ///
    /// # Errors
    ///
    /// Returns an error if no query is loaded, or if query execution fails.
    fn run_query(&mut self, result_query_format: Option<&ResultQueryFormat>) -> Result<()>;

    /// Serializes the query results to a writer.
    ///
    /// # Arguments
    ///
    /// * `result_format` - Optional output format for the results (uses default if None)
    /// * `writer` - The destination to write the serialized results to
    ///
    /// # Errors
    ///
    /// Returns an error if no query results are available or serialization fails.
    fn serialize_query_results<W: io::Write>(
        &self,
        result_query_format: Option<&ResultQueryFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current query and results.
    fn reset_query_results(&mut self);
}

impl QueryOperations for crate::Rudof {
    fn load_query(&mut self, query: &InputSpec, query_type: Option<&QueryType>) -> Result<()> {
        load_query(self, query, query_type)
    }

    fn serialize_query<W: io::Write>(&self, writer: &mut W) -> Result<()> {
        serialize_query(self, writer)
    }

    fn reset_query(&mut self) {
        reset_query(self)
    }

    fn run_query(&mut self, result_query_format: Option<&ResultQueryFormat>) -> Result<()> {
        run_query(self, result_query_format)
    }

    fn serialize_query_results<W: io::Write>(
        &self,
        result_query_format: Option<&ResultQueryFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_query_results(self, result_query_format, writer)
    }

    fn reset_query_results(&mut self) {
        reset_query_results(self)
    }
}
