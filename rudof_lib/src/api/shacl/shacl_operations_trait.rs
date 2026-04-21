use crate::{
    Result,
    api::shacl::implementations::{
        load_shacl_schema, reset_shacl_schema, reset_shacl_validation, serialize_shacl_schema,
        serialize_shacl_validation_results, validate_shacl,
    },
    formats::{
        DataReaderMode, InputSpec, ResultShaclValidationFormat, ShaclFormat, ShaclValidationMode,
        ShaclValidationSortByMode,
    },
};
use std::io;

/// Operations for SHACL (Shapes Constraint Language) validation.
pub trait ShaclOperations {
    /// Loads a SHACL schema from an input specification or from the currently loaded data.
    ///
    /// If `schema` is provided, the method loads the SHACL shapes from the specified input
    /// using the optional `schema_format`, `base`, and `reader_mode`.
    /// If `schema` is `None`, the method treats the currently loaded RDF data graph
    /// as the SHACL shapes graph.
    ///
    /// # Arguments
    ///
    /// * `schema` - Optional input specification defining the schema source
    /// * `schema_format` - Optional SHACL format (uses default if None)
    /// * `base` - Optional base IRI for resolving relative IRIs (uses default if None)
    /// * `reader_mode` - Optional parsing mode (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be parsed or loaded.
    fn load_shacl_schema(
        &mut self,
        schema: Option<&InputSpec>,
        schema_format: Option<&ShaclFormat>,
        base: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()>;

    /// Serializes the current SHACL schema to a writer.
    ///
    /// # Arguments
    ///
    /// * `shacl_format` - Optional output format (uses default if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no schema is loaded or serialization fails.
    fn serialize_shacl_schema<W: io::Write>(&self, shacl_format: Option<&ShaclFormat>, writer: &mut W) -> Result<()>;

    /// Resets the SHACL schema.
    fn reset_shacl_schema(&mut self);

    /// Validates the current RDF data using the loaded SHACL schema and shapes.
    ///
    /// If no shapes are explicitly loaded, the validation assumes that the shapes
    /// are defined within the SHACL schema itself.
    ///
    /// # Arguments
    ///
    /// * `mode` - Optional validation mode (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if no SHACL schema or shapes is loaded.
    fn validate_shacl(&mut self, mode: Option<&ShaclValidationMode>) -> Result<()>;

    /// Serializes the SHACL validation results to a writer.
    ///
    /// # Arguments
    ///
    /// * `shacl_validation_sort_order_mode` - Optional sorting mode for the validation results (uses default order if None)
    /// * `result_shacl_validation_format` - Optional output format for validation results (uses default if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available.
    fn serialize_shacl_validation_results<W: io::Write>(
        &self,
        shacl_validation_sort_order_mode: Option<&ShaclValidationSortByMode>,
        result_shacl_validation_format: Option<&ResultShaclValidationFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the SHACL validation.
    fn reset_shacl_validation(&mut self);
}

impl ShaclOperations for crate::Rudof {
    fn load_shacl_schema(
        &mut self,
        schema: Option<&InputSpec>,
        schema_format: Option<&ShaclFormat>,
        base: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()> {
        load_shacl_schema(self, schema, schema_format, base, reader_mode)
    }

    fn serialize_shacl_schema<W: io::Write>(&self, shacl_format: Option<&ShaclFormat>, writer: &mut W) -> Result<()> {
        serialize_shacl_schema(self, shacl_format, writer)
    }

    fn reset_shacl_schema(&mut self) {
        reset_shacl_schema(self)
    }

    fn validate_shacl(&mut self, mode: Option<&ShaclValidationMode>) -> Result<()> {
        validate_shacl(self, mode)
    }

    fn serialize_shacl_validation_results<W: io::Write>(
        &self,
        shacl_validation_sort_order_mode: Option<&ShaclValidationSortByMode>,
        result_shacl_validation_format: Option<&ResultShaclValidationFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_shacl_validation_results(
            self,
            shacl_validation_sort_order_mode,
            result_shacl_validation_format,
            writer,
        )
    }

    fn reset_shacl_validation(&mut self) {
        reset_shacl_validation(self)
    }
}
