use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

use component::And;
use component::Class;
use component::Component;
use component::Datatype;
use component::HasValue;
use component::In;
use component::MaxCount;
use component::MaxLength;
use component::MinCount;
use component::MinLength;
use component::Node;
use component::Nodekind;
use component::Not;
use component::Or;
use component::Xone;
use model::focus_rdf::FocusRdf;
use model::rdf::Rdf;
use model::rdf::TObjectRef;
use model::rdf::TPredicateRef;
use model::rdf::TSubjectRef;
use model::Term;
use model::Triple;
use node_kind::NodeKind;
use node_shape::NodeShape;
use prefixmap::PrefixMap;
use property_shape::PropertyShape;
use shacl_path::SHACLPath;
use shape::Shape;
use srdf::model::Iri;
use srdf::*;
use target::Target;

use crate::vocab::*;
use crate::*;

use super::shacl_parser_error::ShaclParserError;

type Result<A> = std::result::Result<A, ShaclParserError>;

struct State<R: Rdf> {
    pending: Vec<TObjectRef<R>>,
}

impl<R: Rdf> State<R> {
    fn from(pending: Vec<TObjectRef<R>>) -> Self {
        State { pending }
    }

    fn pop_pending(&mut self) -> Option<TObjectRef<R>> {
        self.pending.pop()
    }
}

pub struct ShaclParser<R: FocusRdf> {
    rdf_parser: RDFParser<R>,
    shapes: HashMap<TObjectRef<R>, Shape<R>>,
}

impl<R: FocusRdf + Clone + Default> ShaclParser<R> {
    pub fn new(rdf: R) -> ShaclParser<R> {
        ShaclParser {
            rdf_parser: RDFParser::new(rdf),
            shapes: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Schema<R>> {
        let prefixmap: PrefixMap = self.rdf_parser.prefixmap().unwrap_or_default();
        let mut state = State::from(self.shapes_candidates()?);

        while let Some(node) = state.pop_pending() {
            let node: TObjectRef<R> = node;
            if let Entry::Vacant(e) = self.shapes.entry(node.clone()) {
                self.rdf_parser.rdf.set_focus(node.clone());
                let shape = Self::shape(&mut state)
                    .parse_impl(&mut self.rdf_parser.rdf)
                    .map_err(|e| ShaclParserError::RdfParseError { err: e })?;
                e.insert(shape);
            }
        }

        Ok(Schema::default()
            .with_prefixmap(prefixmap)
            .with_shapes(self.shapes.clone()))
    }

    fn shapes_candidates(&mut self) -> Result<Vec<TObjectRef<R>>> {
        // subjects with type `sh:NodeShape`
        let rdf_type = Self::rdf_type();
        let node_shape = Self::sh_node_shape().into();

        let node_shape_instances =
            match self
                .rdf_parser
                .rdf
                .triples_matching(None, Some(&rdf_type), Some(&node_shape))
            {
                Ok(triples) => triples.map(Triple::subject).collect::<HashSet<_>>(),
                Err(e) => {
                    return Err(ShaclParserError::Custom {
                        msg: format!("Error obtaining values with type sh:NodeShape: {e}"),
                    })
                }
            };

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

        // TODO: subjects with type `sh:PropertyShape<R>`
        let property_shapes_instances = HashSet::new();

        // TODO: subjects with type `sh:Shape<R>`
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

    fn get_sh_or_values(&mut self) -> Result<HashSet<TSubjectRef<R>>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_or())? {
            self.rdf_parser.set_focus(&s.into());
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                match v.clone().try_into() {
                    Ok(subj) => {
                        rs.insert(subj);
                    }
                    Err(_) => {
                        return Err(ShaclParserError::OrValueNoSubject {
                            term: format!("{v}"),
                        })
                    }
                }
            }
        }
        Ok(rs)
    }

