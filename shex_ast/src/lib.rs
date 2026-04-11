//! ShEx Abstract Syntax
//!
//! Ths abstract syntax follows the [ShEx abstract syntax spec](https://shex.io/)
//!

// #![deny(rust_2018_idioms)]
pub mod ast;
pub mod compact;
pub mod ir;
pub mod node;
pub mod pred;
pub mod resolve_method;
pub mod shapemap;
pub mod shex_format;
pub mod shexr;

use crate::ir::semantic_action_context::SemanticActionContext;
pub use ast::*;
pub use compact::*;
pub use ir::schema_ir_error::*;
pub use ir::shape_label_idx::*;
pub use node::*;
pub use pred::*;
use rbe::{MatchCond, NoState};
pub use resolve_method::*;
pub use shex_format::*;
pub use shexr::*;

/// The result type used in this crate, which is a `Result` that can contain any error that implements the `SchemaIRError` trait.
type CResult<T> = Result<T, Box<SchemaIRError>>;

/// The type of conditions used in this crate, which is a `MatchCond` that takes `Pred`, `Node`, `ShapeLabelIdx`, `SemanticActionContext`, and `NoState` as type parameters.
type Cond = MatchCond<Pred, Node, ShapeLabelIdx, SemanticActionContext, NoState>;

/// The type of expressions used in this crate, which is a `RbeTable` that takes `Pred`, `Node`, `ShapeLabelIdx`, `SemanticActionContext`, and `NoState` as type parameters.
pub type Expr = rbe::RbeTable<Pred, Node, ShapeLabelIdx, SemanticActionContext, NoState>;
