use std::io::Write;

use crate::{Result, Rudof, api::shex::ShExOperations};

/// Builder for `compile_shex_schema_to_file` operation.
///
/// Writes the currently loaded ShEx `SchemaIR` to `writer` as a precompiled
/// cache. Requires a schema to have been loaded first.
pub struct CompileShexSchemaToFileBuilder<'a, W: Write> {
    rudof: &'a Rudof,
    writer: &'a mut W,
}

impl<'a, W: Write> CompileShexSchemaToFileBuilder<'a, W> {
    /// Creates a new builder instance.
    ///
    /// This is called internally by `Rudof::compile_shex_schema_to_file()`
    /// and should not be constructed directly.
    pub(crate) fn new(rudof: &'a Rudof, writer: &'a mut W) -> Self {
        Self { rudof, writer }
    }

    /// Executes the compile-to-cache operation.
    pub fn execute(self) -> Result<()> {
        <Rudof as ShExOperations>::compile_shex_schema_to_file(self.rudof, self.writer)
    }
}
