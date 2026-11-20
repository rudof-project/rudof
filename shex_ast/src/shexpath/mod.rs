//! [ShExPath](https://shexspec.github.io/spec/ShExPath): a path language that addresses the
//! elements of a ShEx schema.
//!
//! [`ShExPathParser`](crate::compact::ShExPathParser) reads a path into a [`ShExPath`]. A path is
//! evaluated over the ShExJ form of a schema, a [`ShExJDocument`], and selects
//! [`ShExPathItem`]s: the nodes of that document, each with its [`Location`].

mod document;
mod eval;
mod item;
mod path_ast;
mod shexj_view;
mod shexpath_error;

pub use document::*;
pub use item::*;
pub use path_ast::*;
pub use shexj_view::*;
pub use shexpath_error::*;
