use crate::{
    Result, Rudof,
    api::shex::ShExOperations,
    formats::{DataReaderMode, InputSpec},
};

/// Builder for `load_shex_schema_precompiled` operation.
///
/// Loads a precompiled ShEx `SchemaIR` cache into the internal state,
/// skipping parsing, imports, and AST to IR compilation, as well as negative cycles detection.
pub struct LoadShexSchemaPrecompiledBuilder<'a> {
    rudof: &'a mut Rudof,
    schema: &'a InputSpec,
    reader_mode: Option<&'a DataReaderMode>,
}

impl<'a> LoadShexSchemaPrecompiledBuilder<'a> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::load_shex_schema_precompiled()`
    /// and should not be constructed directly.
    pub(crate) fn new(rudof: &'a mut Rudof, schema: &'a InputSpec) -> Self {
        Self {
            rudof,
            schema,
            reader_mode: None,
        }
    }

    /// Sets the reader mode used to validate the cache.
    ///
    /// If `Strict` and the cache reports negation cycles, loading fails.
    /// If `Lax`, loading succeeds regardless.
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Executes the precompiled-load operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::load_shex_schema_precompiled(self.rudof, self.schema, self.reader_mode)
    }
}
