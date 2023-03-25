use shex_testsuite::manifest_schemas::ManifestSchemas;
use std::{fs, path::Path};

fn main() {
    let manifest_path = Path::new("shex_testsuite/shexTest/schemas/manifest.jsonld");
    let manifest = {
        let manifest_str = fs::read_to_string(&manifest_path).unwrap();
        serde_json::from_str::<ManifestSchemas>(&manifest_str).unwrap()
    };
    manifest.run();
}
