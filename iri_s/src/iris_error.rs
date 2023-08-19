use oxrdf::IriParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IriSError {
    #[error(transparent)]
    IriParseError(#[from] IriParseError),
}
