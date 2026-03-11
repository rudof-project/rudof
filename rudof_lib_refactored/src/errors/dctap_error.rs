use thiserror::Error;

/// Errors that can occur when working with DC-TAP (Dublin Core Tabular Application Profiles).
#[derive(Error, Debug)]
pub enum DCTapError {
    /// The DC-TAP format specified is not supported by Rudof.
    #[error("Unsupported DC-TAP format: '{format}'. Valid formats are: 'csv', 'xlsx', 'xlsb', 'xlsm', 'xls'")]
    UnsupportedDCTapFormat { format: String },

    /// The DC-TAP result format specified is not supported by Rudof.
    #[error("Unsupported DC-TAP result format: '{format}'. Valid formats are: 'internal', 'json'")]
    UnsupportedResultDCTapFormat { format: String },
}