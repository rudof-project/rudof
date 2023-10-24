use iri_s::IriSError;
use oxiri::IriParseError;
use prefixmap::PrefixMapError;
use rio_turtle::TurtleError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SRDFGraphError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingPathError { path_name: String, error: io::Error },

    #[error(transparent)]
    ErrorReadingTurtle {
        #[from]
        err: TurtleError,
    },

    #[error("Turtle error: {turtle_error:?} str: {data:?}")]
    TurtleError {
        data: String,
        turtle_error: TurtleError,
    },

    #[error(transparent)]
    IriParseError {
        #[from]
        err: IriParseError,
    },

    #[error(transparent)]
    IriSError {
        #[from]
        err: IriSError,
    },

    #[error(transparent)]
    PrefixMapError {
        #[from]
        err: PrefixMapError,
    },
}
