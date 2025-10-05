use std::fmt::{Display, Formatter};

use iri_s::MimeType;
use serde::{Deserialize, Serialize};
use srdf::RDFFormat;

/// Contains possible ShEx formats
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
pub enum ShExFormat {
    #[default]
    ShExC,
    ShExJ,
    RDFFormat(RDFFormat),
}

impl MimeType for ShExFormat {
    fn mime_type(&self) -> &'static str {
        match self {
            ShExFormat::ShExC => "text/shex",
            ShExFormat::ShExJ => "application/shex+json",
            ShExFormat::RDFFormat(rdf_format) => rdf_format.mime_type(),
        }
    }
}

impl Display for ShExFormat {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ShExFormat::ShExC => write!(f, "ShExC"),
            ShExFormat::ShExJ => write!(f, "ShExJ"),
            ShExFormat::RDFFormat(rdf_format) => write!(f, "{}", rdf_format),
        }
    }
}
