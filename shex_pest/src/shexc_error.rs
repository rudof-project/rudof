use iri_s::IriS;

use crate::parser_error::ParserError;

#[derive(Debug)]
pub enum ShExCError {
    ParseError{ msg: String },
    IRIError{ msg: String },
    AbsoluteIRIExpectedError{ iri: IriS },
    Unexpected(ParserError)
}

