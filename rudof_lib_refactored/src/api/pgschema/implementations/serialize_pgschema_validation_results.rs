use crate::{Result, Rudof, errors::PgSchemaError, formats::ResultPgSchemaValidationFormat};
use std::io;

pub fn serialize_pgschema_validation_results<W: io::Write>(
    rudof: &Rudof,
    result_pg_schema_validation_format: Option<&ResultPgSchemaValidationFormat>,
    show_colors: Option<bool>,
    writer: &mut W,
) -> Result<()> {
    let pgschema_validation_results = rudof
        .pg_schema_validation_results
        .as_ref()
        .ok_or(PgSchemaError::NoValidationResultsAvailable)?;

    let result_pg_shema_validation_format = result_pg_schema_validation_format.copied().unwrap_or_default();

    match result_pg_shema_validation_format {
        ResultPgSchemaValidationFormat::Compact => {
			write!(writer, "{}", pgschema_validation_results).map_err(|e| PgSchemaError::FailedIoOperation { error: e.to_string() })?;
		},
        ResultPgSchemaValidationFormat::Json => {
			pgschema_validation_results
				.as_json(writer)
				.map_err(|e| PgSchemaError::FailedIoOperation { error: e.to_string() })?;
		},
        ResultPgSchemaValidationFormat::Csv => {
            pgschema_validation_results
                .as_csv(writer, show_colors.unwrap_or(true))
                .map_err(|e| PgSchemaError::FailedIoOperation { error: e.to_string() })?;
        },
        ResultPgSchemaValidationFormat::Details => {
            todo!("Implement details format for Property Graph schema validation results serialization");
        },
    }

    Ok(())
}
