use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError<'a> {
    #[error("Nom Parsing error: {err}")]
    NomError { err: nom::error::Error<&'a str> },

    #[error("{msg}")]
    Custom { msg: String },
}
