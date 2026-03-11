use thiserror::Error;

/// Errors that can occur when working with ShEx schemas.
#[derive(Error, Debug)]
pub enum ShExError {
    /// The ShEx format specified is not supported by Rudof.
    #[error("Unsupported ShEx format: '{format}'. Valid formats are: 'internal', 'simple', 'shexc', 'shexj', 'json', 'jsonld', 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads'")]
    UnsupportedShExFormat { format: String },
}