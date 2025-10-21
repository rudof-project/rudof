extern crate anyhow;
extern crate clap;
extern crate dctap;
extern crate iri_s;
extern crate oxrdf;
extern crate prefixmap;
extern crate regex;
extern crate rudof_generate;
extern crate serde_json;
extern crate shacl_ast;
extern crate shapes_converter;
extern crate srdf;
extern crate supports_color;
extern crate tracing;
extern crate tracing_subscriber;

use anyhow::*;
use clap::Parser;
use rudof_cli::CliShaclFormat;
use rudof_cli::ShExFormat as CliShExFormat;
use rudof_cli::SortByResultShapeMap;
use rudof_cli::SortByShaclValidationReport;
use rudof_cli::cli::{Cli, Command};
use rudof_cli::data::run_data;
use rudof_cli::data_format::DataFormat;
use rudof_cli::node::run_node;
use rudof_cli::query::run_query;
use rudof_cli::rdf_config::run_rdf_config;
use rudof_cli::run_compare;
use rudof_cli::{
    GenerateSchemaFormat, ValidationMode, run_convert, run_dctap, run_service, run_shacl,
    run_shapemap, run_shex, run_validate_shacl, run_validate_shex,
};
use rudof_lib::{InputSpec, RudofConfig};
use std::io;
use std::path::PathBuf;
use std::result::Result::Ok;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::EnvFilter, fmt};

