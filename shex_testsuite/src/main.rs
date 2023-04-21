use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use shex_testsuite::manifest_schemas::ManifestSchemas;
use shex_testsuite::{
    manifest::Manifest, manifest_negative_structure::ManifestNegativeStructure,
    manifest_negative_syntax::ManifestNegativeSyntax, manifest_validation::ManifestValidation,
};
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

    #[arg(value_enum, default_value_t = Mode::Schemas)]
    mode: Mode,

    /// Turn debug on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Mode {
    Schemas,
    Validation,
    NegativeSyntax,
    NegativeStructure,
}

fn get_base(path: &Path) -> Result<PathBuf> {
    let mut base = PathBuf::from(path);
    if base.pop() {
        Ok(base)
    } else {
        bail!("Error obtaining base from {}", path.display())
    }
}

fn parse_manifest(manifest_str: String, mode: Mode) -> Result<Box<dyn Manifest>> {
    match mode {
        Mode::Schemas => {
            let manifest_schemas = serde_json::from_str::<ManifestSchemas>(&manifest_str)?;
            Ok(Box::new(manifest_schemas))
        }
        Mode::Validation => {
            let manifest_validation = serde_json::from_str::<ManifestValidation>(&manifest_str)?;
            Ok(Box::new(manifest_validation))
        }
        Mode::NegativeStructure => {
            let manifest_schemas =
                serde_json::from_str::<ManifestNegativeStructure>(&manifest_str)?;
            Ok(Box::new(manifest_schemas))
        }
        Mode::NegativeSyntax => {
            let manifest_negative_syntax =
                serde_json::from_str::<ManifestNegativeSyntax>(&manifest_str)?;
            Ok(Box::new(manifest_negative_syntax))
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // let manifest_path = Path::new("shex_testsuite/shexTest/schemas/manifest.jsonld");
    let manifest_path = Path::new(&cli.manifest_filename);
    let base = get_base(manifest_path)?;
    let manifest = {
        let path_buf = manifest_path.canonicalize()?;
        if cli.debug > 0 {
            println!("path_buf: {}", &path_buf.display());
        }
        let manifest_str = fs::read_to_string(&manifest_path)
            .with_context(|| format!("Failed to read manifest: {}", manifest_path.display()))?;
        parse_manifest(manifest_str, cli.mode)?
    };
    let count = manifest.len();
    match manifest.run(&base, cli.debug) {
        Ok(()) => {
            println!("End of processing {count} entries");
            Ok(())
        }
        Err(e) => {
            bail!("Error: {}", e)
        }
    }
}
