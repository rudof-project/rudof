use thiserror::Error;

#[derive(Error, Debug)]
pub enum HelperError {
    #[error("Subjects cannot be Literal values")]
    ParseLiteralToSubject,
}
