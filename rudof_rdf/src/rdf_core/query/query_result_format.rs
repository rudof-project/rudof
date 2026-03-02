use std::{fmt::Display, str::FromStr};

/// Represents RDF serialization formats for query results.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryResultFormat {
    /// Turtle (Terse RDF Triple Language) format.
    Turtle,
    /// N-Triples format.
    NTriples,
    /// JSON-LD (JSON for Linking Data) format.
    JsonLd,
    /// RDF/XML format.
    RdfXml,
    /// CSV (Comma-Separated Values) format.
    Csv,
    /// TriG format.
    TriG,
    /// N3 (Notation3) format.
    N3,
    /// N-Quads format.
    NQuads,
}

impl QueryResultFormat {
    pub fn mime_type(&self) -> &'static str {
        match self {
            QueryResultFormat::Turtle => "text/turtle",
            QueryResultFormat::NTriples => "application/n-triples",
            QueryResultFormat::JsonLd => "application/ld+json",
            QueryResultFormat::RdfXml => "application/rdf+xml",
            QueryResultFormat::Csv => "text/csv",
            QueryResultFormat::TriG => "application/trig",
            QueryResultFormat::N3 => "text/n3",
            QueryResultFormat::NQuads => "application/n-quads",
        }
    }
}

impl Display for QueryResultFormat {
    /// Formats the query result format as its canonical name.
    ///
    /// # Format Names
    ///
    /// - Turtle → "Turtle"
    /// - N-Triples → "N-Triples"
    /// - JSON-LD → "JSON-LD"
    /// - RDF/XML → "RDF/XML"
    /// - CSV → "CSV"
    /// - TriG → "TriG"
    /// - N3 → "N3"
    /// - NQuads → "NQuads"
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            QueryResultFormat::Turtle => "Turtle",
            QueryResultFormat::NTriples => "N-Triples",
            QueryResultFormat::JsonLd => "JSON-LD",
            QueryResultFormat::RdfXml => "RDF/XML",
            QueryResultFormat::Csv => "CSV",
            QueryResultFormat::TriG => "TriG",
            QueryResultFormat::N3 => "N3",
            QueryResultFormat::NQuads => "NQuads",
        };
        write!(f, "{s}")
    }
}

impl FromStr for QueryResultFormat {
    /// Error type returned when parsing fails.
    type Err = String;

    /// Parses a string into a query result format.
    ///
    /// This implementation is case-insensitive and accepts multiple variations
    /// for each format, including common abbreviations and alternative spellings.
    ///
    /// # Accepted Strings
    ///
    /// - **NQuads**: "nquads", "nq"
    /// - **N3**: "n3"
    /// - **TriG**: "trig"
    /// - **Turtle**: "turtle", "ttl"
    /// - **N-Triples**: "ntriples", "nt"
    /// - **JSON-LD**: "jsonld", "json-ld"
    /// - **RDF/XML**: "rdfxml", "rdf-xml", "rdf/xml"
    /// - **CSV**: "csv"
    /// # Errors
    ///
    /// Returns an error message in the format: `"Unknown query result format: {input}"`
    /// if the input string doesn't match any known format.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nquads" | "nq" => Ok(QueryResultFormat::NQuads),
            "n3" => Ok(QueryResultFormat::N3),
            "trig" => Ok(QueryResultFormat::TriG),
            "turtle" | "ttl" => Ok(QueryResultFormat::Turtle),
            "ntriples" | "nt" => Ok(QueryResultFormat::NTriples),
            "jsonld" | "json-ld" => Ok(QueryResultFormat::JsonLd),
            "rdfxml" | "rdf-xml" | "rdf/xml" => Ok(QueryResultFormat::RdfXml),
            "csv" => Ok(QueryResultFormat::Csv),
            _ => Err(format!("Unknown query result format: {s}")),
        }
    }
}
