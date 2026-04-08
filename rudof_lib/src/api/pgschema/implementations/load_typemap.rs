use pgschema::parser::map_builder::MapBuilder;

use crate::{Result, Rudof, errors::PgSchemaError, formats::InputSpec};
use std::io::Read;

pub fn load_typemap(rudof: &mut Rudof, typemap: &InputSpec) -> Result<()> {
    let mut typemap_reader =
        typemap
            .open_read(None, "Property Graph schema")
            .map_err(|error| PgSchemaError::DataSourceSpec {
                message: format!(
                    "Failed to open Property Graph schema source '{}': {error}",
                    typemap.source_name()
                ),
            })?;

    let mut typemap_content = String::new();
    typemap_reader
        .read_to_string(&mut typemap_content)
        .map_err(|error| PgSchemaError::DataSourceSpec {
            message: format!("Failed to read typemap source '{}': {error}", typemap.source_name()),
        })?;

    let typemap =
        MapBuilder::new()
            .parse_map(typemap_content.as_str())
            .map_err(|error| PgSchemaError::FailedParsingTypemap {
                error: error.to_string(),
            })?;

    rudof.typemap = Some(typemap);

    Ok(())
}
