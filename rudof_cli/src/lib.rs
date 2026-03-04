#[cfg(target_family = "wasm")]
compile_error!("This crate is not intended to be used in a WebAssembly environment.");

pub mod cli;
pub mod commands;
pub mod output;
