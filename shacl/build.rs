//! Defines the `sparql_validation` cfg: the SPARQL-backed parts of the
//! validator (`RdfData`, `SparqlEngine`, endpoints) are only available when
//! the `sparql` feature is enabled *and* the target is not `wasm`. On `wasm`
//! the native engine over an in-memory graph is always available, whatever
//! features Cargo's feature unification turns on.
fn main() {
    println!("cargo::rustc-check-cfg=cfg(sparql_validation)");
    let sparql = std::env::var_os("CARGO_FEATURE_SPARQL").is_some();
    let wasm = std::env::var("CARGO_CFG_TARGET_FAMILY").is_ok_and(|families| families.split(',').any(|f| f == "wasm"));
    if sparql && !wasm {
        println!("cargo::rustc-cfg=sparql_validation");
    }
}
