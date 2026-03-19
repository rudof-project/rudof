use crate::{Rudof, Result, api::shex::ShExOperations};

pub struct CheckShexSchemaBuilder<'a, W: std::io::Write> {
    rudof: &'a crate::Rudof,
    schema: &'a crate::formats::InputSpec,
    schema_format: Option<&'a crate::formats::ShExFormat>,
    base_schema: Option<&'a str>,
    writer: &'a mut W,
}

impl<'a, W: std::io::Write> CheckShexSchemaBuilder<'a, W> {
    /// Creates a new builder instance.
    /// 
    /// This is called internally by `Rudof::check_shex_schema()` and should not
    /// be constructed directly.
    pub(crate) fn new(rudof: &'a crate::Rudof, schema: &'a crate::formats::InputSpec, writer: &'a mut W) -> Self {
        Self {
            rudof,
            schema,
            schema_format: None,
            base_schema: None,
            writer,
        }
    }

    /// Sets the ShEx schema format.
    /// 
    /// # Arguments
    /// 
    /// * `schema_format` - The format to use when checking the schema
    pub fn with_shex_schema_format(mut self, schema_format: &'a crate::formats::ShExFormat) -> Self {
        self.schema_format = Some(schema_format);
        self
    }

    /// Sets the base IRI for resolving relative IRIs in the schema.
    /// 
    /// # Arguments
    /// 
    /// * `base_schema` - The base IRI to use for resolution
    pub fn with_base_schema(mut self, base_schema: &'a str) -> Self {
        self.base_schema = Some(base_schema);
        self
    }

    /// Executes the schema checking operation with the configured parameters.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::check_shex_schema(self.rudof, self.schema, self.schema_format, self.base_schema, self.writer)
    }
}