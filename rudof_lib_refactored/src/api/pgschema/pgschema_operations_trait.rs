use crate::{
    Result,
    formats::{InputSpec, ResultPgSchemaValidationFormat},
    api::pgschema::implementations::{
        load_pgschema, serialize_pgschema, reset_pgschema, run_pgschema_validation,
        serialize_pgschema_validation_results, reset_pgschema_validation,
    }
};
use std::io;

/// Operations for Property Graph schema management and validation.
pub trait PgSchemaOperations {
    /// Loads a Property Graph schema from an input specification.
    ///
    /// # Arguments
    ///
    /// * `pg_schema` - Input specification defining the Property Graph schema source
    /// * `base_schema` - Optional base IRI for resolving relative IRIs in the schema
    ///
    /// # Errors
    ///
    /// Returns an error if the Property Graph schema cannot be parsed or loaded.
    fn load_pgschema(
        &mut self,
        pg_schema: &InputSpec
    ) -> Result<()>;

    /// Serializes the current Property Graph schema to a writer.
    ///
    /// # Arguments
    ///
    /// * `writer` - The destination to write the serialized Property Graph schema to
    ///
    /// # Errors
    ///
    /// Returns an error if no Property Graph schema is loaded or serialization fails.
    fn serialize_pgschema<W: io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current Property Graph schema.
    fn reset_pgschema(&mut self);

    /// Runs validation on the Property Graph schema using the loaded pgschema and shape map.
    ///
    /// # Errors
    ///
    /// Returns an error if no pgschema or shapemap is loaded.
    fn validate_pgschema(&mut self) -> Result<()>;

    /// Serializes the Property Graph schema validation results to a writer.
    ///
    /// # Arguments
    ///
    /// * `result_format` - Optional output format for the validation results (uses default if None)
    /// * `writer` - The destination to write the serialized validation results to
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available or serialization fails.
    fn serialize_pgschema_validation_results<W: io::Write>(
        &self,
        result_format: Option<&ResultPgSchemaValidationFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the Property Graph schema validation.
    fn reset_pgschema_validation(&mut self);
}

impl PgSchemaOperations for crate::Rudof {
    fn load_pgschema(
        &mut self,
        pg_schema: &InputSpec,
    ) -> Result<()> {
        load_pgschema(self, pg_schema)
    }

    fn serialize_pgschema<W: io::Write>(
        &self,
        writer: &mut W,
    ) -> Result<()> {
        serialize_pgschema(self, writer)
    }

    fn reset_pgschema(&mut self) {
        reset_pgschema(self)
    }

    fn validate_pgschema(&mut self) -> Result<()> {
        run_pgschema_validation(self)
    }

    fn serialize_pgschema_validation_results<W: io::Write>(
        &self,
        result_format: Option<&ResultPgSchemaValidationFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_pgschema_validation_results(self, result_format, writer)
    }

    fn reset_pgschema_validation(&mut self) {
        reset_pgschema_validation(self)
    }
}