    fn get_sh_xone_values(&mut self) -> Result<HashSet<TSubjectRef<R>>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_xone())? {
            self.rdf_parser.set_focus(&s.into());
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                match v.clone().try_into() {
                    Ok(subj) => {
                        rs.insert(subj);
                    }
                    Err(_) => {
                        return Err(ShaclParserError::XOneValueNoSubject {
                            term: format!("{v}"),
                        })
                    }
                }
            }
        }
        Ok(rs)
    }

    fn get_sh_and_values(&mut self) -> Result<HashSet<TSubjectRef<R>>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_and())? {
            self.rdf_parser.set_focus(&s.into());
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                match v.clone().try_into() {
                    Ok(subj) => {
                        rs.insert(subj);
                    }
                    Err(_) => {
                        return Err(ShaclParserError::AndValueNoSubject {
                            term: format!("{v}"),
                        })
                    }
                }
            }
        }
        Ok(rs)
    }

    fn get_sh_not_values(&mut self) -> Result<HashSet<TSubjectRef<R>>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_not())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn get_sh_node_values(&mut self) -> Result<HashSet<TSubjectRef<R>>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_node())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn objects_with_predicate(&self, pred: TPredicateRef<R>) -> Result<HashSet<TSubjectRef<R>>> {
        let triples = self
            .rdf_parser
            .rdf
            .triples_with_predicate(&pred)
            .map_err(|e| ShaclParserError::Custom {
                msg: format!("Error obtaining values with predicate sh:property: {e}"),
            })?;
        let values_as_subjects = triples.flat_map(Self::triple_object_as_subject).collect();
        Ok(values_as_subjects)
    }

    fn rdf_type() -> TPredicateRef<R> {
        TPredicateRef::<R>::new(RDF_TYPE_STR)
    }

    fn sh_node_shape() -> TPredicateRef<R> {
        TPredicateRef::<R>::new(SH_NODE_SHAPE_STR)
    }

    fn sh_property() -> TPredicateRef<R> {
        TPredicateRef::<R>::new(SH_PROPERTY_STR)
    }

    fn sh_or() -> TPredicateRef<R> {
        TPredicateRef::<R>::new(SH_OR_STR)
    }

    fn sh_xone() -> TPredicateRef<R> {
        TPredicateRef::<R>::new(SH_XONE_STR)
    }

    fn sh_and() -> TPredicateRef<R> {
        TPredicateRef::<R>::new(SH_AND_STR)
    }

    fn sh_not() -> TPredicateRef<R> {
        TPredicateRef::<R>::new(SH_NOT_STR)
    }

    fn sh_node() -> TPredicateRef<R> {
        TPredicateRef::<R>::new(SH_NODE_STR)
    }

    fn triple_object_as_subject(triple: &R::Triple) -> Result<TSubjectRef<R>> {
        match triple.object().clone().try_into() {
            Ok(obj) => Ok(obj),
            Err(_) => Err(ShaclParserError::Custom {
                msg: format!("Expected triple object value to act as a subject: {triple}"),
            }),
        }
    }

    fn subject_to_node(subject: &TSubjectRef<R>) -> TObjectRef<R> {
        subject.clone().into()
    }

    fn shape<'a>(state: &'a mut State<R>) -> impl RDFNodeParse<R, Output = Shape<R>> + 'a {
        node_shape()
            .then(move |ns| ok(&Shape::NodeShape(*Box::new(ns))))
            .or(property_shape(state).then(|ps| ok(&Shape::PropertyShape(ps))))
    }
}

fn components<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
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

