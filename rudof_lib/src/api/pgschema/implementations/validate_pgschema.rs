use crate::{
    Result, Rudof,
    errors::{DataError, PgSchemaError},
    types::Data,
};
use pgschema::{pgs::PropertyGraphSchema, type_map::TypeMap};

pub fn validate_pgschema(rudof: &mut Rudof) -> Result<()> {
    let (data, pg_schema, typemap) = validate_loaded_data_schema_and_typemap(rudof)?;

    let pg_schema_validation_results =
        typemap
            .validate(pg_schema, data.unwrap_pg_mut())
            .map_err(|error| PgSchemaError::FailedPgschemaValidation {
                error: error.to_string(),
            })?;

    rudof.pg_schema_validation_results = Some(pg_schema_validation_results);

    Ok(())
}

fn validate_loaded_data_schema_and_typemap(rudof: &mut Rudof) -> Result<(&mut Data, &PropertyGraphSchema, &TypeMap)> {
    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoDataLoaded))?;

    if !data.is_pg() {
        Err(Box::new(DataError::NoPgDataLoaded))?
    }

    let pg_schema = rudof.pg_schema.as_ref().ok_or(PgSchemaError::NoPgschemaLoaded)?;

    let typemap = rudof.typemap.as_ref().ok_or(PgSchemaError::NoTypemapLoaded)?;

    Ok((data, pg_schema, typemap))
}
