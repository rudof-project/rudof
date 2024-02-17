use anyhow::{bail, Context, Result};
use clap::Parser;
use tracing::debug;
use shex_testsuite::manifest_mode::ManifestMode;
use shex_testsuite::manifest_run_result::ManifestRunResult;
use shex_testsuite::manifest_schemas::ManifestSchemas;
use shex_testsuite::print_result_mode::PrintResultMode;
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
        default_value = "shex_testsuite/config.yml"
    )]
    config: String,

    #[arg(value_enum, short = 'x', long="run_mode", default_value_t = ManifestRunMode::CollectErrors)]
    manifest_run_mode: ManifestRunMode,

    #[arg(value_enum, short = 'f', long="manifest_mode", default_value = None)]
    manifest_mode: Option<ManifestMode>,

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
        debug!("path_buf: {}", &path_buf.display());
        let manifest_str = fs::read_to_string(&manifest_path)
            .with_context(|| format!("Failed to read manifest: {}", manifest_path.display()))?;

        let manifest_mode = if let Some(mm) = cli.manifest_mode {
          mm 
        } else {
          config.manifest_mode  
        };

        parse_manifest(manifest_str, manifest_mode)?
    };

    let entries = match (cli.entry_name, config.single_entries) {
        (None, None) => None,
        (None, Some(es)) => Some(es),
        (Some(es), None) => Some(es),
        (Some(es), Some(_)) => Some(es),
    };

    let result = manifest.run(
        &base,
        cli.manifest_run_mode,
        config.excluded_entries,
        entries,
        cli.trait_name,
    );

    print_result(result, cli.print_result_mode);
    Ok(())
}

fn parse_config(file_name: String) -> Result<Config, ConfigError> {
    Config::from_file(file_name)
}

fn print_basic(result: &ManifestRunResult) {
    let (npassed, nskipped, nfailed, npanicked) = (
        result.passed.len(),
        result.skipped.len(),
        result.failed.len(),
        result.panicked.len(),
    );
    let overview = format!(
        "Passed: {}, Failed: {}, Skipped: {}, Not implemented: {}",
        npassed, nfailed, nskipped, npanicked
    );
    println!("{}", overview);
}

fn print_failed(result: &ManifestRunResult) {
    println!("--- Failed ---");
    for (name, err) in &result.failed {
        println!("{name} {err}");
    }
}

fn print_failed_simple(result: &ManifestRunResult) {
    println!("--- Failed ---");
    for (name, _) in &result.failed {
        println!("{name}");
    }
}


fn print_panicked(result: &ManifestRunResult) {
    println!("--- Not implemented ---");
    for (name, _err) in &result.panicked {
        println!("{name}");
    }
}

fn print_passed(result: &ManifestRunResult) {
    println!("--- Passed ---");
    for name in &result.passed {
        println!("{name}");
    }
}

fn print_result(result: ManifestRunResult, print_result_mode: PrintResultMode) {
    match print_result_mode {
        PrintResultMode::Basic => {
            print_basic(&result);
        }
        PrintResultMode::All => {
            print_passed(&result);
            print_failed(&result);
            print_panicked(&result);
            print_basic(&result);
        }
        PrintResultMode::Failed => {
            print_failed(&result);
            print_basic(&result);
        }
        PrintResultMode::FailedSimple => {
            print_failed_simple(&result);
            print_basic(&result);
        }
        PrintResultMode::Passed => {
            print_passed(&result);
            print_basic(&result);
        }
        PrintResultMode::NotImplemented => {
            print_panicked(&result);
            print_basic(&result);
        }
    }
}
