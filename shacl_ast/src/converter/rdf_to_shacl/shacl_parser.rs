use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use srdf::{
    rdf_parser,
    combine_vec, has_type, instances_of, ok, optional, parse_nodes, property_value, then, then_state, 
    property_values, property_values_int, term, FocusRDF, RDFNode, RDFNodeParse, RDFParseError, RDFParser, RDF_TYPE, SHACLPath, Object, Triple, property_value_debug, combine_parsers, property_values_iri, SRDFBasic, set_focus, parse_rdf_list, rdf_list, PResult,
};

use crate::{
    node_shape::NodeShape,
    schema::Schema,
    shape::Shape,
    target::{self, Target},
    SH_NODE_SHAPE, SH_PROPERTY, SH_TARGET_CLASS, SH_TARGET_NODE, SH_PROPERTY_SHAPE, property_shape::PropertyShape, SH_PATH, component::Component, SH_MIN_COUNT, SH_MAX_COUNT, SH_DATATYPE, SH_CLASS, SH_NODE_KIND, node_kind::NodeKind, SH_IRI_STR, SH_LITERAL_STR, SH_BLANKNODE_STR, SH_BLANK_NODE_OR_IRI_STR, SH_BLANK_NODE_OR_LITERAL_STR, SH_IRI_OR_LITERAL_STR, SH_IN, value::Value, SH_OR,
};
use std::fmt::Debug;

use super::shacl_parser_error::ShaclParserError;

type Result<A> = std::result::Result<A, ShaclParserError>;


struct State {
    pending: Vec<RDFNode>,
    shapes: HashMap<RDFNode, Shape>
}

impl State {
    fn new() -> Self {
       State {
         pending: Vec::new(),
         shapes: HashMap::new()
       }   
    }

    fn from(pending: Vec<RDFNode>) -> Self {
        State {
            pending,
            shapes: HashMap::new()
        }
    }

    fn pop_pending(&mut self) -> Option<RDFNode> {
        self.pending.pop()
    }
}

pub struct ShaclParser<RDF>
where
    RDF: FocusRDF + Debug,
{
    rdf_parser: RDFParser<RDF>,
    shapes: HashMap<RDFNode, Shape>,
}

impl<RDF> ShaclParser<RDF>
where
    RDF: FocusRDF + Debug,
{
    pub fn new(rdf: RDF) -> ShaclParser<RDF> {
        ShaclParser {
            rdf_parser: RDFParser::new(rdf),
            shapes: HashMap::new(),
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
        let mut state = State::from(self.shapes_candidates()?);
        while let Some(node) = state.pop_pending() {
            if !self.shapes.contains_key(&node) {
                let term = RDF::object_as_term(&node);
                self.rdf_parser.rdf.set_focus(&term);
                let shape = Self::shape(&mut state)
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

    fn shape<'a>(state: &'a mut State) -> impl RDFNodeParse<RDF, Output = Shape> + 'a
    where
        RDF: FocusRDF + 'a,
    {
        node_shape().then(move |ns| ok(&Shape::NodeShape(ns))).
        or(property_shape(state).then( |ps| ok(&Shape::PropertyShape(ps))))
    }

}

fn components<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where RDF: FocusRDF
{
  combine_parsers!(
    min_count(), 
    max_count(), 
    in_component(),
    datatype(), 
    node_kind(),
    class(),
    or()
  )
}

fn property_shape<'a, RDF>(state: &'a mut State) -> impl RDFNodeParse<RDF, Output = PropertyShape> + 'a
where
    RDF: FocusRDF + 'a,
{
    optional(has_type(SH_PROPERTY_SHAPE.clone()))
        .with(id().and(path()).then(move |(id, path)| {
            ok(&PropertyShape::new(id, path))
        }))
        .then(|ps| targets().flat_map(move |ts| Ok(ps.clone().with_targets(ts))))
        .then(|ps| {
            property_shapes().flat_map(move |prop_shapes| 
                Ok(ps.clone().with_property_shapes(prop_shapes)
            ))
        }).then(move |ps| 
            property_shape_components(ps))
}

fn property_shape_components<RDF>(ps: PropertyShape) -> impl RDFNodeParse<RDF, Output = PropertyShape> 
  where RDF: FocusRDF
  {
    components().flat_map(move |cs| 
        Ok(ps.clone().with_components(cs))
    )
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
        .then(|ns| {
            property_shapes().flat_map(move |ps| Ok(ns.clone().with_property_shapes(ps)))
        })
}

fn property_shapes<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<RDFNode>> {
    property_values(&SH_PROPERTY).flat_map(|ts| {
      let nodes = ts.iter().map(|t| RDF::term_as_object(t)).collect();
      Ok(nodes)
    })
  }



fn parse_or_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    rdf_list().flat_map(|ls| cnv_or_list::<RDF>(ls))
}

fn cnv_or_list<RDF>(ls: Vec<RDF::Term>) -> PResult<Component>
where RDF: SRDFBasic {
    let shapes: Vec<_> = ls.iter().map(RDF::term_as_object).collect();
    Ok(Component::Or { shapes })
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
where RDF: FocusRDF {
    property_value(&SH_PATH).then(shacl_path)
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


fn targets<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target>>
where
    RDF: FocusRDF,
{
    combine_vec(targets_class(), targets_node())
}


fn min_count<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
   property_values_int(&SH_MIN_COUNT).map(|ns| {
     ns.iter().map(|n| Component::MinCount(n.clone())).collect()
   })
}

fn max_count<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
   property_values_int(&SH_MAX_COUNT).map(|ns| {
     ns.iter().map(|n| Component::MaxCount(n.clone())).collect()
   })
}

fn datatype<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
   property_values_iri(&SH_DATATYPE).map(|ns| {
     ns.iter().map(|iri| Component::Datatype(IriRef::iri(iri.clone()))).collect()
   })
}

