use shex_ast::ir::{
    cache::CacheReaderMode, map_action_extension::MapActionExtension, schema_ir::SchemaIR,
    semantic_actions_registry::SemanticActionsRegistry, test_action_extension::TestActionExtension,
};
use shex_validation::Validator as ShExValidator;

use crate::{
    Result, Rudof,
    errors::ShExError,
    formats::{DataReaderMode, InputSpec},
};

pub fn load_shex_schema_precompiled(
    rudof: &mut Rudof,
    schema: &InputSpec,
    reader_mode: Option<&DataReaderMode>,
) -> Result<()> {
    let reader = schema
        .open_read(None, "ShEx precompiled schema")
        .map_err(|error| ShExError::DataSourceSpec {
            message: format!("Failed to open cache source '{}': {error}", schema.source_name()),
        })?;

    let mode = to_cache_reader_mode(reader_mode.copied().unwrap_or_default());

    let map_state = rudof.map_state.clone().unwrap_or_default();
    let registry = SemanticActionsRegistry::new().with(vec![
        Box::new(TestActionExtension::new()),
        Box::new(MapActionExtension::new(map_state)),
    ]);

    let schema_ir = SchemaIR::read(reader, registry, mode).map_err(|error| ShExError::FailedReadingShExCache {
        error: error.to_string(),
    })?;

    // The cache header already records whether the schema has negation cycles
    // (via `has_neg_cycle` at compile time), and `SchemaIR::read` rejects a
    // cache whose header contradicts the caller's `CacheReaderMode`. Skip the
    // Tarjan SCC pass in the validator constructor.
    let validator = ShExValidator::with_neg_cycle_check(&schema_ir, &rudof.config.validator_config(), false).map_err(
        |_| ShExError::FailedCompilingShExSchema {
            error: "Failed to create ShEx validator from precompiled schema.".to_string(),
        },
    )?;

    rudof.shex_schema = None;
    rudof.shex_schema_ir = Some(schema_ir);
    rudof.shex_validator = Some(validator);

    Ok(())
}

fn to_cache_reader_mode(m: DataReaderMode) -> CacheReaderMode {
    match m {
        DataReaderMode::Strict => CacheReaderMode::Strict,
        DataReaderMode::Lax => CacheReaderMode::Lax,
    }
}
