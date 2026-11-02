//! Reading schema files from the async API without blocking its runtime.

use std::path::Path;

/// Reads a file to a string with tokio, so that the async runtime is not blocked.
#[cfg(not(target_family = "wasm"))]
pub(crate) async fn read_to_string(path: &Path) -> std::io::Result<String> {
    tokio::fs::read_to_string(path).await
}

/// WebAssembly has no file system: schemas are given as text (`load_schema_str`).
#[cfg(target_family = "wasm")]
pub(crate) async fn read_to_string(path: &Path) -> std::io::Result<String> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        format!(
            "cannot read {} in WebAssembly: load the schema as text with load_schema_str",
            path.display()
        ),
    ))
}