fn property_shape<'a, R: FocusRdf + Clone>(
    _state: &'a mut State<R>,
) -> impl RDFNodeParse<R, Output = PropertyShape<R>> + 'a {
    optional(has_type(SH_PROPERTY_SHAPE.clone()))
        .with(
            id().and(path())
                .then(|(id, path)| ok(&PropertyShape::new(id, path))),
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

fn property_shape_components<R: FocusRdf + Clone>(
    ps: PropertyShape<R>,
) -> impl RDFNodeParse<R, Output = PropertyShape<R>> {
    components().flat_map(move |cs| Ok(ps.clone().with_components(cs)))
}

fn node_shape<R: FocusRdf + Clone>() -> impl RDFNodeParse<R, Output = NodeShape<R>> {
    not(property_values_non_empty(&SH_PATH)).with(
        term()
            .then(move |t: TObjectRef<R>| ok(&NodeShape::new(t)))
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

fn property_shapes<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<TObjectRef<R>>> {
    property_values(&SH_PROPERTY).flat_map(|ts| {
        let nodes: Vec<_> = ts.into_iter().collect();
        Ok(nodes)
    })
}

fn parse_xone_values<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component<R>> {
    rdf_list().flat_map(|ls| cnv_xone_list::<R>(ls))
}

fn cnv_xone_list<R: Rdf>(ls: Vec<TObjectRef<R>>) -> ParserResult<Component<R>> {
    Ok(Component::Xone(Xone::new(ls)))
}

fn parse_and_values<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component<R>> {
    rdf_list().flat_map(|ls| cnv_and_list::<R>(ls))
}

fn cnv_and_list<R: Rdf>(ls: Vec<TObjectRef<R>>) -> ParserResult<Component<R>> {
    Ok(Component::And(And::new(ls)))
}

fn parse_not_value<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component<R>> {
    term().flat_map(|t| cnv_not::<R>(t))
}

fn parse_node_value<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component<R>> {
    term().flat_map(|t| cnv_node::<R>(t))
}

fn cnv_node<R: Rdf>(t: TObjectRef<R>) -> ParserResult<Component<R>> {
    Ok(Component::Node(Node::new(t)))
}

fn cnv_not<R: Rdf>(t: TObjectRef<R>) -> ParserResult<Component<R>> {
    Ok(Component::Not(Not::new(t)))
}

fn parse_or_values<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component<R>> {
    rdf_list().flat_map(|ls| cnv_or_list::<R>(ls))
}

fn cnv_or_list<R: Rdf>(ls: Vec<TObjectRef<R>>) -> ParserResult<Component<R>> {
    Ok(Component::Or(Or::new(ls)))
}

fn id<R: FocusRdf>() -> impl RDFNodeParse<R, Output = TObjectRef<R>> {
    term().then(|t: TObjectRef<R>| ok(&t))
}

/// Parses the property value of the focus node as a SHACL path
fn path<R: FocusRdf>() -> impl RDFNodeParse<R, Output = SHACLPath<R::Triple>> {
    property_value(&SH_PATH).then(shacl_path)
}

/// Parses the current focus node as a SHACL path
fn shacl_path<R: FocusRdf>(
    term: TObjectRef<R>,
) -> impl RDFNodeParse<R, Output = SHACLPath<R::Triple>> {
    match term.as_iri() {
        Some(iri) => ok(&SHACLPath::iri(iri.clone())),
        None => todo!(),
    }
}

fn targets<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Target<R>>> {
    combine_vec(targets_class(), targets_node())
}

fn closed<R: FocusRdf>() -> impl RDFNodeParse<R, Output = bool> {
    property_bool(&SH_CLOSED)
}

fn min_count<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values_int(&SH_MIN_COUNT).map(|ns| {
        ns.iter()
            .map(|n| Component::MinCount(MinCount::new(*n)))
            .collect()
    })
}

fn max_count<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values_int(&SH_MAX_COUNT).map(|ns| {
        ns.iter()
            .map(|n| Component::MaxCount(MaxCount::new(*n)))
            .collect()
    })
}

fn min_length<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values_int(&SH_MIN_LENGTH).map(|ns| {
        ns.iter()
            .map(|n| Component::MinLength(MinLength::new(*n)))
            .collect()
    })
}

fn max_length<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values_int(&SH_MAX_LENGTH).map(|ns| {
        ns.iter()
            .map(|n| Component::MaxLength(MaxLength::new(*n)))
            .collect()
    })
}

fn datatype<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values_iri(&SH_DATATYPE).map(|ns| {
        ns.iter()
            .map(|iri| Component::Datatype(Datatype::new(TPredicateRef::<R>::new(iri.as_str()))))
            .collect()
    })
}

fn class<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values(&SH_CLASS).map(|ns| {
        ns.iter()
            .map(|term: &TObjectRef<R>| Component::Class(Class::new(term.clone())))
            .collect()
    })
}

