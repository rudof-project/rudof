use iri_s::{IriS, IriSError};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SchemaWithoutImportsError {
    #[error("Obtaining schema from IRI {iri}. Tried to parse this list of formats: {formats} but they failed")]
    SchemaFromIriRotatingFormats { iri: IriS, formats: String },

    #[error("Dereferencing IRI {iri}. Error: {error}")]
    DereferencingIri { iri: IriS, error: String },

    #[error("ShExC error {error}. String: {content}")]
    ShExCError { error: String, content: String },

    #[error("ShExJ error at IRI: {iri}. Error: {error}")]
    ShExJError { iri: IriS, error: String },
}
