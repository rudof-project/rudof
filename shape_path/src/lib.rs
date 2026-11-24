//! [ShExPath](https://shexspec.github.io/spec/ShExPath): a path language that addresses the
//! elements of a ShEx schema.
//!
//! [`ShExPathParser`] reads a path into a [`ShExPath`]. A path is
//! evaluated over the ShExJ form of a schema, a [`ShExJDocument`], and selects
//! [`ShExPathItem`]s: the nodes of that document, each with its [`Location`].
//!
//! This crate does not depend on `shex_ast`: a [`ShExJDocument`] is made from a ShExJ value, and
//! `shex_ast::shexpath` makes one from a parsed schema.

mod ast;
mod document;
mod error;
mod eval;
mod item;
mod parser;
mod schema_path;

pub use ast::*;
pub use document::*;
pub use error::*;
pub use item::*;
pub use parser::*;
pub use schema_path::*;

#[cfg(test)]
mod tests {
    mod parser_filters_tests;
    mod parser_helpers;
    mod parser_iris_tests;
    mod parser_junctions_tests;
    mod parser_shortcuts_tests;
    mod parser_steps_tests;
}
