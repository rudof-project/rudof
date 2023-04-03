use anyhow::{bail, Context, Result};
use clap::Parser;
use shex_testsuite::manifest::Manifest;
use shex_testsuite::manifest_schemas::ManifestSchemas;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of Manifest file
    #[arg(short, long, value_name = "Manifest FILE (.jsonld)")]
    manifest: PathBuf,

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

fn main() -> Result<()> {
    let cli = Cli::parse();

    // let manifest_path = Path::new("shex_testsuite/shexTest/schemas/manifest.jsonld");
    let manifest_path = cli.manifest.as_path();
    let base = get_base(manifest_path)?;
    let manifest = {
        let path_buf = manifest_path.canonicalize()?;
        if cli.debug > 0 {
            println!("path_buf: {}", &path_buf.display());
        }
        let manifest_str = fs::read_to_string(&manifest_path)
            .with_context(|| format!("Failed to read manifest: {}", manifest_path.display()))?;
        serde_json::from_str::<ManifestSchemas>(&manifest_str)?
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
