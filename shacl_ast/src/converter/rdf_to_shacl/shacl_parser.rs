use prefixmap::{IriRef, PrefixMap};
use srdf::{
    combine_parsers, combine_vec, has_type, not, ok, optional, parse_nodes, property_bool,
    property_value, property_values, property_values_int, property_values_iri,
    property_values_non_empty, rdf_list, term, FocusRDF, Object, PResult, RDFNode, RDFNodeParse,
    RDFParseError, RDFParser, SHACLPath, SRDFBasic, Triple, RDF_TYPE,
};
use std::collections::{HashMap, HashSet};

use crate::{
    component::Component, node_kind::NodeKind, node_shape::NodeShape,
    property_shape::PropertyShape, schema::Schema, shape::Shape, target::Target, value::Value, *,
};
use std::fmt::Debug;

use super::shacl_parser_error::ShaclParserError;

type Result<A> = std::result::Result<A, ShaclParserError>;

struct State {
    pending: Vec<RDFNode>,
}

impl State {
    fn from(pending: Vec<RDFNode>) -> Self {
        State { pending }
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
        let prefixmap: PrefixMap = self.rdf_parser.prefixmap().unwrap_or_default();

        let mut state = State::from(self.shapes_candidates()?);
        while let Some(node) = state.pop_pending() {
            if let std::collections::hash_map::Entry::Vacant(e) = self.shapes.entry(node.clone()) {
                let term = RDF::object_as_term(&node);
                self.rdf_parser.rdf.set_focus(&term);
                let shape = Self::shape(&mut state)
                    .parse_impl(&mut self.rdf_parser.rdf)
                    .map_err(|e| ShaclParserError::RDFParseError { err: e })?;
                e.insert(shape);
            }
        }

        Ok(Schema::new()
            .with_prefixmap(prefixmap)
            .with_shapes(self.shapes.clone()))
    }

    fn shapes_candidates(&mut self) -> Result<Vec<RDFNode>> {
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

        // elements of `sh:or` list
        let sh_or_values = self.get_sh_or_values()?;

        // elements of `sh:xone` list
        let sh_xone_values = self.get_sh_xone_values()?;

        // elements of `sh:and` list
        let sh_and_values = self.get_sh_and_values()?;

        // elements of `sh:not` list
        let sh_not_values = self.get_sh_not_values()?;

        // elements of `sh:not` list
        let sh_node_values = self.get_sh_node_values()?;

        // TODO: subjects with type `sh:PropertyShape`
        let property_shapes_instances = HashSet::new();

        // TODO: subjects with type `sh:Shape`
        let shape_instances = HashSet::new();

        // I would prefer a code like: node_shape_instances.union(subjects_property).union(...)
        // But looking to the union API in HashSet, I think it can't be chained
        let mut candidates = HashSet::new();
        candidates.extend(node_shape_instances);
        candidates.extend(subjects_property);
        candidates.extend(sh_or_values);
        candidates.extend(sh_xone_values);
        candidates.extend(sh_and_values);
        candidates.extend(sh_not_values);
        candidates.extend(sh_node_values);
        candidates.extend(property_shapes_instances);
        candidates.extend(shape_instances);

        let result: Vec<_> = candidates
            .iter()
            .map(|s| Self::subject_to_node(s))
            .collect();
        Ok(result)
    }

