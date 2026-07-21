use std::io::Write;

use shex_ast::ir::cache::CacheFormat;

use crate::{Result, Rudof, errors::ShExError};

pub fn compile_shex_schema_to_file<W: Write>(
    rudof: &Rudof,
    writer: &mut W
) -> Result<()> {
    let schema_ir = rudof.shex_schema_ir.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;

    schema_ir
        .write(writer, CacheFormat::Bincode)
        .map_err(|error| ShExError::FailedWritingShExCache {
            error: error.to_string(),
        })?;

    Ok(())
}
