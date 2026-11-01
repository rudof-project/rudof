#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms)]

pub mod ast;
pub mod ir;
pub mod rdf;
pub mod types;
pub mod validator;

/// Runs `f`, on a new 2 MiB stack if less than 256 KiB of the current one is left.
///
/// Compiling shapes, walking their dependencies and validating them recurse once per nested
/// shape, and validation also once per node of a data chain followed by a recursive shape, so
/// deep shapes graphs and long chains would overflow the stack.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn with_stack<R>(f: impl FnOnce() -> R) -> R {
    stacker::maybe_grow(256 * 1024, 2 * 1024 * 1024, f)
}

/// On wasm, stacker can only move the shadow stack, not the engine's call stack, so it would not help.
#[cfg(target_family = "wasm")]
pub(crate) fn with_stack<R>(f: impl FnOnce() -> R) -> R {
    f()
}

pub mod error {
    pub use crate::ast::error::*;
    pub use crate::ir::error::*;
    pub use crate::rdf::error::*;
    pub use crate::validator::error::*;
}
