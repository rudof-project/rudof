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
mod shexj;

pub use ast::*;
pub use document::*;
pub use error::*;
pub use item::*;
pub use parser::*;
pub use shexj::*;
