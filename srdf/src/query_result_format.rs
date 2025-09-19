use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryResultFormat {
    Turtle,
    NTriples,
    JsonLd,
    RdfXml,
    Csv,
    TriG,
    N3,
    NQuads,
}

impl Display for QueryResultFormat {
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
    type Err = String;

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
