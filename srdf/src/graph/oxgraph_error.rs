use oxrdfio::RdfParseError;
use prefixmap::PrefixMapError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphError {
    #[error(transparent)]
    Parsing(#[from] RdfParseError),
    #[error(transparent)]
    PrefixMap(#[from] PrefixMapError),
    #[error(transparent)]
    Mutation(#[from] MutableGraphError),
}

#[derive(Error, Debug)]
pub enum MutableGraphError {
    #[error(transparent)]
    PrefixMap(#[from] PrefixMapError),
}

#[derive(Error, Debug)]
pub enum GraphParseError {
    #[error(transparent)]
    OxRdfParse(#[from] RdfParseError),
    #[error(transparent)]
    PrefixMap(#[from] PrefixMapError),
    #[error(transparent)]
    Mutation(#[from] MutableGraphError),
    #[error(transparent)]
    Graph(#[from] GraphError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum SubjectConversionError {
    #[error("The term: {}, is not a valid subject", _0)]
    FromTerm(String),
}

#[derive(Error, Debug)]
pub enum TermConversionError {
    #[error("The subject: {}, is not a valid term", _0)]
    FromSubject(String),
}