    fn get_sh_or_values(&mut self) -> Result<HashSet<RDF::Subject>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_or())? {
            let term = RDF::subject_as_term(&s);
            self.rdf_parser.set_focus(&term);
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Some(subj) = RDF::term_as_subject(&v) {
                    rs.insert(subj);
                } else {
                    return Err(ShaclParserError::OrValueNoSubject {
                        term: format!("{v}"),
                    });
                }
            }
        }
        Ok(rs)
    }

    fn get_sh_xone_values(&mut self) -> Result<HashSet<RDF::Subject>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_xone())? {
            let term = RDF::subject_as_term(&s);
            self.rdf_parser.set_focus(&term);
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Some(subj) = RDF::term_as_subject(&v) {
                    rs.insert(subj);
                } else {
                    return Err(ShaclParserError::XOneValueNoSubject {
                        term: format!("{v}"),
                    });
                }
            }
        }
        Ok(rs)
    }

    fn get_sh_and_values(&mut self) -> Result<HashSet<RDF::Subject>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_and())? {
            let term = RDF::subject_as_term(&s);
            self.rdf_parser.set_focus(&term);
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Some(subj) = RDF::term_as_subject(&v) {
                    rs.insert(subj);
                } else {
                    return Err(ShaclParserError::AndValueNoSubject {
                        term: format!("{v}"),
                    });
                }
            }
        }
        Ok(rs)
    }

    fn get_sh_not_values(&mut self) -> Result<HashSet<RDF::Subject>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_not())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn get_sh_node_values(&mut self) -> Result<HashSet<RDF::Subject>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_node())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn objects_with_predicate(&self, pred: RDF::IRI) -> Result<HashSet<RDF::Subject>> {
        let triples = self
            .rdf_parser
            .rdf
            .triples_with_predicate(&pred)
            .map_err(|e| ShaclParserError::Custom {
                msg: format!("Error obtaining values with predicate sh:property: {e}"),
            })?;
        let values_as_subjects = triples
            .iter()
            .flat_map(Self::triple_object_as_subject)
            .collect();
        Ok(values_as_subjects)
    }

    /*fn values_of_list(&mut self, term: RDF::Term) -> Result<Vec<RDF::Term>> {
        let values = set_focus(&term).with(rdf_list()).parse_impl(&mut self.rdf_parser.rdf)?;
        Ok(values)
    }*/

    fn rdf_type() -> RDF::IRI {
        RDF::iri_s2iri(&RDF_TYPE)
    }

    fn sh_node_shape() -> RDF::Term {
        RDF::iri_s2term(&SH_NODE_SHAPE)
    }

    fn sh_property() -> RDF::IRI {
        RDF::iri_s2iri(&SH_PROPERTY)
    }

    fn sh_or() -> RDF::IRI {
        RDF::iri_s2iri(&SH_OR)
    }

    fn sh_xone() -> RDF::IRI {
        RDF::iri_s2iri(&SH_XONE)
    }

    fn sh_and() -> RDF::IRI {
        RDF::iri_s2iri(&SH_AND)
    }

    fn sh_not() -> RDF::IRI {
        RDF::iri_s2iri(&SH_NOT)
    }

    fn sh_node() -> RDF::IRI {
        RDF::iri_s2iri(&SH_NODE)
    }

    fn triple_object_as_subject(triple: &Triple<RDF>) -> Result<RDF::Subject> {
        let subj = RDF::term_as_subject(&triple.obj()).ok_or_else(|| ShaclParserError::Custom {
            msg: format!("Expected triple object value to act as a subject: {triple}"),
        })?;
        Ok(subj)
    }

    fn subject_to_node(subject: &RDF::Subject) -> RDFNode {
        RDF::subject_as_object(subject)
    }

    fn shape<'a>(state: &'a mut State) -> impl RDFNodeParse<RDF, Output = Shape> + 'a
    where
        RDF: FocusRDF + 'a,
    {
        node_shape()
            .then(move |ns| ok(&Shape::NodeShape(Box::new(ns))))
            .or(property_shape(state).then(|ps| ok(&Shape::PropertyShape(ps))))
    }
}

fn components<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    combine_parsers!(
        min_count(),
        max_count(),
        in_component(),
        datatype(),
        node_kind(),
        class(),
        or(),
        xone(),
        and(),
        not_parser(),
        node(),
        min_length(),
        max_length(),
        has_value()
    )
}

fn property_shape<'a, RDF>(
    _state: &'a mut State,
) -> impl RDFNodeParse<RDF, Output = PropertyShape> + 'a
where
    RDF: FocusRDF + 'a,
{
    optional(has_type(SH_PROPERTY_SHAPE.clone()))
        .with(
            id().and(path())
                .then(move |(id, path)| ok(&PropertyShape::new(id, path))),
        )
        .then(|ps| targets().flat_map(move |ts| Ok(ps.clone().with_targets(ts))))
        .then(|ps| {
            optional(closed()).flat_map(move |c| {
                if let Some(true) = c {
                    Ok(ps.clone().with_closed(true))
                } else {
                    Ok(ps.clone())
                }
            })
        })
        .then(|ps| {
            property_shapes()
                .flat_map(move |prop_shapes| Ok(ps.clone().with_property_shapes(prop_shapes)))
        })
        .then(move |ps| property_shape_components(ps))
}

