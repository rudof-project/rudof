use thiserror::Error;

/// Errors that can occur during conversion operations.
#[derive(Error, Debug)]
pub enum ConversionError {
    /// The conversion input mode is not supported by Rudof.
    #[error("Unsupported conversion input mode: '{mode}'. Valid modes are: 'shacl', 'shex', 'dctap'")]
    UnsupportedConversionMode { mode: String },

    /// The conversion result mode is not supported by Rudof.
    #[error("Unsupported conversion result mode: '{mode}'. Valid modes are: 'sparql', 'shex', 'uml', 'html', 'shacl'")]
    UnsupportedResultConversionMode { mode: String },

    /// The input format for conversion is not supported by Rudof.
    #[error("Unsupported conversion input format: '{format}'. Valid formats are: 'csv', 'xlsx', 'shexc', 'shexj', 'turtle'")]
    UnsupportedConversionFormat { format: String },

    /// The conversion result format is not supported by Rudof.
    #[error("Unsupported result conversion format: '{format}'. Valid formats are: 'default', 'internal', 'json', 'shexc', 'shexj', 'turtle', 'uml', 'html', 'svg', 'png'")]
    UnsupportedResultConversionFormat { format: String },

    /// The format cannot be converted to ShEx.
    #[error("Cannot convert format '{format}' to ShEx. Supported formats for ShEx: 'shexc', 'shexj', 'turtle'")]
    UnsupportedConversionToShEx { format: String },

    /// The format cannot be converted to SHACL.
    #[error("Cannot convert format '{format}' to SHACL. Supported formats for SHACL: 'turtle'")]
    UnsupportedConversionToShacl { format: String },

    /// The format cannot be converted to DC-TAP.
    #[error("Cannot convert format '{format}' to DC-TAP. Supported formats for DC-TAP: 'csv', 'xlsx'")]
    UnsupportedConversionToDCTap { format: String },

    /// The result conversion format cannot be used for ShEx conversion.
    #[error("Cannot use output format '{format}' for ShEx conversion. Supported formats: 'shexc', 'shexj', 'turtle'")]
    UnsupportedResultConversionFormatToShEx { format: String },

    /// The result conversion format cannot be used for SHACL conversion.
    #[error("Cannot use output format '{format}' for SHACL conversion. Supported formats: 'default', 'turtle'")]
    UnsupportedResultConversionFormatToShacl { format: String },
}