fn class<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
   property_values(&SH_CLASS).map(|ns| {
     ns.iter().map(|term| Component::Class(RDF::term_as_object(term))).collect()
   })
}

fn node_kind<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
   property_values(&SH_NODE_KIND).flat_map(|ns| {
     let nks: Vec<_> = ns.iter().flat_map(|term| {
        let nk = term_to_node_kind::<RDF>(term)?;
        Ok::<Component, ShaclParserError>(Component::NodeKind(nk))
     }).collect();
     Ok(nks)
   })
}

fn in_component<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values(&SH_IN).then(move |node_set| {
        let nodes: Vec<_> = node_set.into_iter().collect();
        parse_nodes(nodes, parse_in_values())
    })
}

fn parse_in_values<RDF>() -> impl RDFNodeParse<RDF, Output = Component> 
where RDF: FocusRDF {
   rdf_list().flat_map(cnv_in_list::<RDF>)
}

fn cnv_in_list<RDF>(ls: Vec<RDF::Term>) -> std::result::Result<Component, RDFParseError>
where RDF: SRDFBasic {
  let values = ls.iter().flat_map(term_to_value::<RDF>).collect();
  Ok(Component::In { values })
}

fn or<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> 
where RDF: FocusRDF {
    property_values(&SH_OR).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        let nodes: Vec<_> = terms.iter().map(|term| RDF::term_as_object(term)).collect();
        /*for node in nodes {
            // TODO...check that it doesn't appear in hashset
            state.pending.push(node)
        }*/
        parse_nodes(terms, parse_or_values())
    })
}


fn term_to_value<RDF>(term: &RDF::Term) -> std::result::Result<Value, RDFParseError> 
where RDF: SRDFBasic {
   let object = RDF::term_as_object(&term);
   match object {
    Object::Iri { iri } => Ok(Value::iri(iri)),
    Object::BlankNode(_) => Err(RDFParseError::UnexpectedBNode { term: format!("{term}")}),
    Object::Literal(lit) => Ok(Value::Literal(lit)),
  }
}

fn term_to_node_kind<RDF>(term: &RDF::Term) -> Result<NodeKind> 
where RDF: SRDFBasic {
    match RDF::term_as_iri(&term) {
        Some(iri) => {
           let iri_s = RDF::iri2iri_s(&iri);
           match iri_s.as_str() {
             SH_IRI_STR => Ok(NodeKind::Iri),
             SH_LITERAL_STR => Ok(NodeKind::Literal),
             SH_BLANKNODE_STR => Ok(NodeKind::BlankNode),
             SH_BLANK_NODE_OR_IRI_STR => Ok(NodeKind::BlankNodeOrIri),
             SH_BLANK_NODE_OR_LITERAL_STR => Ok(NodeKind::BlankNodeOrLiteral),
             SH_IRI_OR_LITERAL_STR => Ok(NodeKind::IRIOrLiteral),
             _ => Err(ShaclParserError::UnknownNodeKind { term: format!("{term}")})
           }
        },
        None => Err(ShaclParserError::ExpectedNodeKind { term: format!("{term}")})
    }
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