fn property_shape_components<RDF>(
    ps: PropertyShape,
) -> impl RDFNodeParse<RDF, Output = PropertyShape>
where
    RDF: FocusRDF,
{
    components().flat_map(move |cs| Ok(ps.clone().with_components(cs)))
}

fn node_shape<RDF>() -> impl RDFNodeParse<RDF, Output = NodeShape>
where
    RDF: FocusRDF,
{
    not(property_values_non_empty(&SH_PATH)).with(
        term()
            .then(move |t: RDF::Term| {
                let id = RDF::term_as_object(&t.clone());
                ok(&NodeShape::new(id))
            })
            .then(|ns| targets().flat_map(move |ts| Ok(ns.clone().with_targets(ts))))
            .then(|ps| {
                optional(closed()).flat_map(move |c| {
                    if let Some(true) = c {
                        Ok(ps.clone().with_closed(true))
                    } else {
                        Ok(ps.clone())
                    }
                })
            })
            .then(|ns| {
                property_shapes().flat_map(move |ps| Ok(ns.clone().with_property_shapes(ps)))
            })
            .then(|ns| components().flat_map(move |cs| Ok(ns.clone().with_components(cs)))),
    )
}

fn property_shapes<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<RDFNode>> {
    property_values(&SH_PROPERTY).flat_map(|ts| {
        let nodes = ts.iter().map(|t| RDF::term_as_object(t)).collect();
        Ok(nodes)
    })
}

fn parse_xone_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    rdf_list().flat_map(|ls| cnv_xone_list::<RDF>(ls))
}

fn cnv_xone_list<RDF>(ls: Vec<RDF::Term>) -> PResult<Component>
where
    RDF: SRDFBasic,
{
    let shapes: Vec<_> = ls.iter().map(RDF::term_as_object).collect();
    Ok(Component::Xone { shapes })
}

fn parse_and_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    rdf_list().flat_map(|ls| cnv_and_list::<RDF>(ls))
}

fn cnv_and_list<RDF>(ls: Vec<RDF::Term>) -> PResult<Component>
where
    RDF: SRDFBasic,
{
    let shapes: Vec<_> = ls.iter().map(RDF::term_as_object).collect();
    Ok(Component::And { shapes })
}

fn parse_not_value<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    term().flat_map(|t| cnv_not::<RDF>(t))
}

fn parse_node_value<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    term().flat_map(|t| cnv_node::<RDF>(t))
}

fn cnv_node<RDF>(t: RDF::Term) -> PResult<Component>
where
    RDF: SRDFBasic,
{
    let shape = RDF::term_as_object(&t);
    Ok(Component::Node { shape })
}

fn cnv_not<RDF>(t: RDF::Term) -> PResult<Component>
where
    RDF: SRDFBasic,
{
    let shape = RDF::term_as_object(&t);
    Ok(Component::Not { shape })
}

fn parse_or_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    rdf_list().flat_map(|ls| cnv_or_list::<RDF>(ls))
}

fn cnv_or_list<RDF>(ls: Vec<RDF::Term>) -> PResult<Component>
where
    RDF: SRDFBasic,
{
    let shapes: Vec<_> = ls.iter().map(RDF::term_as_object).collect();
    Ok(Component::Or { shapes })
}

fn id<RDF>() -> impl RDFNodeParse<RDF, Output = RDFNode>
where
    RDF: FocusRDF,
{
    term().then(move |t: RDF::Term| {
        let id = RDF::term_as_object(&t.clone());
        ok(&id)
    })
}

/// Parses the property value of the focus node as a SHACL path
fn path<RDF>() -> impl RDFNodeParse<RDF, Output = SHACLPath>
where
    RDF: FocusRDF,
{
    property_value(&SH_PATH).then(shacl_path)
}

/// Parses the current focus node as a SHACL path
fn shacl_path<RDF>(term: RDF::Term) -> impl RDFNodeParse<RDF, Output = SHACLPath>
where
    RDF: FocusRDF,
{
    let obj = RDF::term_as_object(&term);
    match obj {
        Object::Iri(iri) => ok(&SHACLPath::iri(iri)),
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

fn closed<RDF>() -> impl RDFNodeParse<RDF, Output = bool>
where
    RDF: FocusRDF,
{
    property_bool(&SH_CLOSED)
}

fn min_count<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_int(&SH_MIN_COUNT)
        .map(|ns| ns.iter().map(|n| Component::MinCount(*n)).collect())
}

fn max_count<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_int(&SH_MAX_COUNT)
        .map(|ns| ns.iter().map(|n| Component::MaxCount(*n)).collect())
}

