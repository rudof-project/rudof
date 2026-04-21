use crate::{Result, Rudof, errors::PgSchemaError, formats::PgSchemaFormat};
use std::io;

pub fn serialize_pgschema<W: io::Write>(
    rudof: &Rudof,
    _result_pg_schema_format: Option<&PgSchemaFormat>,
    writer: &mut W,
) -> Result<()> {
    let pg_schema = rudof.pg_schema.as_ref().ok_or(PgSchemaError::NoPgschemaLoaded)?;

    write!(writer, "{pg_schema}").map_err(|e| PgSchemaError::FailedIoOperation { error: e.to_string() })?;

    Ok(())
}
