use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum RDFError {
    #[error("Conversion error: {msg}")]
    ConversionError { msg: String },

    #[error("Error obtaining objects for subject {subject} and predicate {predicate}: {error}")]
    ErrorObjectsFor {
        subject: String,
        predicate: String,
        error: String,
    },

    #[error("Error obtaining subjects for predicate {predicate} and object {object}: {error}")]
    ErrorSubjectsFor {
        predicate: String,
        object: String,
        error: String,
    },

    #[error("Expected IRI or BlankNode, found literal: {literal}")]
    ExpectedIriOrBlankNodeFoundLiteral { literal: String },

    #[error("Expected IRI or BlankNode, found triple term ({subject}, {predicate}, {object})")]
    ExpectedIriOrBlankNodeFoundTriple {
        subject: String,
        predicate: String,
        object: String,
    },

    #[error("Error checking if RDF contains the triple <{subject}, {predicate}, {object}>: {error}")]
    FailedCheckingAssertion {
        subject: String,
        predicate: String,
        object: String,
        error: String,
    },

    #[error("Error obtaining IRI from IriRef: {iri_ref}")]
    IriRefError { iri_ref: String },

    #[error("Error with language tag '{language}' in literal '{literal}': {error}")]
    LanguageTagError {
        literal: String,
        language: String,
        error: String,
    },

    #[error("Converting literal {literal} to SLiteral")]
    LiteralAsSLiteral { literal: String },

    #[error("Converting Object {object} to RDF term")]
    ObjectAsTerm { object: String },

    #[error("Obtaining triples from RDF: {error}")]
    ObtainingTriples { error: String },

    #[error("RDF error parsing iri {iri}: {error}")]
    ParsingIri { iri: String, error: String },

    #[error("Converting term {term} to BNode")]
    TermAsBNode { term: String },

    #[error("Converting term {term} to IRI")]
    TermAsIri { term: String },

    #[error("Converting term {term} to concrete IRI")]
    TermAsIriS { term: String },

    #[error("Converting Term {term} to Lang")]
    TermAsLang { term: String },

    #[error("Converting term {term} to Literal")]
    TermAsLiteral { term: String },

    #[error("Converting Term {term} to Object: {error}")]
    TermAsObject { term: String, error: String },

    #[error("Converting term {term} to subject")]
    TermAsSubject { term: String },

    #[error("Writing query results in table: {error}")]
    WritingTableError { error: String },
}

impl RDFError {
    /// Creates a generic conversion error from a string slice
    pub fn msg(message: impl Into<String>) -> Self {
        Self::ConversionError {
            msg: message.into(),
        }
    }

    /// Creates a conversion error from a borrowed string without allocation when possible
    pub fn msg_borrowed(message: &str) -> Self {
        Self::ConversionError {
            msg: message.to_owned(),
        }
    }
}
