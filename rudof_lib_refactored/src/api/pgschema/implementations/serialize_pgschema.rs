use crate::{Rudof, Result, formats::PgSchemaFormat};
use std::io;

pub fn serialize_pgschema<W: io::Write>(
	rudof: &Rudof,
	result_pg_schema_format: Option<&PgSchemaFormat>,
	writer: &mut W,
) -> Result<()> {
	todo!()
}
