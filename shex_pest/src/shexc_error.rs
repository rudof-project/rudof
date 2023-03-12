use crate::parser_error::ParserError;

pub enum ShExCError {
    ParseError{ msg: String },
    Unexpected(ParserError)
}