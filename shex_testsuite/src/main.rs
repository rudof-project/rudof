#[cfg(not(target_family = "wasm"))]
use anyhow::{Context, Result, bail};
#[cfg(not(target_family = "wasm"))]
use clap::Parser;
#[cfg(not(target_family = "wasm"))]
use shex_testsuite::{
    config::Config,
    config::ConfigError,
    manifest::Manifest,
    manifest_mode::{ManifestMode, ManifestShExSyntaxMode},
    manifest_negative_structure::ManifestNegativeStructure,
    manifest_negative_syntax::ManifestNegativeSyntax,
    manifest_run_mode::ManifestRunMode,
    manifest_schemas::ManifestSchemas,
    manifest_validation::ManifestValidation,
    print_result_mode::PrintResultMode,
};
#[cfg(not(target_family = "wasm"))]
use std::fmt::Debug;
#[cfg(not(target_family = "wasm"))]
use std::io;
#[cfg(not(target_family = "wasm"))]
use std::{
    fs,
    path::{Path, PathBuf},
};
#[cfg(not(target_family = "wasm"))]
use tracing::trace;
#[cfg(not(target_family = "wasm"))]
use tracing_subscriber::prelude::*;
#[cfg(not(target_family = "wasm"))]
use tracing_subscriber::{filter::EnvFilter, fmt};

#[cfg(not(target_family = "wasm"))]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of Manifest file
    #[arg(
        short = 'm',
        long = "manifest",
        value_name = "Manifest FILE (.jsonld)",
        default_value = "shex_testsuite/shexTest/validation/manifest.jsonld"
    )]
    manifest_filename: String,

    #[arg(
        short = 'c',
        long = "config",
        value_name = "Config file",
        default_value = "shex_testsuite/config.toml"
    )]
    config: String,

    #[arg(value_enum, short = 'x', long="run_mode", default_value_t = ManifestRunMode::CollectErrors)]
    manifest_run_mode: ManifestRunMode,

    #[arg(value_enum, short = 'f', long="manifest_mode", default_value = None)]
    manifest_mode: Option<ManifestMode>,

    #[arg(value_enum, short = 's', long="shex_syntax", default_value = None)]
    shex_syntax: Option<ManifestShExSyntaxMode>,

    #[arg(value_enum, short='p', long="print_result_mode", default_value_t = PrintResultMode::Basic)]
    print_result_mode: PrintResultMode,

    #[arg(
        short = 'e',
        long = "entry",
        value_name = "Entry names",
        default_value = None
    )]
    entry_name: Option<Vec<String>>,

    #[arg(
        short = 't',
        long = "trait",
        value_name = "Trait names",
        default_value = None
    )]
    trait_name: Option<Vec<String>>,
}

#[cfg(not(target_family = "wasm"))]
fn get_base(path: &Path) -> Result<PathBuf> {
    let mut base = PathBuf::from(path);
    if base.pop() {
        Ok(base)
    } else {
        bail!("Error obtaining base from {}", path.display())
    }
}

#[cfg(not(target_family = "wasm"))]
fn parse_manifest(manifest_str: String, mode: ManifestMode) -> Result<Box<dyn Manifest>> {
    match mode {
        ManifestMode::Schemas => {
            let manifest_schemas = serde_json::from_str::<ManifestSchemas>(&manifest_str)?;
            Ok(Box::new(manifest_schemas))
        },
        ManifestMode::Validation => {
            let manifest_validation = serde_json::from_str::<ManifestValidation>(&manifest_str)?;
            Ok(Box::new(manifest_validation))
        },
        ManifestMode::NegativeStructure => {
            let manifest_schemas = serde_json::from_str::<ManifestNegativeStructure>(&manifest_str)?;
            Ok(Box::new(manifest_schemas))
        },
        ManifestMode::NegativeSyntax => {
            let manifest_negative_syntax = serde_json::from_str::<ManifestNegativeSyntax>(&manifest_str)?;
            Ok(Box::new(manifest_negative_syntax))
        },
    }
}

#[cfg(not(target_family = "wasm"))]
fn main() -> Result<()> {
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
    tracing_subscriber::registry().with(filter_layer).with(fmt_layer).init();
    let cli = Cli::parse();

    let manifest_path = Path::new(&cli.manifest_filename);

    let base = get_base(manifest_path)?;
    let config = parse_config(cli.config)?;

    let manifest = {
        let path_buf = manifest_path.canonicalize()?;
        trace!("path_buf: {}", &path_buf.display());
        let manifest_str = fs::read_to_string(manifest_path)
            .with_context(|| format!("Failed to read manifest: {}", manifest_path.display()))?;

        let manifest_mode = if let Some(mm) = cli.manifest_mode {
            mm
        } else {
            config.manifest_mode
        };

        parse_manifest(manifest_str, manifest_mode)?
    };

    let entries = match (cli.entry_name, config.single_entries.clone()) {
        (None, None) => None,
        (None, Some(es)) => Some(es),
        (Some(es), None) => Some(es),
        (Some(es), Some(_)) => Some(es),
    };

    let shex_syntax_mode = if let Some(ssm) = cli.shex_syntax {
        ssm
    } else {
        config.manifest_shex_syntax_mode()
    };

    let result = manifest.run(
        &base,
        cli.manifest_run_mode,
        config.excluded_entries.clone(),
        entries,
        cli.trait_name,
        shex_syntax_mode,
    );

    cli.print_result_mode.print_result(result);
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
fn parse_config(file_name: String) -> Result<Config, ConfigError> {
    Config::from_file(file_name.as_str())
}

#[cfg(target_family = "wasm")]
fn main() {}
