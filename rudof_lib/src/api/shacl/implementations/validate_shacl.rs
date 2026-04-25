use crate::{
    Result, Rudof,
    errors::{DataError, ShaclError},
    formats::ShaclValidationMode,
    types::Data,
};
use shacl::ir::IRSchema;
use shacl::validator::processor::{GraphValidation, ShaclProcessor};
use shacl::validator::store::Graph;

pub fn validate_shacl(rudof: &mut Rudof, mode: Option<&ShaclValidationMode>) -> Result<()> {
    let (data, shacl_schema_ir) = validate_loaded_data_schema_and_shapes(rudof)?;

    let mode = mode.copied().unwrap_or_default();

    let graph: Graph = data.unwrap_rdf_mut().clone().into();
    let mut validator: GraphValidation = graph.into();

    let result = ShaclProcessor::validate(&mut validator, shacl_schema_ir, &mode.into())
        .map_err(|e| ShaclError::FailedShaclValidation { error: e.to_string() })?;

    rudof.shacl_validation_results = Some(result);

    Ok(())
}

fn validate_loaded_data_schema_and_shapes(rudof: &mut Rudof) -> Result<(&mut Data, &IRSchema)> {
    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoDataLoaded))?;

    if !data.is_rdf() {
        Err(Box::new(DataError::NoRdfDataLoaded))?
    }

    let shacl_schema_ir = rudof.shacl_shapes_ir.as_ref().ok_or(ShaclError::NoShaclShapesLoaded)?;

    Ok((data, shacl_schema_ir))
}
