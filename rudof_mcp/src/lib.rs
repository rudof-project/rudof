#[cfg(not(target_family = "wasm"))]
pub mod server;

#[cfg(not(target_family = "wasm"))]
pub mod service;
