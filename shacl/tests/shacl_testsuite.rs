#[cfg(not(target_family = "wasm"))]
#[macro_use]
mod common;
#[cfg(not(target_family = "wasm"))]
mod core;
#[cfg(not(target_family = "wasm"))]
mod shacl12;
#[cfg(not(target_family = "wasm"))]
mod sparql;
