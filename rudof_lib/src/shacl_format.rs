use iri_s::MimeType;
use shacl_ast::ShaclFormat as ShaclAstShaclFormat;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub enum ShaclFormat {
    Internal,
    #[default]
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    JsonLd,
}

// Convert from CLI ShaclFormat to shacl_ast::ShaclFormat (library type)
impl From<ShaclFormat> for ShaclAstShaclFormat {
    fn from(format: ShaclFormat) -> Self {
        match format {
            ShaclFormat::Turtle => ShaclAstShaclFormat::Turtle,
            ShaclFormat::RdfXml => ShaclAstShaclFormat::RdfXml,
            ShaclFormat::NTriples => ShaclAstShaclFormat::NTriples,
            ShaclFormat::TriG => ShaclAstShaclFormat::TriG,
            ShaclFormat::N3 => ShaclAstShaclFormat::N3,
            ShaclFormat::NQuads => ShaclAstShaclFormat::NQuads,
            ShaclFormat::Internal => ShaclAstShaclFormat::Internal,
            ShaclFormat::JsonLd => ShaclAstShaclFormat::JsonLd,
        }
    }
}

// Convert from reference &ShaclFormat to ShaclAstShaclFormat
impl From<&ShaclFormat> for ShaclAstShaclFormat {
    fn from(format: &ShaclFormat) -> Self {
        (*format).into()
    }
}

impl MimeType for ShaclFormat {
    fn mime_type(&self) -> &'static str {
        match self {
            ShaclFormat::Turtle => "text/turtle",
            ShaclFormat::NTriples => "application/n-triples",
            ShaclFormat::RdfXml => "application/rdf+xml",
            ShaclFormat::TriG => "application/trig",
            ShaclFormat::N3 => "text/n3",
            ShaclFormat::NQuads => "application/n-quads",
            ShaclFormat::Internal => "text/turtle",
            ShaclFormat::JsonLd => "application/ld+json",
        }
    }
}

impl Display for ShaclFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShaclFormat::Internal => write!(dest, "internal"),
            ShaclFormat::Turtle => write!(dest, "turtle"),
            ShaclFormat::NTriples => write!(dest, "NTriples"),
            ShaclFormat::RdfXml => write!(dest, "rdfxml"),
            ShaclFormat::TriG => write!(dest, "trig"),
            ShaclFormat::N3 => write!(dest, "n3"),
            ShaclFormat::NQuads => write!(dest, "nquads"),
            ShaclFormat::JsonLd => write!(dest, "jsonld"),
        }
    }
}

impl FromStr for ShaclFormat {
    type Err = crate::RudofError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "internal" => Ok(ShaclFormat::Internal),
            "turtle" => Ok(ShaclFormat::Turtle),
            "ntriples" => Ok(ShaclFormat::NTriples),
            "rdfxml" => Ok(ShaclFormat::RdfXml),
            "trig" => Ok(ShaclFormat::TriG),
            "n3" => Ok(ShaclFormat::N3),
            "nquads" => Ok(ShaclFormat::NQuads),
            "jsonld" => Ok(ShaclFormat::JsonLd),
            other => Err(crate::RudofError::UnsupportedShaclFormat {
                format: other.to_string(),
            }),
        }
    }
}
