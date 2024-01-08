use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use srdf::{
    combine_vec, has_type, instances_of, ok, optional, parse_nodes, property_value,
    property_values, term, FocusRDF, RDFNode, RDFNodeParse, RDFParseError, RDFParser, RDF_TYPE,
};

use crate::{
    node_shape::NodeShape,
    schema::Schema,
    shape::Shape,
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
    shapes: HashMap<RDFNode, Shape>,
    pending: Vec<RDFNode>,
}

impl<RDF> ShaclParser<RDF>
where
    RDF: FocusRDF,
{
    pub fn new(rdf: RDF) -> ShaclParser<RDF> {
        ShaclParser {
            rdf_parser: RDFParser::new(rdf),
            shapes: HashMap::new(),
            pending: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Schema> {
        /*let schema = Self::schema_parser()
        .parse_impl(&mut self.rdf_parser.rdf)
        .map_err(|e| ShaclParserError::RDFParseError { err: e })?; */
        let prefixmap: PrefixMap = self
            .rdf_parser
            .prefixmap()
            .unwrap_or_else(|| PrefixMap::new());
        let mut pending = self.shapes_candidates()?;
        while let Some(node) = pending.pop() {
            if !self.shapes.contains_key(&node) {
                let term = RDF::object_as_term(&node);
                self.rdf_parser.rdf.set_focus(&term);
                let shape = Self::shape()
                    .parse_impl(&mut self.rdf_parser.rdf)
                    .map_err(|e| ShaclParserError::RDFParseError { err: e })?;
                self.shapes.insert(node, shape);
            }
        }
        Ok(Schema::new()
            .with_prefixmap(prefixmap)
            .with_shapes(self.shapes.clone()))
    }

    fn shapes_candidates(&self) -> Result<Vec<RDFNode>> {
        // subjects with type `sh:NodeShape`
        let subjects = self
            .rdf_parser
            .rdf
            .subjects_with_predicate_object(&Self::rdf_type(), &Self::sh_node_shape())
            .map_err(|e| ShaclParserError::Custom {
                msg: format!("Error obtaining values with type sh:NodeShape: {e}"),
            })?;
        // subjects with type `sh:PropertyShape`
        // subjects with type `sh:Shape`
        // subjects with property `sh:property`
        let result: Vec<_> = subjects.iter().map(|s| Self::subject_to_node(s)).collect();
        Ok(result)
    }

    fn rdf_type() -> RDF::IRI {
        let iri = RDF::iri_s2iri(&RDF_TYPE);
        iri
    }

    fn sh_node_shape() -> RDF::Term {
        let iri = RDF::iri_s2term(&SH_NODE_SHAPE);
        iri
    }

    fn subject_to_node(subject: &RDF::Subject) -> RDFNode {
        let obj = RDF::subject_as_object(subject);
        obj
    }

    /*pub fn schema_parser() -> impl RDFNodeParse<RDF, Output = Schema> {
        instances_of(&SH_NODE_SHAPE).then(|vs| {
            let terms: Vec<_> = vs.iter().map(|s| RDF::subject_as_term(s)).collect();
            parse_nodes(terms, Self::node_shape()).flat_map(|ns| {
                let mut schema = Schema::new();
                schema
                    .add_node_shapes(ns)
                    .map_err(|e| RDFParseError::Custom {
                        msg: format!("Error adding node shapes: {e}"),
                    })?;
                Ok(schema)
            })
        })
    }*/

    fn shape() -> impl RDFNodeParse<RDF, Output = Shape>
    where
        RDF: FocusRDF,
    {
        Self::node_shape().then(move |ns| ok(&Shape::NodeShape(ns)))
    }

    fn node_shape() -> impl RDFNodeParse<RDF, Output = NodeShape>
    where
        RDF: FocusRDF,
    {
        has_type(SH_NODE_SHAPE.clone())
            .with(term().then(move |t: RDF::Term| {
                let id = RDF::term_as_object(&t.clone());
                ok(&NodeShape::new(id))
            }))
            .then(|ns| targets().flat_map(move |ts| Ok(ns.clone().with_targets(ts))))
            .then(|ns| {
                property_shapes().flat_map(move |ps| Ok(ns.clone().with_property_shapes(ps)))
            })
    }
}

fn property_shapes<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<RDFNode>>
where
    RDF: FocusRDF,
{
    property_values(&SH_PROPERTY).flat_map(|ts| {
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
    property_values(&SH_TARGET_CLASS).flat_map(move |ts| {
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
    property_values(&SH_TARGET_NODE).flat_map(move |ts| {
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
