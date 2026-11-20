//! Runner for the SHACL W3C test manifests (`sht:Validate`, `sht:EvalNodeExpr`).

#[cfg(not(target_family = "wasm"))]
mod error;
#[cfg(not(target_family = "wasm"))]
pub mod manifest;
#[cfg(not(target_family = "wasm"))]
pub mod runner;

#[cfg(not(target_family = "wasm"))]
pub use error::TestSuiteError;
