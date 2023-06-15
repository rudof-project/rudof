use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use shex_testsuite::manifest_mode::ManifestMode;
use shex_testsuite::manifest_run_result::ManifestRunResult;
// use shex_testsuite::manifest_run_mode;
use shex_testsuite::manifest_schemas::ManifestSchemas;
use shex_testsuite::{
    config::Config, config::ConfigError, manifest::Manifest,
    manifest_negative_structure::ManifestNegativeStructure,
    manifest_negative_syntax::ManifestNegativeSyntax, manifest_run_mode::ManifestRunMode,
    manifest_validation::ManifestValidation,
};
use std::fmt::Debug;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of Manifest file
    #[arg(
        short,
        long,
        value_name = "Manifest FILE (.jsonld)",
        default_value = "shex_testsuite/localTest/schemas/manifest.jsonld"
    )]
    manifest_filename: String,

    #[arg(
        short,
        long,
        value_name = "Config",
        default_value = "shex_testsuite/config.yml"
    )]
    config: String,

    #[arg(value_enum, long="run_mode", default_value_t = ManifestRunMode::CollectErrors)]
    manifest_run_mode: ManifestRunMode,

    /// Turn debug on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn get_base(path: &Path) -> Result<PathBuf> {
    let mut base = PathBuf::from(path);
    if base.pop() {
        Ok(base)
    } else {
        bail!("Error obtaining base from {}", path.display())
    }
}

fn parse_manifest(manifest_str: String, mode: ManifestMode) -> Result<Box<dyn Manifest>> {
    match mode {
        ManifestMode::Schemas => {
            let manifest_schemas = serde_json::from_str::<ManifestSchemas>(&manifest_str)?;
            Ok(Box::new(manifest_schemas))
        }
        ManifestMode::Validation => {
            let manifest_validation = serde_json::from_str::<ManifestValidation>(&manifest_str)?;
            Ok(Box::new(manifest_validation))
        }
        ManifestMode::NegativeStructure => {
            let manifest_schemas =
                serde_json::from_str::<ManifestNegativeStructure>(&manifest_str)?;
            Ok(Box::new(manifest_schemas))
        }
        ManifestMode::NegativeSyntax => {
            let manifest_negative_syntax =
                serde_json::from_str::<ManifestNegativeSyntax>(&manifest_str)?;
            Ok(Box::new(manifest_negative_syntax))
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let manifest_path = Path::new(&cli.manifest_filename);

    let base = get_base(manifest_path)?;
    let config = parse_config(cli.config)?;

    let manifest = {
        let path_buf = manifest_path.canonicalize()?;
        if cli.debug > 0 {
            println!("path_buf: {}", &path_buf.display());
        }
        let manifest_str = fs::read_to_string(&manifest_path)
            .with_context(|| format!("Failed to read manifest: {}", manifest_path.display()))?;

        parse_manifest(manifest_str, config.manifest_mode)?
    };

    let result = manifest.run(
        &base,
        cli.debug,
        cli.manifest_run_mode,
        config.excluded_entries,
        config.single_entries,
    );

    print_result(result, cli.debug);
    Ok(())
}

fn parse_config(file_name: String) -> Result<Config, ConfigError> {
    Config::from_file(file_name)
}

fn print_result(result: ManifestRunResult, debug: u8) -> () {
    let (npassed, nskipped, nfailed) = (
        result.passed.len(),
        result.skipped.len(),
        result.failed.len(),
    );
    let overview = format!(
        "Passed: {}, Failed: {}, Skipped: {}",
        npassed, nfailed, nskipped
    );
    match debug {
        0 => {
            println!("{}", overview);
        }
        _ => {
            for err in result.failed {
                println!("{}", err);
            }
            println!("{}", overview);
        }
    }
}
