use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum SparqlResultFormat {
    XML,
    Turtle,
    TSV,
    RdfXml,
    JSON,
    NTriples,
    CSV,
    JsonLD,
    Other(IriS),
}

impl Display for SparqlResultFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SparqlResultFormat::XML => write!(f, "XML"),
            SparqlResultFormat::Turtle => write!(f, "Turtle"),
            SparqlResultFormat::TSV => write!(f, "TSV"),
            SparqlResultFormat::RdfXml => write!(f, "RDF/XML"),
            SparqlResultFormat::JSON => write!(f, "JSON"),
            SparqlResultFormat::NTriples => write!(f, "N-TRIPLES"),
            SparqlResultFormat::CSV => write!(f, "CSV"),
            SparqlResultFormat::JsonLD => write!(f, "JSON_LD"),
            SparqlResultFormat::Other(iri) => write!(f, "ResultFormat({iri})",),
        }
    }
}
