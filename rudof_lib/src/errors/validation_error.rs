use thiserror::Error;

/// Errors that can occur during validation operations.
#[derive(Error, Debug)]
pub enum ValidationError {
    /// The validation mode specified is not supported by Rudof.
    #[error("Unsupported validation mode: '{mode}'. Valid modes are: 'shex', 'shacl', 'pgschema'")]
    UnsupportedValidationMode { mode: String },

    /// The SHACL validation mode specified is not supported by Rudof.
    #[error("Unsupported SHACL validation mode: '{mode}'. Valid modes are: 'native', 'sparql'")]
    UnsupportedSHACLValidationMode { mode: String },

    /// The validation result sorting mode is not supported by Rudof.
    #[error("Unsupported validation sort by mode: '{mode}'. Valid options are: 'node', 'details'")]
    UnsupportedValidationSortByMode { mode: String },

    /// The ShEx validation result sorting mode is not supported by Rudof.
    #[error(
        "Unsupported ShEx validation sort by mode: '{mode}'. Valid options are: 'node', 'shape', 'status', 'details'"
    )]
    UnsupportedShExValidationSortByMode { mode: String },

    /// The SHACL validation result sorting mode is not supported by Rudof.
    #[error(
        "Unsupported SHACL validation sort by mode: '{mode}'. Valid options are: 'severity', 'node', 'component', 'value', 'path', 'sourceshape', 'details'"
    )]
    UnsupportedShaclValidationSortByMode { mode: String },

    /// The validation result format is not supported by Rudof.
    #[error(
        "Unsupported validation result format: '{format}'. Valid formats are: 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads', 'compact', 'details', 'json', 'csv'"
    )]
    UnsupportedValidationResultFormat { format: String },

    /// The ShEx validation result format is not supported by Rudof.
    #[error(
        "Unsupported ShEx validation result format: '{format}'. Valid formats are: 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads', 'compact', 'details', 'json', 'csv'"
    )]
    UnsupportedShExValidationResultFormat { format: String },

    /// RDF-based formats cannot be converted to ShapeMap format.
    #[error(
        "Cannot convert format '{format}' to ShapeMap format. Only 'compact', 'details', 'json', and 'csv' are supported for ShapeMap conversion"
    )]
    UnsupportedConversionToShapeMap { format: String },

    /// RDF-based formats cannot be converted to Property Graph schema validation format.
    #[error(
        "Cannot convert format '{format}' to Property Graph schema validation format. Only 'compact', 'details', 'json', and 'csv' are supported for Property Graph schema validation conversion"
    )]
    UnsupportedConversionToPgSchemaValidationResultFormat { format: String },

    /// Non-RDF formats cannot be converted to RDF format.
    #[error("Cannot convert format '{format}' to RDF format.")]
    UnsupportedConversionToRDFFormat { format: String },

    /// The SHACL validation result format is not supported by Rudof.
    #[error(
        "Unsupported SHACL validation result format: '{format}'. Valid formats are: 'turtle', 'ntriples', 'rdfxml', 'trig', 'n3', 'nquads', 'minimal', 'compact', 'details', 'json', 'csv'"
    )]
    UnsupportedShaclValidationResultFormat { format: String },

    /// The Property Graph schema validation result format is not supported by Rudof.
    #[error(
        "Unsupported Property Graph schema validation result format: '{format}'. Valid formats are: 'compact', 'details', 'json', 'csv'"
    )]
    NoSupportedPgSchemaValidationResultFormat { format: String },
}
