use thiserror::Error;

/// Errors that can occur during schema comparison operations.
#[derive(Error, Debug)]
pub enum ComparisonError {
    /// The schema comparison mode is not supported.
    #[error("Unsupported comparison mode: '{mode}'. Valid modes are: 'shacl', 'shex', 'dctap', 'service'")]
    UnsupportedComparisonMode { mode: String },

    /// The input format for comparison is not supported.
    #[error("Unsupported input format for comparison: '{format}'. Valid formats are: 'shexc', 'shexj', 'turtle', 'rdfxml', 'ntriples'")]
    UnsupportedComparisonFormat { format: String },

    /// The result format for comparison is not supported.
    #[error("Unsupported result format for comparison: '{format}'. Valid formats are: 'internal', 'json'")]
    UnsupportedResultComparisonFormat { format: String },

    /// ShEx-specific formats cannot be converted to SHACL.
    #[error("Cannot convert format '{format}' to SHACL. SHACL only supports RDF formats (turtle, ntriples, rdfxml)")]
    UnsupportedConversionToShacl { format: String },

    /// RDF/ShEx formats cannot be converted to DC-TAP.
    #[error("Cannot convert format '{format}' to DC-TAP. DC-TAP uses tabular formats (CSV, Excel), not RDF or ShEx formats")]
    UnsupportedConversionToDCTap { format: String },
}