use std::collections::HashMap;
use std::collections::HashSet;

use model::focus_rdf::FocusRdf;
use model::rdf::Object;
use model::rdf::Predicate;
use model::rdf::Rdf;
use model::rdf::Subject;
use prefixmap::IriRef;
use prefixmap::PrefixMap;
use shape::Shape;
use srdf::*;

use crate::vocab::*;
use crate::*;

use super::shacl_parser_error::ShaclParserError;

type Result<A> = std::result::Result<A, ShaclParserError>;

struct State<R: Rdf> {
    pending: Vec<Object<R>>,
}

impl<R: Rdf> State<R> {
    fn from(pending: Vec<Object<R>>) -> Self {
        State { pending }
    }

    fn pop_pending(&mut self) -> Option<Object<R>> {
        self.pending.pop()
    }
}

pub struct ShaclParser<R: FocusRdf> {
    rdf_parser: RDFParser<R>,
    shapes: HashMap<Object<R>, Shape<R>>,
}

impl<R: FocusRdf> ShaclParser<R> {
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
            if let std::collections::hash_map::Entry::Vacant(e) = self.shapes.entry(node.clone()) {
                self.rdf_parser.rdf.set_focus(&node);
                let shape = Self::shape(&mut state)
                    .parse_impl(&mut self.rdf_parser.rdf)
                    .map_err(|e| ShaclParserError::RdfParseError { err: e })?;
                e.insert(shape);
            }
        }

        Ok(
            Schema::default()
                .with_prefixmap(prefixmap)
                .with_shapes(self.shapes.clone())
        )
    }

    fn shapes_candidates(&mut self) -> Result<Vec<Object<R>>> {
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

    fn get_sh_or_values(&mut self) -> Result<HashSet<Subject<R>>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_or())? {
            let term = R::subject_as_term(&s);
            self.rdf_parser.set_focus(&term);
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Some(subj) = R::term_as_subject(&v) {
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

    fn get_sh_xone_values(&mut self) -> Result<HashSet<Subject<R>>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_xone())? {
            let term = R::subject_as_term(&s);
            self.rdf_parser.set_focus(&term);
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Some(subj) = R::term_as_subject(&v) {
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

    fn get_sh_and_values(&mut self) -> Result<HashSet<Subject<R>>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_and())? {
            let term = R::subject_as_term(&s);
            self.rdf_parser.set_focus(&term);
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Some(subj) = R::term_as_subject(&v) {
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

    fn get_sh_not_values(&mut self) -> Result<HashSet<Subject<R>>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_not())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn get_sh_node_values(&mut self) -> Result<HashSet<Subject<R>>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_node())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn objects_with_predicate(&self, pred:Predicate<R>) -> Result<HashSet<Subject<R>>> {
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

    fn rdf_type() -> Predicate<R> {
        Predicate::<R>::new(RDF_TYPE)
    }

    fn sh_node_shape() -> Predicate<R> {
        Predicate::<R>::new(SH_NODE_SHAPE)
    }

    fn sh_property() ->Predicate<R> {
        Predicate::<R>::new(SH_PROPERTY)
    }

    fn sh_or() ->Predicate<R> {
        Predicate::<R>::new(SH_OR)
    }

    fn sh_xone() ->Predicate<R> {
        Predicate::<R>::new(SH_XONE)
    }

    fn sh_and() ->Predicate<R> {
        Predicate::<R>::new(SH_AND)
    }

    fn sh_not() ->Predicate<R> {
        Predicate::<R>::new(SH_NOT)
    }

    fn sh_node() ->Predicate<R> {
        Predicate::<R>::new(SH_NODE)
    }

    fn triple_object_as_subject(triple: &R::Triple) -> Result<Subject<R>> {
        let subj = R::term_as_subject(&triple.obj()).ok_or_else(|| ShaclParserError::Custom {
            msg: format!("Expected triple object value to act as a subject: {triple}"),
        })?;
        Ok(subj)
    }

    fn subject_to_node(subject: &Subject<R>) -> Object<R> {
        R::subject_as_object(subject)
    }

    fn shape<'a>(state: &'a mut State<R>) -> impl RDFNodeParse<R, Output = Shape<R>> + 'a
    {
        node_shape()
            .then(move |ns| ok(&Shape<R>::NodeShape(Box::new(ns))))
            .or(property_shape(state).then(|ps| ok(&Shape<R>::PropertyShape(ps))))
    }
}

fn components<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
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

fn property_shape<'a, R: FocusRdf>(_state: &'a mut State<R>) -> impl RDFNodeParse<R, Output = PropertyShape> + 'a
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

