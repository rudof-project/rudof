// #[cfg(target_family = "wasm")]
// compile_error!("This crate is not intended to be used in a WebAssembly environment.");

#[cfg(not(target_family = "wasm"))]
pub mod server;

#[cfg(not(target_family = "wasm"))]
pub mod service;
