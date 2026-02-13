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

pub(crate) use crate::compact::compact_printer::*;
pub(crate) use crate::compact::grammar::*;
pub use crate::compact::located_parse_error::*;
pub use crate::compact::shex_grammar::*;
pub use crate::shapemap_compact_printer::*;
pub use crate::shapemap_parser::*;
pub use crate::shex_compact_printer::*;
pub use crate::shex_parser::*;
pub use crate::shex_parser_error::*;

// type Result<A> = std::result::Result<A, ParseError>;

// Some definitions were inspired from [Nemo](https://github.com/knowsys/nemo/blob/main/nemo/src/io/parser/types.rs)

pub(crate) type IRes<'a, T> = IResult<Span<'a>, T, LocatedParseError>;

/// A [`LocatedSpan`] over the input.
pub(crate) type Span<'a> = LocatedSpan<&'a str>;