#[allow(unused_variables)]
fn main() -> Result<()> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_target(false)
        .with_line_number(true)
        .with_writer(io::stderr)
        .without_time();
    // Attempts to get the value of RUST_LOG which can be info, debug, trace, If unset, it uses "info"
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    tracing::trace!("rudof running...");

    let args = clientele::args_os()?;
    let cli = Cli::parse_from(args);

    match &cli.command {
        Some(Command::Compare {
            schema1,
            format1,
            input_mode1,
            base1,
            shape1,
            schema2,
            format2,
            input_mode2,
            base2,
            shape2,
            result_format,
            output,
            target_folder,
            force_overwrite,
            config,
            reader_mode,
            show_time,
        }) => {
            let config = get_config(config)?;
            run_compare(
                schema1,
                format1,
                input_mode1,
                base1,
                shape1.as_deref(),
                schema2,
                format2,
                input_mode2,
                base2,
                shape2.as_deref(),
                &reader_mode.into(),
                output,
                result_format,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::RdfConfig {
            input,
            format,
            output,
            result_format,
            config,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            run_rdf_config(
                input,
                format,
                output,
                result_format,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::Mcp {
            host,
            port,
            route_name,
        }) => {
            // Run the MCP server
            rudof_mcp::run_mcp(route_name, port, host)
        }
        Some(Command::Service {
            service,
            service_format,
            output,
            result_service_format,
            config,
            reader_mode,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            run_service(
                service,
                service_format,
                reader_mode,
                output,
                result_service_format,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::Shex {
            schema,
            schema_format,
            shape,
            result_schema_format,
            show_dependencies,
            base,
            output,
            show_time,
            show_schema,
            show_statistics,
            compile,
            force_overwrite,
            reader_mode,
            config,
        }) => {
            let config = get_config(config)?;
            if let Some(show_dependencies) = show_dependencies {
                config
                    .shex_config()
                    .with_show_dependencies(*show_dependencies);
            }
            if let Some(flag) = show_statistics {
                config.shex_config().set_show_extends(*flag);
            }
            let show_time = (*show_time).unwrap_or_default();
            run_shex(
                schema,
                schema_format,
                shape,
                base,
                result_schema_format,
                output,
                show_time,
                show_schema.unwrap_or_default(),
                compile.unwrap_or_default(),
                *force_overwrite,
                &reader_mode.into(),
                &config,
            )
        }
        Some(Command::Validate {
            validation_mode,
            schema,
            schema_format,
            base_schema,
            data,
            data_format,
            base_data,
            reader_mode,
            endpoint,
            node,
            shape,
            shapemap,
            shapemap_format,
            max_steps,
            shacl_validation_mode,
            result_format,
            sort_by,
            output,
            config,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            match validation_mode {
                ValidationMode::ShEx => {
                    let result_shex_format = result_format.to_shex_result_format();
                    let sort_by = cnv_sort_by_validate_result_map(sort_by);
                    run_validate_shex(
                        schema,
                        schema_format,
                        base_schema,
                        data,
                        data_format,
                        base_data,
                        endpoint,
                        &reader_mode.into(),
                        node,
                        shape,
                        shapemap,
                        shapemap_format,
                        cli.debug,
                        &result_shex_format,
                        &sort_by,
                        output,
                        &config,
                        *force_overwrite,
                    )
                }
                ValidationMode::SHACL => {
                    let shacl_format = match &schema_format {
                        None => Ok::<Option<CliShaclFormat>, anyhow::Error>(None),
                        Some(f) => {
                            let f = schema_format_to_shacl_format(f)?;
                            Ok(Some(f))
                        }
                    }?;
                    let result_shacl_validation = result_format.to_shacl_result_format();
                    let sort_by = cnv_sort_by_validate_report(sort_by);
                    run_validate_shacl(
                        schema,
                        &shacl_format,
                        base_schema,
                        data,
                        data_format,
                        base_data,
                        endpoint,
                        &reader_mode.into(),
                        *shacl_validation_mode,
                        cli.debug,
                        &result_shacl_validation,
                        &sort_by,
                        output,
                        &config,
                        *force_overwrite,
                    )
                }
            }
        }
        Some(Command::ShexValidate {
            schema,
            schema_format,
            base_schema,
            data,
            data_format,
            base_data,
            reader_mode,
            endpoint,
            node,
            shape,
            shapemap,
            shapemap_format,
            result_format,
            output,
            config,
            force_overwrite,
            sort_by,
        }) => {
            let config = get_config(config)?;
            run_validate_shex(
                schema,
                schema_format,
                base_schema,
                data,
                data_format,
                base_data,
                endpoint,
                &reader_mode.into(),
                node,
                shape,
                shapemap,
                shapemap_format,
                cli.debug,
                result_format,
                sort_by,
                output,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::ShaclValidate {
            shapes,
            shapes_format,
            base_shapes,
            data,
            data_format,
            base_data,
            reader_mode,
            endpoint,
            mode,
            result_format,
            sort_by,
            output,
            force_overwrite,
            config,
        }) => {
            let config = get_config(config)?;

            run_validate_shacl(
                shapes,
                shapes_format,
                base_shapes,
                data,
                data_format,
                base_data,
                endpoint,
                &reader_mode.into(),
                *mode,
                cli.debug,
                result_format,
                sort_by,
                output,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::Data {
            data,
            data_format,
            base,
            reader_mode,
            output,
            result_format,
            force_overwrite,
            config,
        }) => {
            let config = get_config(config)?;
            run_data(
                data,
                data_format,
                base,
                cli.debug,
                output,
                result_format,
                *force_overwrite,
                &reader_mode.into(),
                &config,
            )
        }
        Some(Command::Node {
            data,
            data_format,
            base,
            endpoint,
            reader_mode,
            node,
            predicates,
            show_node_mode,
            show_hyperlinks,
            output,
            config,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            run_node(
                data,
                data_format,
                base,
                endpoint,
                &reader_mode.into(),
                node,
                predicates,
                show_node_mode,
                show_hyperlinks,
                cli.debug,
                output,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::Shapemap {
            shapemap,
            shapemap_format,
            result_shapemap_format,
            output,
            force_overwrite,
        }) => run_shapemap(
            shapemap,
            shapemap_format,
            result_shapemap_format,
            output,
            *force_overwrite,
        ),
        Some(Command::Shacl {
            data,
            data_format,
            base_data,
            reader_mode,
            shapes,
            shapes_format,
            base_shapes,
            endpoint,
            result_shapes_format,
            output,
            force_overwrite,
            config,
        }) => {
            let config = get_config(config)?;
            run_shacl(
                data,
                data_format,
                base_data,
                endpoint,
                shapes,
                shapes_format,
                base_shapes,
                result_shapes_format,
                output,
                *force_overwrite,
                &reader_mode.into(),
                &config,
            )
        }
        Some(Command::DCTap {
            file,
            format,
            result_format,
            config,
            output,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            run_dctap(
                file,
                format,
                result_format,
                output,
                &config,
                *force_overwrite,
            )?;
            Ok(())
        }
        Some(Command::Convert {
            file,
            format,
            base,
            input_mode,
            shape,
            result_format,
            output,
            output_mode,
            target_folder,
            force_overwrite,
            config,
            show_time,
            reader_mode,
        }) => {
            let config = get_config(config)?;
            run_convert(
                file,
                format,
                base,
                input_mode,
                shape,
                result_format,
                output,
                output_mode,
                target_folder,
                &config,
                *force_overwrite,
                &reader_mode.into(),
                show_time.unwrap_or(false),
            )
        }
        Some(Command::Query {
            query,
            data,
            data_format,
            base,
            endpoint,
            reader_mode,
            output,
            result_query_format,
            query_type,
            config,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            run_query(
                data,
                data_format,
                base,
                endpoint,
                &reader_mode.into(),
                query,
                query_type,
                result_query_format,
                output,
                &config,
                cli.debug,
                *force_overwrite,
            )
        }
        Some(Command::Generate {
            schema,
            schema_format,
            entity_count,
            output,
            result_format,
            seed,
            parallel,
            config,
            force_overwrite,
        }) => run_generate(
            schema,
            schema_format,
            *entity_count,
            output,
            result_format,
            *seed,
            *parallel,
            config,
            *force_overwrite,
        ),
        None => {
            bail!("Command not specified, type `--help` to see list of commands")
        }
    }
}

fn get_config(config: &Option<PathBuf>) -> Result<RudofConfig> {
    match config {
        Some(config_path) => match RudofConfig::from_path(config_path) {
            Ok(c) => Ok(c),
            Err(e) => Err(anyhow!(
                "Error obtaining Rudof config from {}\nError: {e}",
                config_path.display()
            )),
        },
        None => {
            let config = RudofConfig::default_config()?;
            Ok(config)
        }
    }
}

fn schema_format_to_shacl_format(f: &CliShExFormat) -> Result<CliShaclFormat> {
    match f {
        CliShExFormat::Internal => Ok(CliShaclFormat::Internal),
        CliShExFormat::ShExC => Err(anyhow!(
            "Validation using SHACL mode doesn't support ShExC format"
        )),
        CliShExFormat::Simple => Err(anyhow!(
            "Validation using SHACL mode doesn't support {f} format"
        )),
        CliShExFormat::Turtle => Ok(CliShaclFormat::Turtle),
        CliShExFormat::NTriples => Ok(CliShaclFormat::NTriples),
        CliShExFormat::RDFXML => Ok(CliShaclFormat::RDFXML),
        CliShExFormat::TriG => Ok(CliShaclFormat::TriG),
        CliShExFormat::N3 => Ok(CliShaclFormat::N3),
        CliShExFormat::NQuads => Ok(CliShaclFormat::NQuads),
        CliShExFormat::ShExJ => bail!("Validation using SHACL mode doesn't support ShExC format"),
        CliShExFormat::JSON => Ok(CliShaclFormat::JsonLd),
        CliShExFormat::JSONLD => Ok(CliShaclFormat::JsonLd),
    }
}

#[allow(clippy::too_many_arguments)]
fn run_generate(
    schema: &InputSpec,
    schema_format: &GenerateSchemaFormat,
    entity_count: usize,
    output: &Option<PathBuf>,
    result_format: &DataFormat,
    seed: Option<u64>,
    parallel: Option<usize>,
    config_file: &Option<PathBuf>,
    _force_overwrite: bool,
) -> Result<()> {
    use rudof_generate::{DataGenerator, GeneratorConfig};

    // Create tokio runtime
    let runtime = tokio::runtime::Runtime::new()?;

    runtime.block_on(async {
        // Load or create configuration
        let mut config = if let Some(config_path) = config_file {
            if config_path.extension().and_then(|s| s.to_str()) == Some("toml") {
                GeneratorConfig::from_toml_file(config_path)?
            } else {
                GeneratorConfig::from_json_file(config_path)?
            }
        } else {
            GeneratorConfig::default()
        };

        // Apply CLI overrides
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

        // Determine output format (only Turtle and NTriples are supported)
        use rudof_generate::config::OutputFormat;
        config.output.format = match result_format {
            DataFormat::Turtle | DataFormat::TriG | DataFormat::N3 => OutputFormat::Turtle,
            _ => OutputFormat::NTriples,
        };

        // Get schema path
        let schema_path = match schema {
            InputSpec::Path(path) => path.clone(),
            InputSpec::Stdin => {
                bail!("Schema from stdin is not supported for data generation")
            }
            InputSpec::Url(url) => {
                bail!("Schema from URL is not supported yet: {}", url)
            }
            InputSpec::Str(s) => {
                bail!(
                    "Schema from string is not supported for data generation: {}",
                    s
                )
            }
        };

        // Create generator
        let mut generator = DataGenerator::new(config)?;

        // Load schema based on format
        match schema_format {
            GenerateSchemaFormat::Auto => {
                generator.load_schema_auto(&schema_path).await?;
            }
            GenerateSchemaFormat::ShEx => {
                generator.load_shex_schema(&schema_path).await?;
            }
            GenerateSchemaFormat::SHACL => {
                generator.load_shacl_schema(&schema_path).await?;
            }
        }

        // Generate data
        generator.generate().await?;

        Ok::<(), anyhow::Error>(())
    })?;

    Ok(())
}

fn cnv_sort_by_validate_result_map(
    s: &rudof_cli::sort_by_validate::SortByValidate,
) -> SortByResultShapeMap {
    match s {
        rudof_cli::sort_by_validate::SortByValidate::Node => SortByResultShapeMap::Node,
        rudof_cli::sort_by_validate::SortByValidate::Details => SortByResultShapeMap::Details,
    }
}

fn cnv_sort_by_validate_report(
    s: &rudof_cli::sort_by_validate::SortByValidate,
) -> SortByShaclValidationReport {
    match s {
        rudof_cli::sort_by_validate::SortByValidate::Node => SortByShaclValidationReport::Node,
        rudof_cli::sort_by_validate::SortByValidate::Details => {
            SortByShaclValidationReport::Details
        }
    }
}
