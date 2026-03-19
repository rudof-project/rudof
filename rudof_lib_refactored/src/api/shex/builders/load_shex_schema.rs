use crate::{Rudof, Result, api::shex::ShExOperations, formats::{InputSpec, ShExFormat, DataReaderMode}};

/// Builder for `load_shex_schema` operation.
///
/// Provides a fluent interface for configuring and executing schema loading
/// operations with optional parameters.
pub struct LoadShexSchemaBuilder<'a> {
    rudof: &'a mut Rudof,
    schema: &'a InputSpec,
    schema_format: Option<&'a ShExFormat>,
    base_schema: Option<&'a str>,
    reader_mode: Option<&'a DataReaderMode>,
}

impl<'a> LoadShexSchemaBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::load_shex_schema()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, schema: &'a InputSpec) -> Self {
        Self {
            rudof,
            schema,
            schema_format: None,
            base_schema: None,
            reader_mode: None,
        }
    }

    /// Sets the ShEx schema format.
    ///
    /// # Arguments
    ///
    /// * `schema_format` - The format to use when loading the schema
    pub fn with_shex_schema_format(mut self, schema_format: &'a ShExFormat) -> Self {
        self.schema_format = Some(schema_format);
        self
    }

    /// Sets the base IRI for resolving relative IRIs in the schema.
    ///
    /// # Arguments
    ///
    /// * `base_schema` - The base IRI to use for resolution
    pub fn with_base(mut self, base_schema: &'a str) -> Self {
        self.base_schema = Some(base_schema);
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
        <Rudof as ShExOperations>::load_shex_schema(
            self.rudof,
            self.schema,
            self.schema_format,
            self.base_schema,
            self.reader_mode,
        )
    }
}
