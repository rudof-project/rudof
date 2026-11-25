use crate::{
    Result, Rudof,
    errors::{DataError, ShExError},
    types::Data,
    utils::declare_data_prefixes,
};
use rudof_rdf::rdf_core::Rdf;
use rudof_rdf_sink::TriplesSink;
use shex_ast::{ir::schema_ir::SchemaIR as ShExSchemaIR, shapemap::NodeSelector, shapemap::QueryShapeMap};
use shex_validation::Validator as ShExValidator;

/// Validates with `read_sink` (every triple read), `valid_sink` (the valid
/// subgraph) and `invalid_sink` (the triples read outside the valid subgraph)
/// installed in the session's validator for this run only: they are removed
/// afterwards, whether the validation succeeded or not.
pub fn validate_shex(
    rudof: &mut Rudof,
    read_sink: Option<TriplesSink>,
    valid_sink: Option<TriplesSink>,
    invalid_sink: Option<TriplesSink>,
) -> Result<()> {
    let has_sink = read_sink.is_some() || valid_sink.is_some() || invalid_sink.is_some();
    declare_data_prefixes(
        rudof,
        read_sink.iter().chain(valid_sink.iter()).chain(invalid_sink.iter()),
    )?;
    if has_sink && let Some(validator) = rudof.shex_validator.as_mut() {
        validator.set_read_sink(read_sink);
        validator.set_valid_sink(valid_sink);
        validator.set_invalid_sink(invalid_sink);
    }
    let result = run_validation(rudof);
    if has_sink && let Some(validator) = rudof.shex_validator.as_mut() {
        validator.set_read_sink(None);
        validator.set_valid_sink(None);
        validator.set_invalid_sink(None);
    }
    result
}

fn run_validation(rudof: &mut Rudof) -> Result<()> {
    let (data, shex_schema, shapemap, shex_validator) = prepare_loaded_data_schema_and_shapemap(rudof)?;
    let rdf_data = data.unwrap_rdf_mut();

    let needs_store = shapemap.iter().any(|asc| {
        matches!(
            asc.node_selector,
            NodeSelector::Sparql { .. } | NodeSelector::TriplePattern { .. }
        )
    });

    if needs_store {
        rdf_data
            .check_store()
            .map_err(|e| ShExError::FailedInitializingQueryStore { error: e.to_string() })?;
    }

    let nodes_prefixmap = rdf_data.prefixmap().unwrap_or_default();
    let result = shex_validator
        .validate_shapemap(shapemap, rdf_data, shex_schema, &Some(nodes_prefixmap))
        .map_err(|e| ShExError::FailedShExValidation { error: e.to_string() })?;

    // Read back the map state that was mutated by MapActionExtension closures during validation.
    // The SchemaIR's registry holds an Arc<Mutex<MapState>> that is shared with every compiled
    // closure, so locking it here gives the fully-populated state.
    if let Some(schema_ir) = &rudof.shex_schema_ir
        && let Some(arc) = schema_ir.get_map_state_arc()
    {
        let state = arc.lock().unwrap().clone();
        rudof.map_state = Some(state);
    }

    rudof.shex_validation_results = Some(result);

    Ok(())
}

fn prepare_loaded_data_schema_and_shapemap(
    rudof: &mut Rudof,
) -> Result<(&mut Data, &ShExSchemaIR, &QueryShapeMap, &ShExValidator)> {
    let data = rudof.data.as_mut().ok_or(Box::new(DataError::NoDataLoaded))?;

    if !data.is_rdf() {
        return Err(Box::new(DataError::NoRdfDataLoaded).into());
    }

    let shex_schema_ir = rudof.shex_schema_ir.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;
    let shex_validator = rudof.shex_validator.as_ref().ok_or(ShExError::NoShExSchemaLoaded)?;

    let shapemap = rudof.shapemap.as_ref().ok_or(ShExError::NoShapemapLoaded)?;

    Ok((data, shex_schema_ir, shapemap, shex_validator))
}