fn min_length<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_int(&SH_MIN_LENGTH)
        .map(|ns| ns.iter().map(|n| Component::MinLength(*n)).collect())
}

fn max_length<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_int(&SH_MAX_LENGTH)
        .map(|ns| ns.iter().map(|n| Component::MaxLength(*n)).collect())
}

fn datatype<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_iri(&SH_DATATYPE).map(|ns| {
        ns.iter()
            .map(|iri| Component::Datatype(IriRef::iri(iri.clone())))
            .collect()
    })
}

fn class<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values(&SH_CLASS).map(|ns| {
        ns.iter()
            .map(|term| Component::Class(RDF::term_as_object(term)))
            .collect()
    })
}

fn node_kind<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values(&SH_NODE_KIND).flat_map(|ns| {
        let nks: Vec<_> = ns
            .iter()
            .flat_map(|term| {
                let nk = term_to_node_kind::<RDF>(term)?;
                Ok::<Component, ShaclParserError>(Component::NodeKind(nk))
            })
            .collect();
        Ok(nks)
    })
}

fn has_value<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values(&SH_HAS_VALUE).then(move |node_set| {
        let nodes: Vec<_> = node_set.into_iter().collect();
        parse_nodes(nodes, parse_has_value_values())
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
where
    RDF: FocusRDF,
{
    rdf_list().flat_map(cnv_in_list::<RDF>)
}

fn parse_has_value_values<RDF>() -> impl RDFNodeParse<RDF, Output = Component>
where
    RDF: FocusRDF,
{
    term().flat_map(cnv_has_value::<RDF>)
}

fn cnv_has_value<RDF>(term: RDF::Term) -> std::result::Result<Component, RDFParseError>
where
    RDF: SRDFBasic,
{
    let value = term_to_value::<RDF>(&term)?;
    Ok(Component::HasValue { value })
}

fn term_to_value<RDF>(term: &RDF::Term) -> std::result::Result<Value, RDFParseError>
where
    RDF: SRDFBasic,
{
    match RDF::term_as_object(term) {
        Object::Iri(iri) => Ok(Value::Iri(IriRef::Iri(iri))),
        Object::BlankNode(bn) => Err(RDFParseError::BlankNodeNoValue {
            bnode: bn.to_string(),
        }),
        Object::Literal(lit) => Ok(Value::Literal(lit)),
    }
}

fn cnv_in_list<RDF>(ls: Vec<RDF::Term>) -> std::result::Result<Component, RDFParseError>
where
    RDF: SRDFBasic,
{
    let values = ls.iter().flat_map(term_to_value::<RDF>).collect();
    Ok(Component::In { values })
}

fn or<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values(&SH_OR).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_or_values())
    })
}

fn xone<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values(&SH_XONE).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_xone_values())
    })
}

fn and<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values(&SH_AND).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_and_values())
    })
}

fn not_parser<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values(&SH_NOT).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_not_value())
    })
}

fn node<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values(&SH_NODE).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_node_value())
    })
}

fn term_to_node_kind<RDF>(term: &RDF::Term) -> Result<NodeKind>
where
    RDF: SRDFBasic,
{
    match RDF::term_as_iri(term) {
        Some(iri) => {
            let iri_s = RDF::iri2iri_s(&iri);
            match iri_s.as_str() {
                SH_IRI_STR => Ok(NodeKind::Iri),
                SH_LITERAL_STR => Ok(NodeKind::Literal),
                SH_BLANKNODE_STR => Ok(NodeKind::BlankNode),
                SH_BLANK_NODE_OR_IRI_STR => Ok(NodeKind::BlankNodeOrIri),
                SH_BLANK_NODE_OR_LITERAL_STR => Ok(NodeKind::BlankNodeOrLiteral),
                SH_IRI_OR_LITERAL_STR => Ok(NodeKind::IRIOrLiteral),
                _ => Err(ShaclParserError::UnknownNodeKind {
                    term: format!("{term}"),
                }),
            }
        }
        None => Err(ShaclParserError::ExpectedNodeKind {
            term: format!("{term}"),
        }),
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
                let node = RDF::term_as_object(t);
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
                let node = RDF::term_as_object(t);
                Target::TargetNode(node)
            })
            .collect();
        Ok(result)
    })
}
