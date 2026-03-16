use crate::{Rudof, Result, api::shacl::ShaclOperations, formats::{InputSpec, ShaclFormat, DataReaderMode}};

/// Builder for `load_shacl_schema` operation.
///
/// Provides a fluent interface for configuring and executing schema loading
/// operations with optional parameters.
pub struct LoadShaclSchemaBuilder<'a> {
    rudof: &'a mut Rudof,
    schema: &'a InputSpec,
    schema_format: Option<&'a ShaclFormat>,
    base: Option<&'a str>,
    reader_mode: Option<&'a DataReaderMode>,
}

impl<'a> LoadShaclSchemaBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::load_shacl_schema()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, schema: &'a InputSpec) -> Self {
        Self {
            rudof,
            schema,
            schema_format: None,
            base: None,
            reader_mode: None,
        }
    }

    /// Sets the SHACL schema format.
    ///
    /// # Arguments
    ///
    /// * `schema_format` - The format to use when loading the schema
    pub fn with_schema_format(mut self, schema_format: &'a ShaclFormat) -> Self {
        self.schema_format = Some(schema_format);
        self
    }

    /// Sets the base IRI for resolving relative IRIs.
    ///
    /// # Arguments
    ///
    /// * `base` - The base IRI to use for resolution
    pub fn with_base(mut self, base: &'a str) -> Self {
        self.base = Some(base);
        self
    }

    /// Sets the reader mode for schema loading.
    ///
    /// # Arguments
    ///
    /// * `reader_mode` - The reading mode to apply during schema loading
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Executes the schema loading operation with the configured parameters.
    ///
    /// # Errors
    ///
    /// Returns an error if the schema cannot be loaded or parsed.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShaclOperations>::load_shacl_schema(
            self.rudof,
            self.schema,
            self.schema_format,
            self.base,
            self.reader_mode,
        )
    }
}
