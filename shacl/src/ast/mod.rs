//! SHACL AST
//! Represents [SHACL](https://www.w3.org/TR/shacl/) Abstract Syntax Tree.
//! This project started as a re-implementation in Rust of [SHACL-s](https://github.com/weso/shacl-s).

mod component;
mod node_shape;
mod property_shape;
mod schema;
mod shape;
pub(crate) mod error;
mod node_expr;

pub(crate) use component::ASTComponent;
pub(crate) use node_shape::ASTNodeShape;
pub(crate) use property_shape::ASTPropertyShape;
pub(crate) use schema::ASTSchema;
