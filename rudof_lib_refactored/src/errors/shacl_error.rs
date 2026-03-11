use thiserror::Error;

/// Errors that can occur when working with SHACL schemas.
#[derive(Error, Debug)]
pub enum ShaclError {
    /// The SHACL schema format specified is not supported by Rudof.
    #[error("Unsupported SHACL schema format: '{format}'. Valid formats are: 'internal', 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads', 'jsonld'")]
    UnsupportedShaclSchemaFormat { format: String },
}