fn property_shape_components<R: FocusRdf>(ps: PropertyShape) -> impl RDFNodeParse<R, Output = PropertyShape>
{
    components().flat_map(move |cs| Ok(ps.clone().with_components(cs)))
}

fn node_shape<R: FocusRdf>() -> impl RDFNodeParse<R, Output = NodeShape>
{
    not(property_values_non_empty(&SH_PATH)).with(
        term()
            .then(move |t: Object<R>| {
                let id = R::term_as_object(&t.clone());
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

fn property_shapes<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Object<R>>> {
    property_values(&SH_PROPERTY).flat_map(|ts| {
        let nodes = ts.iter().map(|t| R::term_as_object(t)).collect();
        Ok(nodes)
    })
}

fn parse_xone_values<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component> {
    rdf_list().flat_map(|ls| cnv_xone_list::<R>(ls))
}

fn cnv_xone_list<R: Rdf>(ls: Vec<Object<R>>) -> ParserResult<Component>
{
    let shapes: Vec<_> = ls.iter().map(R::term_as_object).collect();
    Ok(Component::Xone { shapes })
}

fn parse_and_values<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component> {
    rdf_list().flat_map(|ls| cnv_and_list::<R>(ls))
}

fn cnv_and_list<R: Rdf>(ls: Vec<Object<R>>) -> ParserResult<Component>
{
    let shapes: Vec<_> = ls.iter().map(R::term_as_object).collect();
    Ok(Component::And { shapes })
}

fn parse_not_value<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component> {
    term().flat_map(|t| cnv_not::<R>(t))
}

fn parse_node_value<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component> {
    term().flat_map(|t| cnv_node::<R>(t))
}

fn cnv_node<R: Rdf>(t: Object<R>) -> ParserResult<Component>
{
    let shape = R::term_as_object(&t);
    Ok(Component::Node { shape })
}

fn cnv_not<R: Rdf>(t: Object<R>) -> ParserResult<Component>
{
    let shape = R::term_as_object(&t);
    Ok(Component::Not { shape })
}

fn parse_or_values<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component> {
    rdf_list().flat_map(|ls| cnv_or_list::<R>(ls))
}

fn cnv_or_list<R: Rdf>(ls: Vec<Object<R>>) -> ParserResult<Component>
{
    let shapes: Vec<_> = ls.iter().map(R::term_as_object).collect();
    Ok(Component::Or { shapes })
}

fn id<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Object<R>>
{
    term().then(move |t: Object<R>| {
        let id = R::term_as_object(&t.clone());
        ok(&id)
    })
}

/// Parses the property value of the focus node as a SHACL path
fn path<R: Rdf>() -> impl RDFNodeParse<R, Output = SHACLPath>
{
    property_value(&SH_PATH).then(shacl_path)
}

/// Parses the current focus node as a SHACL path
fn shacl_path<R: Rdf>(term: Object<R>) -> impl RDFNodeParse<R, Output = SHACLPath>
{
    let obj = R::term_as_object(&term);
    match obj {
        Object::Iri(iri) => ok(&SHACLPath::iri(iri)),
        Object::BlankNode(_) => todo!(),
        Object::Literal(_) => todo!(),
    }
}

fn targets<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Target>>
{
    combine_vec(targets_class(), targets_node())
}

fn closed<R: FocusRdf>() -> impl RDFNodeParse<R, Output = bool>
{
    property_bool(&SH_CLOSED)
}

fn min_count<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values_int(&SH_MIN_COUNT)
        .map(|ns| ns.iter().map(|n| Component::MinCount(*n)).collect())
}

fn max_count<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values_int(&SH_MAX_COUNT)
        .map(|ns| ns.iter().map(|n| Component::MaxCount(*n)).collect())
}

fn min_length<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values_int(&SH_MIN_LENGTH)
        .map(|ns| ns.iter().map(|n| Component::MinLength(*n)).collect())
}

fn max_length<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values_int(&SH_MAX_LENGTH)
        .map(|ns| ns.iter().map(|n| Component::MaxLength(*n)).collect())
}

