use thiserror::Error;

/// Errors that can occur when working with ShapeMaps.
#[derive(Error, Debug)]
pub enum ShapeMapError {
    /// The ShapeMap format specified is not supported by Rudof.
    #[error(
        "Unsupported ShapeMap format: '{format}'. Valid formats are: 'compact', 'internal', 'json', 'details', 'csv'"
    )]
    UnsupportedShapeMapFormat { format: String },
}
