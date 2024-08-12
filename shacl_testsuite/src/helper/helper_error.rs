use thiserror::Error;

#[derive(Error, Debug)]
pub enum HelperError {
    #[error("No triple found por given terms")]
    NoTripleFound,
}
