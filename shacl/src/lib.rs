#![doc = include_str!("../README.md")]
#![deny(rust_2018_idioms)]

pub mod ast;
pub mod ir;
pub mod rdf;
pub mod types;
pub mod validation;

pub mod error {
    pub use crate::ast::error::*;
    pub use crate::ir::error::*;
    pub use crate::rdf::error::*;
}
