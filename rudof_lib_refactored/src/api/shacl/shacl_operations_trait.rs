use crate::{
    Result,
    formats::{InputSpec, ShaclFormat, DataReaderMode, ShaclValidationMode, ShaclValidationSortByMode},
    api::shacl::implementations::{
        load_shacl_schema, load_shapes, reset_shacl_validation, reset_shapes, reset_shacl_schema,
        serialize_shacl_schema, serialize_shacl_validation_results, serialize_shapes, validate_shacl
    }
};
use std::io;

/// Operations for SHACL (Shapes Constraint Language) validation.
pub trait ShaclOperations {
    /// Loads a SHACL schema from an input specification.
    ///
    /// # Arguments
    ///
    /// * `schema` - Input specification defining the schema source
    /// * `schema_format` - Optional SHACL format (uses default if None)
    /// * `base` - Optional base IRI for resolving relative IRIs (uses default if None)
    /// * `reader_mode` - The parsing mode (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be parsed or loaded.
    fn load_shacl_schema(
        &mut self,
        schema: &InputSpec,
        schema_format: Option<&ShaclFormat>,
        base: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()>;

    /// Serializes the current SHACL schema to a writer.
    ///
    /// # Arguments
    ///
    /// * `format` - Optional output format (uses default if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no schema is loaded or serialization fails.
    fn serialize_shacl_schema<W: io::Write>(
        &self, 
        format: Option<&ShaclFormat>, 
        writer: &mut W
    ) -> Result<()>;

    /// Resets the SHACL schema.
    fn reset_shacl_schema(&mut self);

    /// Loads SHACL shapes from an input specification.
    ///
    /// # Arguments
    ///
    /// * `shapes` - Input specification defining the shapes source
    /// * `format` - Optional SHACL format (uses default if None)
    /// * `base` - Optional base IRI for resolving relative IRIs (uses default if None)
    /// * `reader_mode` - The parsing mode (uses default if None)
    ///
    /// # Errors
    ///
    /// Returns an error if the shapes cannot be parsed or loaded.
    fn load_shapes(
        &mut self,
        shapes: &InputSpec,
        format: Option<&ShaclFormat>,
        base: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()>;

    /// Serializes the current SHACL shapes to a writer.
    ///
    /// # Arguments
    ///
    /// * `format` - Optional output format (uses default if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no shapes are loaded or serialization fails.
    fn serialize_shapes<W: io::Write>(
        &self,
        format: Option<&ShaclFormat>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the current SHACL shapes.
    fn reset_shapes(&mut self);

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
    fn validate_shacl(
        &mut self, 
        mode: Option<&ShaclValidationMode>,
    ) -> Result<()>;

    /// Serializes the SHACL validation results to a writer.
    ///
    /// # Arguments
    ///
    /// * `sort_order` - Optional sorting mode for the validation results (uses default order if None)
    /// * `writer` - The destination to write to
    ///
    /// # Errors
    ///
    /// Returns an error if no validation results are available.
    fn serialize_shacl_validation_results<W: io::Write>(
        &self,
        sort_order: Option<&ShaclValidationSortByMode>,
        writer: &mut W,
    ) -> Result<()>;

    /// Resets the SHACL validation.
    fn reset_shacl_validation(&mut self);
}

impl ShaclOperations for crate::Rudof {
    fn load_shacl_schema(
        &mut self,
        schema: &InputSpec,
        schema_format: Option<&ShaclFormat>,
        base: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()> {
        load_shacl_schema(self, schema, schema_format, base, reader_mode)
    }

    fn serialize_shacl_schema<W: io::Write>(
        &self, 
        format: Option<&ShaclFormat>, 
        writer: &mut W
    ) -> Result<()> {
        serialize_shacl_schema(self, format, writer)
    }

    fn reset_shacl_schema(&mut self) {
        reset_shacl_schema(self)
    }

    fn load_shapes(
        &mut self,
        shapes: &InputSpec,
        format: Option<&ShaclFormat>,
        base: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<()> {
        load_shapes(self, shapes, format, base, reader_mode)
    }

    fn serialize_shapes<W: io::Write>(
        &self,
        format: Option<&ShaclFormat>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_shapes(self, format, writer)
    }

    fn reset_shapes(&mut self) {
        reset_shapes(self)
    }

    fn validate_shacl(
        &mut self, 
        mode: Option<&ShaclValidationMode>,
    ) -> Result<()> {
        validate_shacl(self, mode)
    }

    fn serialize_shacl_validation_results<W: io::Write>(
        &self,
        sort_order: Option<&ShaclValidationSortByMode>,
        writer: &mut W,
    ) -> Result<()> {
        serialize_shacl_validation_results(self, sort_order, writer)
    }

    fn reset_shacl_validation(&mut self) {
        reset_shacl_validation(self)
    }
}