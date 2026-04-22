//! SHACL AST
//! Represents [SHACL](https://www.w3.org/TR/shacl/) Abstract Syntax Tree.
//! This project started as a re-implementation in Rust of [SHACL-s](https://github.com/weso/shacl-s).

mod component;
pub(crate) mod error;
mod node_expr;
mod node_shape;
mod property_shape;
mod reifier_info;
mod schema;
mod shape;

pub use component::ASTComponent;
pub use node_shape::ASTNodeShape;
pub use property_shape::ASTPropertyShape;
pub use reifier_info::ReifierInfo;
pub use schema::ASTSchema;
pub use shape::ASTShape;
