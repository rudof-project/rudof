use std::io;
use rio_turtle::TurtleError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SRDFError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingPathError { path_name: String, error: io::Error },

    #[error("{entry_name}: error reading turtle at {path_name:?}. Error: {turtle_err:?}")]
    ErrorReadingTurtle {
        entry_name: String,
        path_name: String,
        turtle_err: String,
    },

    #[error("Turtle error: {turtle_error:?} str: {data:?}")]
    TurtleError {
        data: String,
        turtle_error: TurtleError,
    },
}
