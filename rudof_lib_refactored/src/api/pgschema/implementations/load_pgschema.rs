use pgschema::parser::pgs_builder::PgsBuilder;
use crate::{Result, Rudof, errors::PgSchemaError, formats::InputSpec, formats::PgSchemaFormat};
use std::io::Read;

pub fn load_pgschema(
    rudof: &mut Rudof,
    pg_schema: &InputSpec,
    _pg_schema_format: Option<&PgSchemaFormat>,
) -> Result<()> {
    let mut pg_schema_reader =
        pg_schema
            .open_read(None, "Property Graph schema")
            .map_err(|error| PgSchemaError::DataSourceSpec {
                message: format!(
                    "Failed to open Property Graph schema source '{}': {error}",
                    pg_schema.source_name()
                ),
            })?;

    let mut pg_schema_content = String::new();
    pg_schema_reader
		.read_to_string(&mut pg_schema_content)
        .map_err(|error| PgSchemaError::DataSourceSpec {
            message: format!(
                "Failed to read Property Graph schema source '{}': {error}",
                pg_schema.source_name()
            ),
        })?;

    let pg_schema = PgsBuilder::new()
        .parse_pgs(pg_schema_content.as_str())
        .map_err(|error| PgSchemaError::FailedParsingPgSchema {
            error: error.to_string(),
        })?;

	rudof.pg_schema = Some(pg_schema);

    Ok(())
}
