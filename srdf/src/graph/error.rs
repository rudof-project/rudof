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
