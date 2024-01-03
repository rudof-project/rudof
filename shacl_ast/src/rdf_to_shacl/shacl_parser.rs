use prefixmap::PrefixMap;
use srdf::{
    instances_of, ok, optional, parse_nodes, property_value, term, FocusRDF, RDFNode, RDFNodeParse,
    RDFParseError, RDFParser,
};

use crate::{
    node_shape::NodeShape, schema::Schema, target::Target, SH_NODE_SHAPE, SH_TARGET_CLASS,
};

use super::shacl_parser_error::ShaclParserError;

type Result<A> = std::result::Result<A, ShaclParserError>;

pub struct ShaclParser<RDF>
where
    RDF: FocusRDF,
{
    rdf_parser: RDFParser<RDF>,
}

impl<RDF> ShaclParser<RDF>
where
    RDF: FocusRDF,
{
    pub fn new(rdf: RDF) -> ShaclParser<RDF> {
        ShaclParser {
            rdf_parser: RDFParser::new(rdf),
        }
    }

    pub fn parse(&mut self) -> Result<Schema> {
        let schema = Self::schema_parser()
            .parse_impl(&mut self.rdf_parser.rdf)
            .map_err(|e| ShaclParserError::RDFParseError { err: e })?;
        let prefixmap: PrefixMap = self
            .rdf_parser
            .prefixmap()
            .unwrap_or_else(|| PrefixMap::new());
        Ok(schema.with_prefixmap(prefixmap))
    }

    pub fn schema_parser() -> impl RDFNodeParse<RDF, Output = Schema> {
        instances_of(&SH_NODE_SHAPE).then(|vs| {
            let terms: Vec<_> = vs.iter().map(|s| RDF::subject_as_term(s)).collect();
            parse_nodes(terms, node_shape()).flat_map(|ns| {
                let mut schema = Schema::new();
                schema
                    .add_node_shapes(ns)
                    .map_err(|e| RDFParseError::Custom {
                        msg: format!("Error adding node shapes: {e}"),
                    })?;
                Ok(schema)
            })
        })
    }
}

fn node_shape<RDF>() -> impl RDFNodeParse<RDF, Output = NodeShape>
where
    RDF: FocusRDF,
{
    term()
        .flat_map(|t| {
            let id = RDF::term_as_object(&t);
            Ok(NodeShape::new(id))
        })
        .then(move |ns| {
            optional(property_value(&SH_TARGET_CLASS)).flat_map(move |maybe_target_class| {
                println!("Maybe target_class: {maybe_target_class:?}");
                let ns = match maybe_target_class {
                    None => ns.clone(),
                    Some(term) => {
                        let mut new_ns = ns.clone();
                        let node = RDF::term_as_object(&term);
                        new_ns.add_target(Target::TargetClass(node));
                        new_ns
                    }
                };
                Ok(ns)
            })
        })
}
