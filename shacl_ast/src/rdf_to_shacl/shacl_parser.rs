use prefixmap::PrefixMap;
use srdf::{FocusRDF, RDFParser, RDFNodeParse, ok, instances_of, parse_nodes, term, RDFNode, RDFParseError};

use crate::{schema::Schema, SH_NODE_SHAPE, node_shape::NodeShape};

use super::shacl_parser_error::ShaclParserError;

type Result<A> = std::result::Result<A, ShaclParserError>;

pub struct ShaclParser<RDF> 
where RDF: FocusRDF {
  rdf_parser: RDFParser<RDF>
}

impl<RDF> ShaclParser<RDF> 
where RDF: FocusRDF {
    pub fn new(rdf: RDF) -> ShaclParser<RDF> {
        ShaclParser {
            rdf_parser: RDFParser::new(rdf)
        }
    }

    pub fn parse(&mut self) -> Result<Schema> {
        let schema = Self::schema_parser().parse_impl(&mut self.rdf_parser.rdf).map_err(|e| {
            ShaclParserError::RDFParseError { err: e }
        })?;
        let prefixmap: PrefixMap = self.rdf_parser.prefixmap().unwrap_or_else(|| PrefixMap::new());
        Ok(schema.with_prefixmap(prefixmap))
    }

    pub fn schema_parser() -> impl RDFNodeParse<RDF, Output = Schema> {
        instances_of(&SH_NODE_SHAPE).then(|vs| {
            let terms: Vec<_> = vs.iter().map(|s| RDF::subject_as_term(s)).collect();
            parse_nodes(terms, node_shape()).flat_map(|ns| {
                let mut schema = Schema::new();
                schema.add_node_shapes(ns);
                Ok(schema)
            }
            )
          }
        )
    }

}

fn node_shape<RDF>() -> impl RDFNodeParse<RDF, Output = NodeShape> 
where RDF: FocusRDF {
    term().flat_map(|t| {
        let id = term_as_node::<RDF>(&t).map_err(|e| {
            RDFParseError::Custom { msg: format!("Expected RDFNode parsing node shape, found {t}: {e}") }
        })?;
        Ok(NodeShape::new(id))
     })
}

fn term_as_node<RDF>(term: &RDF::Term) -> Result<RDFNode> where RDF: FocusRDF {
   todo!()
}