fn node_kind<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values(&SH_NODE_KIND).flat_map(|ns| {
        let nks: Vec<_> = ns
            .iter()
            .flat_map(|term| {
                let nk = term_to_node_kind::<R>(term)?;
                Ok::<Component<R>, ShaclParserError>(Component::NodeKind(Nodekind::new(nk)))
            })
            .collect();
        Ok(nks)
    })
}

fn has_value<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values(&SH_HAS_VALUE).then(move |node_set| {
        let nodes: Vec<_> = node_set.into_iter().collect();
        parse_nodes(nodes, parse_has_value_values())
    })
}

fn in_component<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values(&SH_IN).then(move |node_set| {
        let nodes: Vec<_> = node_set.into_iter().collect();
        parse_nodes(nodes, parse_in_values())
    })
}

fn parse_in_values<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component<R>> {
    rdf_list().flat_map(cnv_in_list::<R>)
}

fn parse_has_value_values<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component<R>> {
    term().flat_map(cnv_has_value::<R>)
}

fn cnv_has_value<R: Rdf>(term: TObjectRef<R>) -> std::result::Result<Component<R>, RdfParseError> {
    let value = term_to_value::<R>(&term)?;
    Ok(Component::HasValue(HasValue::new(value)))
}

fn term_to_value<R: Rdf>(
    term: &TObjectRef<R>,
) -> std::result::Result<TObjectRef<R>, RdfParseError> {
    match (term.is_iri(), term.is_blank_node(), term.is_literal()) {
        (true, false, false) => Ok(term.clone()),
        (false, false, true) => Ok(term.clone()),
        (false, true, false) => Err(RdfParseError::BlankNodeNoValue {
            bnode: term.to_string(),
        }),
        _ => unreachable!(),
    }
}

fn cnv_in_list<R: Rdf>(ls: Vec<TObjectRef<R>>) -> std::result::Result<Component<R>, RdfParseError> {
    let values = ls.iter().flat_map(term_to_value::<R>).collect();
    Ok(Component::In(In::new(values)))
}

fn or<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values(&SH_OR).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_or_values())
    })
}

fn xone<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values(&SH_XONE).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_xone_values())
    })
}

fn and<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values(&SH_AND).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_and_values())
    })
}

fn not_parser<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values(&SH_NOT).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_not_value())
    })
}

fn node<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component<R>>> {
    property_values(&SH_NODE).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_node_value())
    })
}

fn term_to_node_kind<R: Rdf>(term: &TObjectRef<R>) -> Result<NodeKind> {
    match term.as_iri() {
        Some(iri) => match iri.as_iri_s().as_str() {
            SH_IRI_STR => Ok(NodeKind::Iri),
            SH_LITERAL_STR => Ok(NodeKind::Literal),
            SH_BLANKNODE_STR => Ok(NodeKind::BlankNode),
            SH_BLANK_NODE_OR_IRI_STR => Ok(NodeKind::BlankNodeOrIri),
            SH_BLANK_NODE_OR_LITERAL_STR => Ok(NodeKind::BlankNodeOrLiteral),
            SH_IRI_OR_LITERAL_STR => Ok(NodeKind::IRIOrLiteral),
            _ => Err(ShaclParserError::UnknownNodeKind {
                term: format!("{term}"),
            }),
        },
        None => Err(ShaclParserError::ExpectedNodeKind {
            term: format!("{term}"),
        }),
    }
}

fn targets_class<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Target<R>>> {
    property_values(&SH_TARGET_CLASS).flat_map(move |ts| {
        let result = ts
            .iter()
            .map(|t: &TObjectRef<R>| Target::TargetClass(t.clone()))
            .collect();
        Ok(result)
    })
}

fn targets_node<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Target<R>>> {
    property_values(&SH_TARGET_NODE).flat_map(move |ts| {
        let result = ts
            .iter()
            .map(|t: &TObjectRef<R>| Target::TargetNode(t.clone()))
            .collect();
        Ok(result)
    })
}
