//! ShEx compact syntax parser
//!
//! Example
//!
//! ```
//! # use iri_s::IriS;
//!
//! use shex_ast::{Schema, Shape, ShapeExpr, ShapeExprLabel};
//! use shex_compact::ShExParser;
//!
//! let str = r#"prefix : <http://example.org/>
//!              :S {}
//!             "#;
//!
//! let schema = ShExParser::parse(str, None).unwrap();
//! let mut expected = Schema::new();
//! expected.add_prefix("", &IriS::new_unchecked("http://example.org/"));
//! expected.add_shape(
//!   ShapeExprLabel::iri_unchecked("http://example.org/S"),
//!   ShapeExpr::empty_shape(),
//!   false
//! );
//! assert_eq!(schema,expected)
//!
//! ```
pub mod compact_printer;
mod grammar;
mod grammar_structs;
mod shapemap_grammar;
pub mod shex_parser;
pub mod shex_parser_error;

pub use crate::compact_printer::*;
pub use crate::grammar::*;
pub use crate::shapemap_grammar::*;
pub use crate::shex_parser::*;
pub use crate::shex_parser_error::*;
