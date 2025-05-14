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
mod compact_printer;
mod grammar;
mod grammar_structs;
mod located_parse_error;
pub mod shapemap_compact_printer;
mod shapemap_grammar;
pub mod shapemap_parser;
pub mod shex_compact_printer;
mod shex_grammar;
pub mod shex_parser;
pub mod shex_parser_error;

use nom::IResult;
use nom_locate::LocatedSpan;

pub(crate) use crate::compact_printer::*;
pub(crate) use crate::grammar::*;
pub use crate::located_parse_error::*;
pub use crate::shapemap_compact_printer::*;
pub use crate::shapemap_parser::*;
pub use crate::shex_compact_printer::*;
pub use crate::shex_grammar::*;
pub use crate::shex_parser::*;
pub use crate::shex_parser_error::*;

// type Result<A> = std::result::Result<A, ParseError>;

// Some definitions were inspired from [Nemo](https://github.com/knowsys/nemo/blob/main/nemo/src/io/parser/types.rs)

pub(crate) type IRes<'a, T> = IResult<Span<'a>, T, LocatedParseError>;

/// A [`LocatedSpan`] over the input.
pub(crate) type Span<'a> = LocatedSpan<&'a str>;
