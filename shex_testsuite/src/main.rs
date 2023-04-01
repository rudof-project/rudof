use shex_testsuite::manifest_schemas::{IOError, ManifestSchemas};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let manifest_path = Path::new("shex_testsuite/shexTest/schemas/manifest.jsonld");
    let mut base = PathBuf::from(manifest_path);
    base.pop();
    let manifest = {
        let path_buf = manifest_path.canonicalize()?;
        println!("path_buf: {}", &path_buf.display());
        let manifest_str =
            fs::read_to_string(&manifest_path).map_err(|e| IOError::new(path_buf, e))?;
        // println!("manifest_str: {}", manifest_str);
        serde_json::from_str::<ManifestSchemas>(&manifest_str)?
    };
    println!(
        "Before running manifest {}",
        manifest.graph[0].entries.len()
    );
    manifest.run(&base)
}
