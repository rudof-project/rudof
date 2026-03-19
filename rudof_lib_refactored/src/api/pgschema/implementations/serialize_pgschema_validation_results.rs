use crate::{Rudof, Result, formats::ResultPgSchemaValidationFormat};
use std::io;

pub fn serialize_pgschema_validation_results<W: io::Write>(
	rudof: &Rudof,
	result_pg_schema_validation_format: Option<&ResultPgSchemaValidationFormat>,
	writer: &mut W,
) -> Result<()> {
	todo!()
}
