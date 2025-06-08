use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use srdf::Literal;
use srdf::{
    combine_parsers, combine_vec, get_focus, has_type, instances_of, lang::Lang,
    literal::Literal as EnumLiteral, matcher::Any, not, ok, optional, parse_nodes, property_bool,
    property_value, property_values, property_values_int, property_values_iri,
    property_values_literal, property_values_non_empty, property_values_string, rdf_list, term,
    FocusRDF, Iri as _, PResult, RDFNode, RDFNodeParse, RDFParseError, RDFParser, Rdf, SHACLPath,
    Term, Triple, RDFS_CLASS, RDF_TYPE,
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
                self.rdf_parser.rdf.set_focus(&node.clone().into());
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
        let node_shape_instances: HashSet<_> = self
            .rdf_parser
            .rdf
            .triples_matching(Any, Self::rdf_type(), Self::sh_node_shape())
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

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
            .into_iter()
            .map(|subject: RDF::Subject| subject.into())
            .map(|term: RDF::Term| term.into())
            .collect();

        Ok(result)
    }

    fn get_sh_or_values(&mut self) -> Result<HashSet<RDF::Subject>> {
        let mut rs = HashSet::new();
        for subject in self.objects_with_predicate(Self::sh_or())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Ok(subj) = v.clone().try_into() {
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
        for subject in self.objects_with_predicate(Self::sh_xone())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Ok(subj) = v.clone().try_into() {
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
        for subject in self.objects_with_predicate(Self::sh_and())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Ok(subj) = v.clone().try_into() {
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
        let values_as_subjects = self
            .rdf_parser
            .rdf
            .triples_with_predicate(pred)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_object)
            .flat_map(TryInto::try_into)
            .collect();
        Ok(values_as_subjects)
    }

    /*fn values_of_list(&mut self, term: RDF::Term) -> Result<Vec<RDF::Term>> {
        let values = set_focus(&term).with(rdf_list()).parse_impl(&mut self.rdf_parser.rdf)?;
        Ok(values)
    }*/

    fn rdf_type() -> RDF::IRI {
        RDF_TYPE.clone().into()
    }

    fn sh_node_shape() -> RDF::Term {
        let iri: RDF::IRI = SH_NODE_SHAPE.clone().into();
        iri.into()
    }

    fn sh_property() -> RDF::IRI {
        SH_PROPERTY.clone().into()
    }

    fn sh_or() -> RDF::IRI {
        SH_OR.clone().into()
    }

    fn sh_xone() -> RDF::IRI {
        SH_XONE.clone().into()
    }

    fn sh_and() -> RDF::IRI {
        SH_AND.clone().into()
    }

    fn sh_not() -> RDF::IRI {
        SH_NOT.clone().into()
    }

    fn sh_node() -> RDF::IRI {
        SH_NODE.clone().into()
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
        has_value(),
        language_in(),
        pattern(),
        min_inclusive(),
        min_exclusive(),
        max_inclusive(),
        max_exclusive()
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
            .then(move |t: RDF::Term| ok(&NodeShape::new(t.into())))
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
        let nodes = ts.into_iter().map(Into::into).collect();
        Ok(nodes)
    })
}

fn parse_xone_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    rdf_list().flat_map(|ls| cnv_xone_list::<RDF>(ls))
}

fn cnv_xone_list<RDF>(ls: Vec<RDF::Term>) -> PResult<Component>
where
    RDF: Rdf,
{
    let shapes: Vec<_> = ls.into_iter().map(Into::into).collect();
    Ok(Component::Xone { shapes })
}

fn parse_and_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    rdf_list().flat_map(|ls| cnv_and_list::<RDF>(ls))
}

fn cnv_and_list<RDF>(ls: Vec<RDF::Term>) -> PResult<Component>
where
    RDF: Rdf,
{
    let shapes: Vec<_> = ls.into_iter().map(Into::into).collect();
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
    RDF: Rdf,
{
    Ok(Component::Node { shape: t.into() })
}

fn cnv_not<RDF>(t: RDF::Term) -> PResult<Component>
where
    RDF: Rdf,
{
    Ok(Component::Not { shape: t.into() })
}

fn parse_or_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    rdf_list().flat_map(|ls| cnv_or_list::<RDF>(ls))
}

