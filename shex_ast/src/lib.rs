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

pub use ast::*;
pub use compact::*;
pub use ir::schema_ir_error::*;
pub use ir::shape_label_idx::*;
pub use node::*;
pub use pred::*;
use rbe::MatchCond;
pub use resolve_method::*;
pub use shex_format::*;
pub use shexr::*;

type CResult<T> = Result<T, Box<SchemaIRError>>;
type Cond = MatchCond<Pred, Node, ShapeLabelIdx>;

pub type Expr = rbe::RbeTable<Pred, Node, ShapeLabelIdx>;
