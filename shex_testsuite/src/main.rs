use shex_testsuite::manifest_schemas::ManifestSchemas;
use std::{fs, path::Path};

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let manifest_path = Path::new("shex_testsuite/shexTest/schemas/manifest.jsonld");
    let manifest = {
        let manifest_str = fs::read_to_string(&manifest_path)?;
        serde_json::from_str::<ManifestSchemas>(&manifest_str)?
    };
    manifest.run()
}
