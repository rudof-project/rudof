use crate::{
    Result,
    formats::{InputSpec, ResultPgSchemaValidationFormat, PgSchemaFormat},
    api::pgschema::implementations::{
        load_pgschema, serialize_pgschema, reset_pgschema, validate_pgschema,
        serialize_pgschema_validation_results, reset_pgschema_validation,
        load_typemap, reset_typemap,
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
    /// * `pg_schema_format` - Optional format of the input Property Graph schema (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the Property Graph schema cannot be parsed or loaded.
    fn load_pgschema(
        &mut self,
        pg_schema: &InputSpec,
        pg_schema_format: Option<&PgSchemaFormat>
    ) -> Result<()>;

    /// Serializes the current Property Graph schema to a writer.
    ///
    /// # Arguments
    ///
    /// * `result_pg_schema_format` - Optional output format for the Property Graph schema (uses default if None)
    /// * `writer` - The destination to write the serialized Property Graph schema to
    ///
    /// # Errors
    ///
    /// Returns an error if no Property Graph schema is loaded or serialization fails.
    fn serialize_pgschema<W: io::Write>(
        &self,
        result_pg_schema_format: Option<&PgSchemaFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current Property Graph schema.
    fn reset_pgschema(&mut self);

    /// Loads a typemap from an input specification.
    /// 
    /// # Arguments
    /// * `typemap` - Input specification defining the typemap source
    /// 
    /// # Errors
    /// Returns an error if the typemap cannot be parsed or loaded.
    fn load_typemap(&mut self, typemap: &InputSpec) -> Result<()>;

    /// Resets the current typemap.
    fn reset_typemap(&mut self);

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
    /// * `result_pg_schema_validation_format` - Optional output format for the validation results (uses default if None)
    /// * `show_colors` - Optional flag to indicate whether to include ANSI color codes in the output (defaults to true if None)
    /// * `writer` - The destination to write the serialized validation results to
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available or serialization fails.
    fn serialize_pgschema_validation_results<W: io::Write>(
        &self,
        result_pg_schema_validation_format: Option<&ResultPgSchemaValidationFormat>,
        show_colors: Option<bool>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the Property Graph schema validation.
    fn reset_pgschema_validation(&mut self);
}

impl PgSchemaOperations for crate::Rudof {
    fn load_pgschema(
        &mut self,
        pg_schema: &InputSpec,
        pg_schema_format: Option<&PgSchemaFormat>
    ) -> Result<()> {
        load_pgschema(self, pg_schema, pg_schema_format)
    }

    fn serialize_pgschema<W: io::Write>(
        &self,
        result_pg_schema_format: Option<&PgSchemaFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_pgschema(self, result_pg_schema_format, writer)
    }

    fn reset_pgschema(&mut self) {
        reset_pgschema(self)
    }

    fn load_typemap(&mut self, typemap: &InputSpec) -> Result<()> {
        load_typemap(self, typemap)
    }

    fn reset_typemap(&mut self) {
        reset_typemap(self)
    }

    fn validate_pgschema(&mut self) -> Result<()> {
        validate_pgschema(self)
    }

    fn serialize_pgschema_validation_results<W: io::Write>(
        &self,
        result_pg_schema_validation_format: Option<&ResultPgSchemaValidationFormat>,
        show_colors: Option<bool>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_pgschema_validation_results(self, result_pg_schema_validation_format, show_colors, writer)
    }

    fn reset_pgschema_validation(&mut self) {
        reset_pgschema_validation(self)
    }
}