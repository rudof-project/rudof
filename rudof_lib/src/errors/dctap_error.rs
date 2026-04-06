use thiserror::Error;

/// Errors that can occur when working with DCTap (Dublin Core Tabular Application Profiles).
#[derive(Error, Debug)]
pub enum DCTapError {
    /// The DCTap format specified is not supported by Rudof.
    #[error("Unsupported DCTap format: '{format}'. Valid formats are: 'csv', 'xlsx', 'xlsb', 'xlsm', 'xls'")]
    UnsupportedDCTapFormat { format: String },

    /// The DCTap result format specified is not supported by Rudof.
    #[error("Unsupported DCTap result format: '{format}'. Valid formats are: 'internal', 'json'")]
    UnsupportedResultDCTapFormat { format: String },

    /// Errors related to specifying the data source.
    #[error("Data source specification error: {message}")]
    DataSourceSpec { message: String },

    /// Errors that occur during DCTap data parsing.
    #[error("No DCTap data loaded. Please load DCTap data before attempting to serialize.")]
    NoDCTapLoaded,

    /// Errors that occur during DCTap serialization.
    #[error("Failed to serialize DCTap in format '{format}': {error}")]
    FailedSerializingDCTap { format: String, error: String },
}