fn datatype<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values_iri(&SH_DATATYPE).map(|ns| {
        ns.iter()
            .map(|iri| Component::Datatype(IriRef::iri(iri.clone())))
            .collect()
    })
}

fn class<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values(&SH_CLASS).map(|ns| {
        ns.iter()
            .map(|term| Component::Class(R::term_as_object(term)))
            .collect()
    })
}

fn node_kind<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values(&SH_NODE_KIND).flat_map(|ns| {
        let nks: Vec<_> = ns
            .iter()
            .flat_map(|term| {
                let nk = term_to_node_kind::<R>(term)?;
                Ok::<Component, ShaclParserError>(Component::NodeKind(nk))
            })
            .collect();
        Ok(nks)
    })
}

fn has_value<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values(&SH_HAS_VALUE).then(move |node_set| {
        let nodes: Vec<_> = node_set.into_iter().collect();
        parse_nodes(nodes, parse_has_value_values())
    })
}

fn in_component<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values(&SH_IN).then(move |node_set| {
        let nodes: Vec<_> = node_set.into_iter().collect();
        parse_nodes(nodes, parse_in_values())
    })
}

fn parse_in_values<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component>
{
    rdf_list().flat_map(cnv_in_list::<R>)
}

fn parse_has_value_values<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Component>
{
    term().flat_map(cnv_has_value::<R>)
}

fn cnv_has_value<R: Rdf>(term: Object<R>) -> std::result::Result<Component, RdfParseError>
{
    let value = term_to_value::<R>(&term)?;
    Ok(Component::HasValue { value })
}

fn term_to_value<R: Rdf>(term: &Object<R>) -> std::result::Result<Value, RdfParseError>
{
    match R::term_as_object(term) {
        Object::Iri(iri) => Ok(Value::Iri(IriRef::Iri(iri))),
        Object::BlankNode(bn) => Err(RdfParseError::BlankNodeNoValue {
            bnode: bn.to_string(),
        }),
        Object::Literal(lit) => Ok(Value::Literal(lit)),
    }
}

fn cnv_in_list<R: Rdf>(ls: Vec<Object<R>>) -> std::result::Result<Component, RdfParseError>
{
    let values = ls.iter().flat_map(term_to_value::<R>).collect();
    Ok(Component::In { values })
}

fn or<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values(&SH_OR).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_or_values())
    })
}

fn xone<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values(&SH_XONE).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_xone_values())
    })
}

fn and<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values(&SH_AND).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_and_values())
    })
}

fn not_parser<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values(&SH_NOT).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_not_value())
    })
}

fn node<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Component>>
{
    property_values(&SH_NODE).then(move |terms_set| {
        let terms: Vec<_> = terms_set.into_iter().collect();
        parse_nodes(terms, parse_node_value())
    })
}

fn term_to_node_kind<R: Rdf>(term: &Object<R>) -> Result<NodeKind>
{
    match R::term_as_iri(term) {
        Some(iri) => {
            let iri_s = R::iri2iri_s(iri);
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

fn targets_class<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Target>>
{
    property_values(&SH_TARGET_CLASS).flat_map(move |ts| {
        let result = ts
            .iter()
            .map(|t| {
                let node = R::term_as_object(t);
                Target::TargetClass(node)
            })
            .collect();
        Ok(result)
    })
}

fn targets_node<R: FocusRdf>() -> impl RDFNodeParse<R, Output = Vec<Target>>
{
    property_values(&SH_TARGET_NODE).flat_map(move |ts| {
        let result = ts
            .iter()
            .map(|t| {
                let node = R::term_as_object(t);
                Target::TargetNode(node)
            })
            .collect();
        Ok(result)
    })
}
