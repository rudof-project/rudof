use std::ops::Deref;

use iri_s::IriS;
use prefixmap::PrefixMap;
use srdf::{
    combine_vec, has_type, instances_of, ok, optional, parse_nodes, property_value,
    property_values, term, FocusRDF, RDFNode, RDFNodeParse, RDFParseError, RDFParser,
};

use crate::{
    node_shape::NodeShape,
    schema::Schema,
    target::{self, Target},
    SH_NODE_SHAPE, SH_PROPERTY, SH_TARGET_CLASS, SH_TARGET_NODE,
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
    has_type(SH_NODE_SHAPE.clone())
        .with(term().then(move |t: RDF::Term| {
            let id = RDF::term_as_object(&t.clone());
            ok(&NodeShape::new(id))
        }))
        .then(|ns| targets().flat_map(move |ts| Ok(ns.clone().with_targets(ts))))
        .then(|ns| property_shapes().flat_map(move |ps| Ok(ns.clone().with_property_shapes(ps))))
}

fn property_shapes<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<RDFNode>>
where
    RDF: FocusRDF,
{
    let property = RDF::iri_s2iri(&SH_PROPERTY);
    property_values(&property).flat_map(|ts| {
        let nodes = ts.iter().map(|t| RDF::term_as_object(t)).collect();
        Ok(nodes)
    })
}

fn targets<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target>>
where
    RDF: FocusRDF,
{
    combine_vec(targets_class(), targets_node())
}

fn targets_class<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target>>
where
    RDF: FocusRDF,
{
    let target_class_property = RDF::iri_s2iri(&SH_TARGET_CLASS);
    property_values(&target_class_property).flat_map(move |ts| {
        let result = ts
            .iter()
            .map(|t| {
                let node = RDF::term_as_object(&t);
                Target::TargetClass(node)
            })
            .collect();
        Ok(result)
    })
}

fn targets_node<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target>>
where
    RDF: FocusRDF,
{
    let target_class_property = RDF::iri_s2iri(&SH_TARGET_NODE);
    property_values(&target_class_property).flat_map(move |ts| {
        let result = ts
            .iter()
            .map(|t| {
                let node = RDF::term_as_object(&t);
                Target::TargetNode(node)
            })
            .collect();
        Ok(result)
    })
}

/* .then(move |ns| {
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
*/
