extern crate anyhow;
extern crate clap;
extern crate dctap;
extern crate iri_s;
extern crate oxrdf;
extern crate prefixmap;
extern crate regex;
extern crate serde_json;
extern crate shacl_ast;
extern crate shapemap;
extern crate shapes_converter;
extern crate srdf;
extern crate supports_color;
extern crate tracing;
extern crate tracing_subscriber;

use anyhow::*;
use clap::Parser;
use rudof_cli::CliShaclFormat;
use rudof_cli::ShExFormat as CliShExFormat;
use rudof_cli::cli::{Cli, Command};
use rudof_cli::data::run_data;
use rudof_cli::node::run_node;
use rudof_cli::query::run_query;
use rudof_cli::rdf_config::run_rdf_config;
use rudof_cli::run_compare;
use rudof_cli::{
    ValidationMode, run_convert, run_dctap, run_service, run_shacl, run_shapemap, run_shex,
    run_validate_shacl, run_validate_shex,
};
use rudof_lib::RudofConfig;
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

    tracing::debug!("rudof running...");

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let cli = Cli::parse_from(args);

    match &cli.command {
        Some(Command::Compare {
            schema1,
            format1,
            input_mode1,
            shape1,
            schema2,
            format2,
            input_mode2,
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
                shape1.as_deref(),
                schema2,
                format2,
                input_mode2,
                shape2.as_deref(),
                reader_mode,
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
            result_schema_format,
            show_dependencies,
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
                result_schema_format,
                output,
                show_time,
                show_schema.unwrap_or_default(),
                compile.unwrap_or_default(),
                *force_overwrite,
                reader_mode,
                &config,
            )
        }
        Some(Command::Validate {
            validation_mode,
            schema,
            schema_format,
            data,
            data_format,
            reader_mode,
            endpoint,
            node,
            shape,
            shapemap,
            shapemap_format,
            max_steps,
            shacl_validation_mode,
            result_format,
            output,
            config,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            match validation_mode {
                ValidationMode::ShEx => {
                    let result_shex_format = result_format.to_shex_result_format();
                    run_validate_shex(
                        schema,
                        schema_format,
                        data,
                        data_format,
                        endpoint,
                        reader_mode,
                        node,
                        shape,
                        shapemap,
                        shapemap_format,
                        cli.debug,
                        &result_shex_format,
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
                    run_validate_shacl(
                        schema,
                        &shacl_format,
                        data,
                        data_format,
                        endpoint,
                        reader_mode,
                        *shacl_validation_mode,
                        cli.debug,
                        &result_shacl_validation,
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
            data,
            data_format,
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
        }) => {
            let config = get_config(config)?;
            run_validate_shex(
                schema,
                schema_format,
                data,
                data_format,
                endpoint,
                reader_mode,
                node,
                shape,
                shapemap,
                shapemap_format,
                cli.debug,
                result_format,
                output,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::ShaclValidate {
            shapes,
            shapes_format,
            data,
            data_format,
            reader_mode,
            endpoint,
            mode,
            result_format,
            output,
            force_overwrite,
            config,
        }) => {
            let config = get_config(config)?;

            run_validate_shacl(
                shapes,
                shapes_format,
                data,
                data_format,
                endpoint,
                reader_mode,
                *mode,
                cli.debug,
                result_format,
                output,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::Data {
            data,
            data_format,
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
                cli.debug,
                output,
                result_format,
                *force_overwrite,
                reader_mode,
                &config,
            )
        }
        Some(Command::Node {
            data,
            data_format,
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
                endpoint,
                reader_mode,
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
            reader_mode,
            shapes,
            shapes_format,
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
                endpoint,
                shapes,
                shapes_format,
                result_shapes_format,
                output,
                *force_overwrite,
                reader_mode,
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
                input_mode,
                shape,
                result_format,
                output,
                output_mode,
                target_folder,
                &config,
                *force_overwrite,
                reader_mode,
                show_time.unwrap_or(false),
            )
        }
        Some(Command::Query {
            query,
            data,
            data_format,
            endpoint,
            reader_mode,
            output,
            result_query_format,
            config,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            run_query(
                data,
                data_format,
                endpoint,
                reader_mode,
                query,
                result_query_format,
                output,
                &config,
                cli.debug,
                *force_overwrite,
            )
        }
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
        None => Ok(RudofConfig::default()),
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
        CliShExFormat::ShExJ => bail!("Validation using SHACL mode doesn't support ShExC format"),
        CliShExFormat::Turtle => Ok(CliShaclFormat::Turtle),
        CliShExFormat::NTriples => Ok(CliShaclFormat::NTriples),
        CliShExFormat::RDFXML => Ok(CliShaclFormat::RDFXML),
        CliShExFormat::TriG => Ok(CliShaclFormat::TriG),
        CliShExFormat::N3 => Ok(CliShaclFormat::N3),
        CliShExFormat::NQuads => Ok(CliShaclFormat::NQuads),
    }
}
