use crate::rdf_core::RDFError;
use iri_s::MimeType;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

/// Represents RDF serialization formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Serialize, Deserialize)]
pub enum RDFFormat {
    /// Turtle (Terse RDF Triple Language) format.
    #[default]
    Turtle,
    /// N-Triples format.
    NTriples,
    /// RDF/XML format.
    RDFXML,
    /// TriG format.
    TriG,
    /// N3 (Notation3) format.
    N3,
    /// N-Quads format.
    NQuads,
    /// JSON-LD (JSON for Linking Data) format.
    JsonLd,
}

impl RDFFormat {
    /// Returns the file extensions associated with this format.
    /// 
    /// # Extensions by Format
    ///
    /// - **Turtle**: `["ttl", "turtle"]`
    /// - **N-Triples**: `["nt"]`
    /// - **RDF/XML**: `["rdf", "xml"]`
    /// - **TriG**: `["trig"]`
    /// - **N3**: `["n3"]`
    /// - **NQuads**: `["nq", "nquads"]`
    /// - **JSON-LD**: `["jsonld", "json-ld", "json"]`
    pub fn extensions(&self) -> Vec<&'static str> {
        match self {
            RDFFormat::Turtle => vec!["ttl", "turtle"],
            RDFFormat::NTriples => vec!["nt"],
            RDFFormat::RDFXML => vec!["rdf", "xml"],
            RDFFormat::TriG => vec!["trig"],
            RDFFormat::N3 => vec!["n3"],
            RDFFormat::NQuads => vec!["nq", "nquads"],
            RDFFormat::JsonLd => vec!["jsonld", "json-ld", "json"],
        }
    }
}

impl MimeType for RDFFormat {
    /// Returns the IANA-registered MIME type for this RDF format.
    ///
    /// # MIME Types by Format
    ///
    /// - **Turtle**: `text/turtle`
    /// - **N-Triples**: `application/n-triples`
    /// - **RDF/XML**: `application/rdf+xml`
    /// - **TriG**: `application/trig`
    /// - **N3**: `text/n3`
    /// - **NQuads**: `application/n-quads`
    /// - **JSON-LD**: `application/ld+json`
    fn mime_type(&self) -> &'static str {
        match self {
            RDFFormat::Turtle => "text/turtle",
            RDFFormat::NTriples => "application/n-triples",
            RDFFormat::RDFXML => "application/rdf+xml",
            RDFFormat::TriG => "application/trig",
            RDFFormat::N3 => "text/n3",
            RDFFormat::NQuads => "application/n-quads",
            RDFFormat::JsonLd => "application/ld+json",
        }
    }
}

impl FromStr for RDFFormat {
    /// Error type returned when parsing fails.
    type Err = RDFError;

    /// Parses a string into an RDF format.
    ///
    /// This implementation is case-insensitive and accepts multiple variations
    /// for each format, including format names and common abbreviations.
    ///
    /// # Accepted Strings (case-insensitive)
    ///
    /// - **Turtle**: "ttl", "turtle"
    /// - **N-Triples**: "ntriples", "nt"
    /// - **RDF/XML**: "rdf/xml", "rdf"
    /// - **TriG**: "trig"
    /// - **N3**: "n3"
    /// - **NQuads**: "nquads", "nq"
    /// - **JSON-LD**: "jsonld", "json"
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse (case-insensitive)
    /// 
    /// # Errors
    ///
    /// Returns [`RDFError::NotSupportedRDFFormatError`] if the input string doesn't match any known format.
    fn from_str(s: &str) -> Result<RDFFormat, RDFError> {
        match s.to_lowercase().as_str() {
            "ttl" => Ok(RDFFormat::Turtle),
            "turtle" => Ok(RDFFormat::Turtle),
            "ntriples" => Ok(RDFFormat::NTriples),
            "nt" => Ok(RDFFormat::NTriples),
            "rdf/xml" => Ok(RDFFormat::RDFXML),
            "rdf" => Ok(RDFFormat::RDFXML),
            "trig" => Ok(RDFFormat::TriG),
            "n3" => Ok(RDFFormat::N3),
            "nquads" => Ok(RDFFormat::NQuads),
            "nq" => Ok(RDFFormat::NQuads),
            "jsonld" => Ok(RDFFormat::JsonLd),
            "json" => Ok(RDFFormat::JsonLd),
            _ => Err(RDFError::NotSupportedRDFFormatError {
                format: format!("Format {s} not supported").to_string(),
            }),
        }
    }
}

impl Display for RDFFormat {
    /// Formats the RDF format as its canonical name.
    /// 
    /// # Format Names
    ///
    /// - Turtle → "Turtle"
    /// - N-Triples → "N-Triples"
    /// - RDF/XML → "RDF/XML"
    /// - TriG → "TriG"
    /// - N3 → "N3"
    /// - NQuads → "NQuads"
    /// - JSON-LD → "JSONLD"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RDFFormat::Turtle => write!(f, "Turtle"),
            RDFFormat::NTriples => write!(f, "N-Triples"),
            RDFFormat::RDFXML => write!(f, "RDF/XML"),
            RDFFormat::TriG => write!(f, "TriG"),
            RDFFormat::N3 => write!(f, "N3"),
            RDFFormat::NQuads => write!(f, "NQuads"),
            RDFFormat::JsonLd => write!(f, "JSONLD"),
        }
    }
}
