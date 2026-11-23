//! [ShExPath](https://shexspec.github.io/spec/ShExPath): a path language that addresses the
//! elements of a ShEx schema.
//!
//! [`ShExPathParser`] reads a path into a [`ShExPath`]. A path is
//! evaluated over the ShExJ form of a schema, a [`ShExJDocument`], and selects
//! [`ShExPathItem`]s: the nodes of that document, each with its [`Location`].

mod ast;
mod document;
mod error;
mod eval;
mod item;
mod parser;
mod schema_path;
mod shexj;

pub use ast::*;
pub use document::*;
pub use error::*;
pub use item::*;
pub use parser::*;
pub use schema_path::*;
pub use shexj::*;

#[cfg(test)]
mod tests {
    mod parser_filters_tests;
    mod parser_helpers;
    mod parser_iris_tests;
    mod parser_junctions_tests;
    mod parser_shortcuts_tests;
    mod parser_steps_tests;
}
