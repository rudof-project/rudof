use clap::{Arg, Command};
use data_generator::{DataGenerator, GeneratorConfig};
use std::path::PathBuf;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Parse command line arguments
    let matches = Command::new("data_generator")
        .about("Generate synthetic RDF data from ShEx or SHACL schemas")
        .version("0.1.0")
        .arg(
            Arg::new("config")
                .long("config")
                .short('c')
                .value_name("CONFIG_FILE")
                .help("Configuration file (TOML or JSON)")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("schema")
                .long("schema")
                .short('s')
                .value_name("SCHEMA_FILE")
                .help("Schema file (ShEx or SHACL - format auto-detected)")
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .value_name("OUTPUT_FILE")
                .help("Output file path")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("entities")
                .long("entities")
                .short('n')
                .value_name("COUNT")
                .help("Number of entities to generate")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("seed")
                .long("seed")
                .value_name("SEED")
                .help("Random seed for reproducible generation")
                .value_parser(clap::value_parser!(u64)),
        )
        .arg(
            Arg::new("parallel")
                .long("parallel")
                .short('p')
                .value_name("THREADS")
                .help("Number of parallel threads")
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    // Get schema file argument
    let schema_file = matches.get_one::<PathBuf>("schema").unwrap();

    // Load configuration
    let mut config = if let Some(config_file) = matches.get_one::<PathBuf>("config") {
        match load_config(config_file).await {
            Ok(config) => config,
            Err(e) => {
                error!("Failed to load configuration: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        info!("No configuration file specified, using defaults");
        GeneratorConfig::default()
    };

    // Apply command-line overrides
    config.merge_cli_overrides(
        matches.get_one::<usize>("entities").copied(),
        matches.get_one::<PathBuf>("output").cloned(),
        matches.get_one::<u64>("seed").copied(),
    );

    // Override parallel configuration if specified
    if let Some(threads) = matches.get_one::<usize>("parallel") {
        config.parallel.worker_threads = Some(*threads);
    }

    // Validate configuration
    if let Err(e) = config.validate() {
        error!("Configuration validation failed: {}", e);
        std::process::exit(1);
    }

    // Set up tokio runtime with configured thread count
    let mut runtime_builder = tokio::runtime::Builder::new_multi_thread();

    if let Some(threads) = config.parallel.worker_threads {
        runtime_builder.worker_threads(threads);
    }

    info!("Starting data generation...");
    info!("Schema file: {}", schema_file.display());
    info!("Output file: {}", config.output.path.display());
    info!("Entity count: {}", config.generation.entity_count);

    // Create and run the generator
    let start_time = std::time::Instant::now();

    match DataGenerator::new(config) {
        Ok(mut generator) => {
            // Use auto-detection to support both ShEx and SHACL schemas
            if let Err(e) = generator.run_auto(schema_file).await {
                error!("Data generation failed: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("Failed to create data generator: {}", e);
            std::process::exit(1);
        }
    }

    let duration = start_time.elapsed();
    info!("Data generation completed in {:?}", duration);
}

/// Load configuration from file
async fn load_config(config_path: &PathBuf) -> data_generator::Result<GeneratorConfig> {
    if !config_path.exists() {
        return Err(data_generator::DataGeneratorError::Config(format!(
            "Configuration file does not exist: {}",
            config_path.display()
        )));
    }

    match config_path.extension().and_then(|s| s.to_str()) {
        Some("toml") => GeneratorConfig::from_toml_file(config_path),
        Some("json") => GeneratorConfig::from_json_file(config_path),
        _ => {
            // Try to detect format by content
            let content = std::fs::read_to_string(config_path)?;
            if content.trim().starts_with('{') {
                GeneratorConfig::from_json_file(config_path)
            } else {
                GeneratorConfig::from_toml_file(config_path)
            }
        }
    }
}
