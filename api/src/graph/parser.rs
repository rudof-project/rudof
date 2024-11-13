use std::fs::File;
use std::io::BufReader;

use oxrdfio::RdfFormat as OxRdfFormat;
use oxrdfio::RdfParser;

use crate::model::rdf::MutableRdf;
use crate::model::RdfFormat;

use super::error::RdfParserError;
use super::graph::SimpleGraph;

pub struct GraphParser;

impl GraphParser {
    pub fn parse(path: &str, format: RdfFormat) -> Result<SimpleGraph, RdfParserError> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut graph = SimpleGraph::default();

        for triple in RdfParser::from_format(format.into()).for_reader(reader) {
            let triple = triple?;
            let subject = triple.subject;
            let predicate = triple.predicate;
            let object = triple.object;
            graph.add_triple(subject, predicate, object)?;
        }

        Ok(graph)
    }
}

impl From<RdfFormat> for OxRdfFormat {
    fn from(value: RdfFormat) -> Self {
        match value {
            RdfFormat::N3 => OxRdfFormat::N3,
            RdfFormat::Turtle => OxRdfFormat::Turtle,
            RdfFormat::RdfXml => OxRdfFormat::RdfXml,
            RdfFormat::NQuads => OxRdfFormat::NQuads,
            RdfFormat::NTriples => OxRdfFormat::NTriples,
            RdfFormat::TriG => OxRdfFormat::TriG,
        }
    }
}