fn cnv_or_list<RDF>(ls: Vec<RDF::Term>) -> PResult<Component>
where
    RDF: Rdf,
{
    let shapes: Vec<_> = ls.into_iter().map(Into::into).collect();
    Ok(Component::Or { shapes })
}

fn id<RDF>() -> impl RDFNodeParse<RDF, Output = RDFNode>
where
    RDF: FocusRDF,
{
    term().then(move |t: RDF::Term| ok(&t.into()))
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
    if let Ok(iri) = term.try_into() {
        let iri: RDF::IRI = iri;
        let iri_string = iri.as_str();
        let iri_s = IriS::new_unchecked(iri_string);
        ok(&SHACLPath::iri(iri_s))
    } else {
        todo!()
    }
}

fn targets<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target>>
where
    RDF: FocusRDF,
{
    combine_parsers!(
        targets_class(),
        targets_node(),
        targets_implicit_class(),
        targets_subjects_of(),
        targets_objects_of()
    )
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

fn min_inclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_literal(&SH_MIN_INCLUSIVE).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: EnumLiteral = n.as_literal();
                Component::MinInclusive(lit)
            })
            .collect()
    })
}

fn min_exclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_literal(&SH_MIN_EXCLUSIVE).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: EnumLiteral = n.as_literal();
                Component::MinExclusive(lit)
            })
            .collect()
    })
}

fn max_inclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_literal(&SH_MAX_INCLUSIVE).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: EnumLiteral = n.as_literal();
                Component::MaxInclusive(lit)
            })
            .collect()
    })
}

fn max_exclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_literal(&SH_MAX_EXCLUSIVE).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: EnumLiteral = n.as_literal();
                Component::MaxExclusive(lit)
            })
            .collect()
    })
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
            .map(|term: &RDF::Term| Component::Class(term.clone().into()))
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

fn language_in<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Component>> {
    property_values(&SH_LANGUAGE_IN).then(move |node_set| {
        let nodes: Vec<_> = node_set.into_iter().collect();
        parse_nodes(nodes, parse_language_in_values())
    })
}

