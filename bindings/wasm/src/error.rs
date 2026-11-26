use rudof_lib::errors::RudofError;
use std::fmt::{Display, Formatter};

/// An error raised by the bindings: a message and the category it belongs to.
///
/// The categories follow the exception classes of the Python bindings
/// (`ShExError`, `DataError`, ...). In JavaScript they become the `name` of the
/// thrown `Error`, so callers can tell them apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    name: &'static str,
    message: String,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn new(name: &'static str, message: impl Into<String>) -> Self {
        Error {
            name,
            message: message.into(),
        }
    }

    /// An argument has a value that is not one of the accepted ones, e.g. an
    /// unknown format name.
    pub(crate) fn invalid_argument(message: impl Into<String>) -> Self {
        Error::new("RangeError", message)
    }

    /// The error category, e.g. `ShExError`.
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.message)
    }
}

impl std::error::Error for Error {}

impl From<RudofError> for Error {
    fn from(error: RudofError) -> Self {
        let name = match &error {
            RudofError::Config(_) => "ConfigError",
            RudofError::InputSpec(_) => "InputError",
            RudofError::Data(_) => "DataError",
            RudofError::ShapeMap(_) => "ShapeMapError",
            RudofError::ShEx(_) => "ShExError",
            RudofError::Shacl(_) => "ShaclError",
            RudofError::PgSchema(_) => "PgSchemaError",
            RudofError::PgDb(_) => "PgDbError",
            RudofError::Validation(_) => "ValidationError",
            RudofError::NodeInspection(_) => "NodeInspectionError",
            RudofError::DCTap(_) => "DCTapError",
            RudofError::Conversion(_) => "ConversionError",
            RudofError::Comparison(_) => "ComparisonError",
            RudofError::RdfConfig(_) => "RdfConfigError",
            RudofError::Service(_) => "ServiceError",
            RudofError::Query(_) | RudofError::UnsupportedResultQueryFormatSelect { .. } => "QueryError",
            RudofError::Generate(_) => "GenerateError",
            RudofError::Iri(_) => "IriError",
            RudofError::MapState(_) => "MapStateError",
            RudofError::Materialize(_) => "MaterializeError",
            RudofError::Prefixes(_) => "PrefixesError",
            RudofError::NotImplemented { .. } => "UnsupportedOperationError",
            RudofError::Generic { .. } => "RudofError",
        };
        Error::new(name, error.to_string())
    }
}

impl From<rudof_lib::ConfigError> for Error {
    fn from(error: rudof_lib::ConfigError) -> Self {
        Error::new("ConfigError", error.to_string())
    }
}
