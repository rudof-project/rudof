mod node_parser;
mod parser;
mod parser_error;

pub use node_parser::*;
pub use parser::*;
pub use parser_error::*;

pub type ParserResult<A> = Result<A, RdfParseError>;
