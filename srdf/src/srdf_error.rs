use thiserror::Error;

#[derive(Error, Debug)]
pub enum RDFError {
    #[error("Conversion error {msg}")]
    ConversionError { msg: String },

    #[error("Converting Object {object} to RDF term")]
    ObjectAsTerm { object: String },

    #[error("Converting term {term} to IRI")]
    TermAsIri { term: String },

    #[error("Converting term {term} to concrete IRI")]
    TermAsIriS { term: String },

    #[error("Converting term {term} to Literal")]
    TermAsLiteral { term: String },

    #[error("Converting literal {literal} to SLiteral")]
    LiteralAsSLiteral { literal: String },

    #[error("Converting Term {term} to Object")]
    TermAsObject { term: String },

    #[error("Converting term {term} to subject")]
    TermAsSubject { term: String },

    #[error("Converting Term {term} to Lang")]
    TermAsLang { term: String },

    #[error("Comparison error: {term1} with {term2}")]
    ComparisonError { term1: String, term2: String },
}

impl RDFError {
    pub fn msg(str: &str) -> RDFError {
        RDFError::ConversionError {
            msg: str.to_owned(),
        }
    }
}