fn pattern<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Component>> {
    property_values_string(&SH_PATTERN).flat_map(|strs| match strs.len() {
        0 => Ok(Vec::new()),
        1 => {
            let pattern = strs.first().unwrap().clone();
            let flags = None;
            Ok(vec![Component::Pattern { pattern, flags }])
        }
        _ => todo!(), // Error...
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

fn parse_language_in_values<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Component> {
    rdf_list().flat_map(cnv_language_in_list::<R>)
}

fn cnv_has_value<RDF>(term: RDF::Term) -> std::result::Result<Component, RDFParseError>
where
    RDF: Rdf,
{
    let value = term_to_value::<RDF>(&term)?;
    Ok(Component::HasValue { value })
}

fn cnv_language_in_list<R: FocusRDF>(
    terms: Vec<R::Term>,
) -> std::result::Result<Component, RDFParseError> {
    let langs: Vec<Lang> = terms.iter().flat_map(term_to_lang::<R>).collect();
    Ok(Component::LanguageIn { langs })
}

fn term_to_value<RDF>(term: &RDF::Term) -> std::result::Result<Value, RDFParseError>
where
    RDF: Rdf,
{
    if term.is_blank_node() {
        Err(RDFParseError::BlankNodeNoValue {
            bnode: term.to_string(),
        })
    } else if let Ok(iri) = term.clone().try_into() {
        let iri: RDF::IRI = iri;
        let iri_string = iri.as_str();
        let iri_s = IriS::new_unchecked(iri_string);
        Ok(Value::Iri(IriRef::Iri(iri_s)))
    } else if let Ok(literal) = term.clone().try_into() {
        let literal: RDF::Literal = literal;
        Ok(Value::Literal(literal.as_literal()))
    } else {
        todo!()
    }
}

fn term_to_lang<R: FocusRDF>(term: &R::Term) -> std::result::Result<Lang, RDFParseError> {
    if term.is_blank_node() {
        Err(RDFParseError::BlankNodeNoValue {
            bnode: term.to_string(),
        })
    } else if let Ok(literal) = term.clone().try_into() {
        let literal: R::Literal = literal;
        let lang = Lang::new(literal.lexical_form());
        match lang {
            Ok(lang) => Ok(lang),
            Err(_) => todo!(),
        }
    } else {
        todo!()
    }
}

fn cnv_in_list<RDF>(ls: Vec<RDF::Term>) -> std::result::Result<Component, RDFParseError>
where
    RDF: Rdf,
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
    RDF: Rdf,
{
    let iri: RDF::IRI =
        term.clone()
            .try_into()
            .map_err(|_| ShaclParserError::ExpectedNodeKind {
                term: format!("{term}"),
            })?;
    match iri.as_str() {
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

fn targets_class<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target>>
where
    RDF: FocusRDF,
{
    property_values(&SH_TARGET_CLASS).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|t: RDF::Term| Target::TargetClass(t.try_into()))
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
            .into_iter()
            .map(|t: RDF::Term| Target::TargetNode(t.into()))
            .collect();
        Ok(result)
    })
}

fn targets_implicit_class<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target>> {
    // TODO: in general this can be improved
    instances_of(&RDFS_CLASS)
        .and(instances_of(&SH_PROPERTY_SHAPE))
        .and(instances_of(&SH_NODE_SHAPE))
        .and(get_focus())
        .flat_map(
            move |(((class, property_shapes), node_shapes), focus): (_, R::Term)| {
                let result = class
                    .into_iter()
                    .filter(|t: &R::Subject| property_shapes.contains(t) || node_shapes.contains(t))
                    .map(Into::into)
                    .filter(|t: &R::Term| t.clone() == focus)
                    .map(|t: R::Term| Target::TargetImplicitClass(t.into()))
                    .collect();
                Ok(result)
            },
        )
}

fn targets_objects_of<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target>> {
    property_values_iri(&SH_TARGET_OBJECTS_OF).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|t: IriS| Target::TargetObjectsOf(t.into()))
            .collect();
        Ok(result)
    })
}

fn targets_subjects_of<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target>> {
    property_values_iri(&SH_TARGET_SUBJECTS_OF).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|t: IriS| Target::TargetSubjectsOf(t.into()))
            .collect();
        Ok(result)
    })
}

#[cfg(test)]
mod tests {
    use iri_s::IriS;
    use srdf::lang::Lang;
    use srdf::Object;
    use srdf::RDFFormat;
    use srdf::ReaderMode;
    use srdf::SRDFGraph;

    use crate::shape::Shape;

    use super::ShaclParser;

    #[test]
    fn test_language_in() {
        let shape = r#"
            @prefix :    <http://example.org/> .
            @prefix sh:  <http://www.w3.org/ns/shacl#> .
            @prefix xsd: <http://www.w3.org/2001/XMLSchema#> .

            :TestShape a sh:NodeShape ;
                sh:targetNode "Hello"@en ;
                sh:languageIn ( "en" "fr" ) .
        "#;

        let rdf_format = RDFFormat::Turtle;
        let reader_mode = ReaderMode::default();
        let shape_id: Object = IriS::new_unchecked("http://example.org/TestShape").into();

        let graph = SRDFGraph::from_str(shape, &rdf_format, None, &reader_mode).unwrap();
        let schema = ShaclParser::new(graph).parse().unwrap();
        let shape = match schema.get_shape(&shape_id).unwrap() {
            Shape::NodeShape(ns) => ns,
            _ => panic!("Shape is not a NodeShape"),
        };

        match shape.components().first().unwrap() {
            crate::component::Component::LanguageIn { langs } => {
                assert_eq!(langs.len(), 2);
                assert_eq!(langs[0], Lang::new_unchecked("en"));
                assert_eq!(langs[1], Lang::new_unchecked("fr"));
            }
            _ => panic!("Shape has not a LanguageIn component"),
        }
    }
}
