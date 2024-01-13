use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use srdf::{
    combine_vec, has_type, instances_of, ok, optional, parse_nodes, property_value,
    property_values, term, FocusRDF, RDFNode, RDFNodeParse, RDFParseError, RDFParser, RDF_TYPE, SHACLPath, Object, Triple, property_value_debug,
};

use crate::{
    node_shape::NodeShape,
    schema::Schema,
    shape::Shape,
    target::{self, Target},
    SH_NODE_SHAPE, SH_PROPERTY, SH_TARGET_CLASS, SH_TARGET_NODE, SH_PROPERTY_SHAPE, property_shape::PropertyShape, SH_PATH,
};
use std::fmt::Debug;

use super::shacl_parser_error::ShaclParserError;

type Result<A> = std::result::Result<A, ShaclParserError>;

pub struct ShaclParser<RDF>
where
    RDF: FocusRDF + Debug,
{
    rdf_parser: RDFParser<RDF>,
    shapes: HashMap<RDFNode, Shape>,
    pending: Vec<RDFNode>,
}

impl<RDF> ShaclParser<RDF>
where
    RDF: FocusRDF + Debug,
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
        let node_shape_instances = self
            .rdf_parser
            .rdf
            .subjects_with_predicate_object(&Self::rdf_type(), &Self::sh_node_shape())
            .map_err(|e| ShaclParserError::Custom {
                msg: format!("Error obtaining values with type sh:NodeShape: {e}"),
            })?;
        
        // subjects with property `sh:property`
        let subjects_property = self.objects_with_predicate(Self::sh_property())?;
            
        // TODO: subjects with type `sh:PropertyShape`
        let property_shapes_instances = HashSet::new(); 

        // TODO: subjects with type `sh:Shape`
        let shape_instances = HashSet::new(); 

        // I would prefer a code like: node_shape_instances.union(subjects_property).union(...)
        // But looking to the union API in HashSet, I think it can't be chained
        let mut candidates = HashSet::new();
        candidates.extend(node_shape_instances);
        candidates.extend(subjects_property);
        candidates.extend(property_shapes_instances);
        candidates.extend(shape_instances);
        
        let result: Vec<_> = candidates.iter().map(|s| Self::subject_to_node(s)).collect();
        Ok(result)
    }

    fn objects_with_predicate(&self, pred: RDF::IRI) -> Result<HashSet<RDF::Subject>> {
        let triples = self
        .rdf_parser
        .rdf
        .triples_with_predicate(&pred)
        .map_err(|e| ShaclParserError::Custom {
            msg: format!("Error obtaining values with predicate sh:property: {e}"),
        })?;
        let values_as_subjects = triples.iter().flat_map(Self::triple_object_as_subject).collect();
        Ok(values_as_subjects)
    } 


    fn rdf_type() -> RDF::IRI {
        let iri = RDF::iri_s2iri(&RDF_TYPE);
        iri
    }

    fn sh_node_shape() -> RDF::Term {
        let iri = RDF::iri_s2term(&SH_NODE_SHAPE);
        iri
    }

    fn sh_property() -> RDF::IRI {
        let iri = RDF::iri_s2iri(&SH_PROPERTY);
        iri
    }

    fn triple_object_as_subject(triple: &Triple<RDF>) -> Result<RDF::Subject> {
        let subj = RDF::term_as_subject(&triple.obj()).ok_or_else(|| 
            ShaclParserError::Custom { msg: format!("Expected triple object value to act as a subject: {triple}")}
        )?;
        Ok(subj)
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
        Self::node_shape().then(move |ns| ok(&Shape::NodeShape(ns))).
        or(Self::property_shape().then(move |ps| ok(&Shape::PropertyShape(ps))))
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

    fn property_shape() -> impl RDFNodeParse<RDF, Output = PropertyShape>
    where
        RDF: FocusRDF,
    {
        optional(has_type(SH_PROPERTY_SHAPE.clone()))
            .with(id().and(path()).then(move |(id, path)| {
                ok(&PropertyShape::new(id, path))
            }))
            .then(|ps| targets().flat_map(move |ts| Ok(ps.clone().with_targets(ts))))
            .then(|ps| {
                property_shapes().flat_map(move |prop_shapes| Ok(ps.clone().with_property_shapes(prop_shapes)))
            })
    }


}

fn id<RDF>() -> impl RDFNodeParse<RDF, Output = RDFNode> 
where RDF: FocusRDF {
    term().then(move |t: RDF::Term| {
        let id = RDF::term_as_object(&t.clone());
        ok(&id)
    })
}

/// Parses the property value of the focus node as a SHACL path
fn path<RDF>() -> impl RDFNodeParse<RDF, Output = SHACLPath>
where RDF: FocusRDF + Debug {
    property_value_debug(&SH_PATH).then(shacl_path)
}

/// Parses the current focus node as a SHACL path
fn shacl_path<RDF>(term: RDF::Term) -> impl RDFNodeParse<RDF, Output = SHACLPath> 
where RDF: FocusRDF {
    let obj = RDF::term_as_object(&term);
    match obj {
        Object::Iri { iri } => ok(&SHACLPath::iri(iri)),
        Object::BlankNode(_) => todo!(),
        Object::Literal(_) => todo!(),
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
