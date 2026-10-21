use crate::{
    Result, Rudof,
    errors::{DataError, ShaclError},
    formats::{ShaclFetchStrategy, ShaclValidationMode},
    types::Data,
};
use shacl::ir::IRSchema;
use shacl::validator::processor::{CachedDataValidation, GraphValidation, ShaclProcessor};
use shacl::validator::store::Graph;

pub fn validate_shacl(rudof: &mut Rudof, mode: Option<&ShaclValidationMode>) -> Result<()> {
    validate_shacl_with(rudof, mode, None)
}

/// Like [`validate_shacl`], with `fetch_strategy`, if given, overriding the
/// configured `[shacl] fetch_strategy` for this call.
pub fn validate_shacl_with(
    rudof: &mut Rudof,
    mode: Option<&ShaclValidationMode>,
    fetch_strategy: Option<ShaclFetchStrategy>,
) -> Result<()> {
    let mut shacl_config = rudof.config().execute().shacl().clone();
    if let Some(strategy) = fetch_strategy {
        shacl_config = shacl_config.with_fetch_strategy(strategy);
    }

    let (data, shacl_schema_ir) = validate_loaded_data_schema_and_shapes(rudof)?;

    let mode = mode.copied().unwrap_or_default();

    let rdf_data = data.unwrap_rdf_mut().clone();

    // Data from SPARQL endpoints is read through a cache and never scanned as
    // a whole (see `CachedDataValidation`); data in memory is validated as is.
    #[cfg(any(not(target_family = "wasm"), feature = "jspi"))]
    let from_endpoints = !rdf_data.use_endpoints().is_empty();
    #[cfg(all(target_family = "wasm", not(feature = "jspi")))]
    let from_endpoints = false;

    let result = if from_endpoints {
        let mut validator = CachedDataValidation::new(rdf_data);
        ShaclProcessor::validate(&mut validator, shacl_schema_ir, &mode.into(), &shacl_config)
    } else {
        let graph: Graph = rdf_data.into();
        let mut validator: GraphValidation = graph.into();
        ShaclProcessor::validate(&mut validator, shacl_schema_ir, &mode.into(), &shacl_config)
    }
    .map_err(|e| ShaclError::FailedShaclValidation { error: e.to_string() })?;

    rudof.shacl_validation_results = Some(result);

    Ok(())
}

fn validate_loaded_data_schema_and_shapes(rudof: &mut Rudof) -> Result<(&mut Data, &IRSchema)> {
    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoDataLoaded))?;

    if !data.is_rdf() {
        Err(Box::new(DataError::NoRdfDataLoaded))?
    }

    let shacl_schema_ir = rudof.shacl_shapes.as_ref().ok_or(ShaclError::NoShaclShapesLoaded)?;

    Ok((data, shacl_schema_ir))
}
