use crate::{
    Result, Rudof,
    errors::{DataError, ShExError},
    types::Data,
};
use shex_ast::{ir::schema_ir::SchemaIR as ShExSchemaIR, shapemap::QueryShapeMap};
use shex_validation::Validator as ShExValidator;

pub fn validate_shex(rudof: &mut Rudof) -> Result<()> {
    let (data, shex_schema, shapemap, shex_validator) = validate_loaded_data_schema_and_shapemap(rudof)?;
    let rdf_data = data.unwrap_rdf_mut();

    let result = shex_validator
        .validate_shapemap(shapemap, rdf_data, shex_schema, &Some(rdf_data.prefixmap_in_memory()))
        .map_err(|e| ShExError::FailedShExValidation { error: e.to_string() })?;

    rudof.shex_validation_results = Some(result);

    Ok(())
}

fn validate_loaded_data_schema_and_shapemap(
    rudof: &mut Rudof,
) -> Result<(&mut Data, &ShExSchemaIR, &QueryShapeMap, &ShExValidator)> {
    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoDataLoaded))?;

    if !data.is_rdf() {
        Err(Box::new(DataError::NoRdfDataLoaded))?
    }

    let shex_schema_ir = rudof.shex_schema_ir.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;
    let shex_validator = rudof.shex_validator.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;

    let shapemap = rudof.shapemap.as_ref().ok_or(ShExError::NoShapemapLoaded)?;

    Ok((data, shex_schema_ir, shapemap, shex_validator))
}
