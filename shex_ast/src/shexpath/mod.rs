//! [ShExPath](https://shexspec.github.io/spec/ShExPath): a path language that addresses the
//! elements of a ShEx schema.
//!
//! [`ShExPathParser`](crate::compact::ShExPathParser) reads a path into a [`ShExPath`]. A path is
//! evaluated over the ShExJ form of a schema, which [`shexj`] gives.

mod path_ast;
mod shexj_view;
mod shexpath_error;

pub use path_ast::*;
pub use shexj_view::*;
pub use shexpath_error::*;
