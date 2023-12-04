//! ShEx compact syntax parser
//!
//! Example
//!
//! ```
//! use shex_ast::{Shape, ShapeExpr, ShapeExprLabel};
//!
//! let str = r#"prefix : <http://example.org/>
//!              <S> { <p> . }
//!             "#;
//!
//! let schema = ShExParser::parse(str, None).unwrap();
//! let mut expected = Schema::new();
//! expected.add_prefix("", &IriS::new_unchecked("http://example.org/"));
//! expected.add_shape(
//!   ShapeExprLabel:iri_unchecked("http://example.org/S"),
//!   ShapeExpr::Shape(Shape::new(None,None,None)),
//!   false
//! );
//! assert_eq!(schema,expected)
//!
//! ```
mod compact_printer;
mod grammar;
mod grammar_structs;
pub mod parser;
pub mod parser_error;
mod parser_state;

pub use crate::compact_printer::*;
pub use crate::grammar::*;
pub use crate::grammar_structs::*;
pub use crate::parser::*;
pub use crate::parser_error::*;
pub use crate::parser_state::*;
