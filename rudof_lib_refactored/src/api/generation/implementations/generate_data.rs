use crate::{
    Result, Rudof,
    errors::GenerationError,
    formats::{DataFormat, GenerationSchemaFormat, InputSpec},
};
use rudof_generate::{DataGenerator, GeneratorConfig};
use std::path::PathBuf;

pub async fn generate_data(
    _rudof: &Rudof,
    schema: &InputSpec,
    schema_format: &GenerationSchemaFormat,
    result_generation_format: Option<&DataFormat>,
    output: Option<&PathBuf>,
    config_file: Option<&PathBuf>,
    number_entities: usize,
    seed: Option<u64>,
    parallel: Option<usize>,
) -> Result<()> {
    let (config, schema_path) = init_defaults(
        schema,
        result_generation_format,
        config_file,
        output,
        seed,
        parallel,
        number_entities,
    )?;

    let mut generator =
            DataGenerator::new(config).map_err(|e| GenerationError::FailedCreatingDataGenerator { error: e.to_string() })?;

    match schema_format {
        GenerationSchemaFormat::Auto => {
            generator
                .load_schema_auto(&schema_path)
                .await
                .map_err(|e| GenerationError::FailedLoadingSchema { error: e.to_string() })?;
        },
        GenerationSchemaFormat::ShEx => {
            generator
                    .load_shex_schema(&schema_path)
                    .await
                    .map_err(|e| GenerationError::FailedLoadingSchema { error: e.to_string() })?;
        },
        GenerationSchemaFormat::Shacl => {
            generator
                    .load_shacl_schema(&schema_path)
                    .await
                    .map_err(|e| GenerationError::FailedLoadingSchema { error: e.to_string() })?;
        },
    }

    generator
            .generate()
            .await
            .map_err(|e| GenerationError::FailedGeneratingData { error: e.to_string() })?;

    Ok(())
}

fn init_defaults(
    schema: &InputSpec,
    result_generation_format: Option<&DataFormat>,
    config_file: Option<&PathBuf>,
    output: Option<&PathBuf>,
    seed: Option<u64>,
    parallel: Option<usize>,
    entity_count: usize,
) -> Result<(GeneratorConfig, PathBuf)> {
    let mut config = if let Some(config_path) = config_file {
        if config_path.extension().and_then(|s| s.to_str()) == Some("toml") {
            GeneratorConfig::from_toml_file(config_path)
                .map_err(|e| GenerationError::WrongGeneratorConfig { error: e.to_string() })?
        } else {
            GeneratorConfig::from_json_file(config_path)
                .map_err(|e| GenerationError::WrongGeneratorConfig { error: e.to_string() })?
        }
    } else {
        GeneratorConfig::default()
    };

    config.generation.entity_count = entity_count;
    if let Some(output_path) = output {
        config.output.path = output_path.clone();
    }

    if let Some(seed_value) = seed {
        config.generation.seed = Some(seed_value);
    }

    if let Some(threads) = parallel {
        config.parallel.worker_threads = Some(threads);
    }

    let result_generation_format = result_generation_format.copied().unwrap_or_default();
    config.output.format = result_generation_format.into();

    let schema_path = match schema {
        InputSpec::Path(path) => path.clone(),
        InputSpec::Stdin => {
            return Err(GenerationError::UnsupportedGenerationSchemaInput {
                input: "stdin".to_string(),
            })?;
        },
        InputSpec::Url(_url) => {
            return Err(GenerationError::UnsupportedGenerationSchemaInput {
                input: "url".to_string(),
            })?;
        },
        InputSpec::Str(_s) => {
            return Err(GenerationError::UnsupportedGenerationSchemaInput {
                input: "string".to_string(),
            })?;
        },
    };

    Ok((config, schema_path))
}
