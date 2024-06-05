use iri_s::IriSError;
use oxiri::IriParseError;
use oxttl::TurtleParseError;
use prefixmap::PrefixMapError;
use std::io;
use std::io::Error as IOError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SRDFGraphError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingPathError { path_name: String, error: io::Error },

    #[error(transparent)]
    ErrorReadingTurtle {
        #[from]
        err: TurtleParseError,
    },

    #[error(transparent)]
    IOError {
        #[from]
        err: IOError,
    },

    #[error("Turtle error: {turtle_error:?} str: {data:?}")]
    TurtleError {
        data: String,
        turtle_error: TurtleParseError,
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
