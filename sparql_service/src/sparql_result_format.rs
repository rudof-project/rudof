use iri_s::IriS;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum SparqlResultFormat {
    Xml,
    Turtle,
    Tsv,
    RdfXml,
    Json,
    NTriples,
    Csv,
    JsonLD,
    Other(IriS),
}

impl Display for SparqlResultFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SparqlResultFormat::Xml => write!(f, "XML"),
            SparqlResultFormat::Turtle => write!(f, "Turtle"),
            SparqlResultFormat::Tsv => write!(f, "TSV"),
            SparqlResultFormat::RdfXml => write!(f, "RDF/XML"),
            SparqlResultFormat::Json => write!(f, "JSON"),
            SparqlResultFormat::NTriples => write!(f, "N-TRIPLES"),
            SparqlResultFormat::Csv => write!(f, "CSV"),
            SparqlResultFormat::JsonLD => write!(f, "JSON_LD"),
            SparqlResultFormat::Other(iri) => write!(f, "ResultFormat({iri})",),
        }
    }
}
