use crate::{
    Result, Rudof,
    errors::{DataError, ShaclError},
    formats::ShaclValidationMode,
    types::Data,
};
use shacl_ir::compiled::schema_ir::SchemaIR as ShaclSchemaIR;
use shacl_validation::{shacl_processor::{GraphValidation, ShaclProcessor}, store::graph::Graph};

pub fn validate_shacl(rudof: &mut Rudof, mode: Option<&ShaclValidationMode>) -> Result<()> {
    let (data, shacl_schema_ir) = validate_loaded_data_schema_and_shapes(rudof)?;

    let mode = mode.copied().unwrap_or_default();

    let mut validator = GraphValidation::from_graph(Graph::from_data(data.unwrap_rdf_mut().clone()), mode.into());

    let result =
        ShaclProcessor::validate(&mut validator, shacl_schema_ir).map_err(|e| ShaclError::FailedShaclValidation {
            error: e.to_string(),
        })?;
    
    rudof.shacl_validation_results = Some(result);

    Ok(())
}

fn validate_loaded_data_schema_and_shapes(rudof: &mut Rudof) -> Result<(&mut Data, &ShaclSchemaIR)> {
    let data = rudof.data.as_mut().ok_or(DataError::NoDataLoaded)?;

    if !data.is_rdf() {
        Err(DataError::NoRdfDataLoaded)?
    }

    let shacl_schema_ir = rudof.shacl_shapes_ir.as_ref().ok_or(ShaclError::NoShaclShapesLoaded)?;

    Ok((data, shacl_schema_ir))
}
