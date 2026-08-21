use thiserror::Error;

/// Errors that can occur when managing the default list of prefix declarations.
#[derive(Error, Debug)]
pub enum PrefixesError {
    /// The referenced alias is not present in the default prefixes.
    #[error("Unknown prefix alias '{alias}'")]
    AliasNotFound { alias: String },
}
