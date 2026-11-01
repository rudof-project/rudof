#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms)]

pub mod ast;
pub mod ir;
pub mod rdf;
pub mod types;
pub mod validator;

/// Runs `f`, on a new 2 MiB stack if less than 512 KiB of the current one is left.
///
/// Compiling shapes, walking their dependencies and validating them recurse once per nested
/// shape, and validation also once per node of a data chain followed by a recursive shape, so
/// deep shapes graphs and long chains would overflow the stack. What runs between two calls must
/// fit in those 512 KiB. The most is writing a SHACL path at the depth limit as RDF, which
/// `rudof_rdf` does without a check, in about 410 KiB in a debug build.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn with_stack<R>(f: impl FnOnce() -> R) -> R {
    stacker::maybe_grow(512 * 1024, 2 * 1024 * 1024, f)
}

/// On wasm, stacker can only move the shadow stack, not the engine's call stack, so it would not help.
#[cfg(target_family = "wasm")]
pub(crate) fn with_stack<R>(f: impl FnOnce() -> R) -> R {
    f()
}

/// Runs `f`, on a new 4 MiB stack if less than 2 MiB of the current one is left.
///
/// Compiling an `sh:pattern` recurses once per level of nesting, and at its nesting limit takes
/// about 1 MiB in a debug build, in `RDFRegex::new` or in the query engine for a SPARQL `ASK`.
#[cfg(not(target_family = "wasm"))]
pub(crate) fn with_regex_stack<R>(f: impl FnOnce() -> R) -> R {
    stacker::maybe_grow(2 * 1024 * 1024, 4 * 1024 * 1024, f)
}

/// On wasm, as for [`with_stack`].
#[cfg(target_family = "wasm")]
pub(crate) fn with_regex_stack<R>(f: impl FnOnce() -> R) -> R {
    f()
}

pub mod error {
    pub use crate::ast::error::*;
    pub use crate::ir::error::*;
    pub use crate::rdf::error::*;
    pub use crate::validator::error::*;
}
