use crate::{Rudof, Result, formats::InputSpec, formats::PgSchemaFormat};

pub fn load_pgschema(
	rudof: &mut Rudof,
	pg_schema: &InputSpec,
	pg_schema_format: Option<&PgSchemaFormat>
) -> Result<()> {
	todo!()
}
