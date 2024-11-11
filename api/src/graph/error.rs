use oxrdfio::RdfParseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphError {}

#[derive(Error, Debug)]
pub enum MutableGraphError {}

#[derive(Error, Debug)]
pub enum RdfParserError {
    #[error(transparent)]
    Parsing(#[from] RdfParseError),
    #[error(transparent)]
    Inserting(#[from] MutableGraphError),
}
