//! [ShExPath](https://shexspec.github.io/spec/ShExPath): a path language that addresses the
//! elements of a ShEx schema.
//!
//! [`ShExPathParser`](crate::compact::ShExPathParser) reads a path into a [`ShExPath`].

mod path_ast;

pub use path_ast::*;
