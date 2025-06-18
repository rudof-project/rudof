use super::shacl_parser_error::ShaclParserError;
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use shacl_ast::shacl_vocab::{
    sh_and, sh_class, sh_closed, sh_datatype, sh_has_value, sh_in, sh_language_in, sh_max_count,
    sh_max_exclusive, sh_max_inclusive, sh_max_length, sh_min_count, sh_min_exclusive,
    sh_min_inclusive, sh_min_length, sh_node, sh_node_kind, sh_node_shape, sh_not, sh_or,
    sh_pattern, sh_property_shape, sh_target_class, sh_target_node, sh_target_objects_of,
    sh_target_subjects_of, sh_xone,
};
use shacl_ast::{
    component::Component, node_kind::NodeKind, node_shape::NodeShape,
    property_shape::PropertyShape, schema::Schema, shape::Shape, target::Target, value::Value, *,
};
use srdf::Literal;
use srdf::{
    combine_parsers, combine_vec, get_focus, has_type, instances_of, lang::Lang, literal::SLiteral,
    matcher::Any, not, object, ok, optional, parse_property_values, property_bool,
    property_objects, property_value, property_values, property_values_int, property_values_iri,
    property_values_literal, property_values_non_empty, property_values_string, rdf_list, term,
    FocusRDF, Iri as _, PResult, RDFNode, RDFNodeParse, RDFParseError, RDFParser, Rdf, SHACLPath,
    Term, Triple,
};
use srdf::{rdf_type, rdfs_class};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
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
        /*    let prefixmap: PrefixMap = self.rdf_parser.prefixmap().unwrap_or_default();

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
                .triples_matching(Any, Self::rdf_type_iri(), Self::sh_node_shape_iri())
                .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
                .map(Triple::into_subject)
                .collect();

            // subjects with property `sh:property`
            let subjects_property = self.objects_with_predicate(Self::sh_property_iri())?;

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

            Ok(subjects_as_nodes::<RDF>(candidates)?)
            */
        todo!()
    }

    /*fn get_sh_or_values(&mut self) -> Result<HashSet<RDF::Subject>> {
        let mut rs = HashSet::new();
        for subject in self.objects_with_predicate(sh_or())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Ok(subj) = term_to_subject::<RDF>(&v) {
                    rs.insert(subj.clone());
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
        for subject in self.objects_with_predicate(Self::sh_xone_iri())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Ok(subj) = &term_to_subject::<RDF>(&v) {
                    rs.insert(subj.clone());
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
        for subject in self.objects_with_predicate(Self::sh_and_iri())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Ok(subj) = term_to_subject::<RDF>(&v) {
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
        for s in self.objects_with_predicate(Self::sh_not_iri())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn get_sh_node_values(&mut self) -> Result<HashSet<RDF::Subject>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_node_iri())? {
            rs.insert(s);
        }
        Ok(rs)
    }*/

    /*fn objects_with_predicate(&self, pred: RDF::IRI) -> Result<HashSet<RDF::Subject>> {
        let values_as_subjects = self
            .rdf_parser
            .rdf
            .triples_with_predicate(pred)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_object)
            .flat_map(|t| term_to_subject::<RDF>(&t))
            .collect();
        Ok(values_as_subjects)
    }*/

    /*fn values_of_list(&mut self, term: RDF::Term) -> Result<Vec<RDF::Term>> {
        let values = set_focus(&term).with(rdf_list()).parse_impl(&mut self.rdf_parser.rdf)?;
        Ok(values)
    }*/

    /*fn rdf_type_iri() -> RDF::IRI {
        rdf_type().clone().into()
    }

    fn sh_node_shape_iri() -> RDF::Term {
        RDF::iris_as_term(sh_node_shape())
    }

    fn sh_property_iri() -> RDF::IRI {
        sh_property().clone().into()
    }

    fn sh_or_iri() -> RDF::IRI {
        sh_or().clone().into()
    }

    fn sh_xone_iri() -> RDF::IRI {
        sh_xone().clone().into()
    }

    fn sh_and_iri() -> RDF::IRI {
        sh_and().clone().into()
    }

    fn sh_not_iri() -> RDF::IRI {
        sh_not().clone().into()
    }

    fn sh_node_iri() -> RDF::IRI {
        sh_node().clone().into()
    }*/

    /*fn shape<'a>(state: &'a mut State) -> impl RDFNodeParse<RDF, Output = Shape> + 'a
    where
        RDF: FocusRDF + 'a,
    {
        node_shape()
            .then(move |ns| ok(&Shape::NodeShape(Box::new(ns))))
            .or(property_shape(state).then(|ps| ok(&Shape::PropertyShape(Box::new(ps)))))
    }*/
}

/*fn components<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
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
    optional(has_type(sh_property_shape().clone()))
        .with(
            object()
                .and(path())
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
    not(property_values_non_empty(sh_path())).with(
        object()
            .then(move |t: RDFNode| ok(&NodeShape::new(t)))
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
    property_objects(sh_property()).map(|ps| ps.into_iter().collect())
}

fn parse_xone_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    rdf_list().flat_map(|ls| cnv_xone_list::<RDF>(ls))
}

fn cnv_xone_list<RDF>(ls: Vec<RDF::Term>) -> PResult<Component>
where
    RDF: Rdf,
{
    let shapes: Vec<_> = terms_as_nodes::<RDF>(ls)?; // ls.into_iter().map(|t| t.try_into()).collect();
    Ok(Component::Xone { shapes })
}

fn parse_and_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    rdf_list().flat_map(|ls| cnv_and_list::<RDF>(ls))
}

fn cnv_and_list<RDF>(ls: Vec<RDF::Term>) -> PResult<Component>
where
    RDF: Rdf,
{
    let shapes: Vec<_> = terms_as_nodes::<RDF>(ls)?;
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
    let shape = RDF::term_as_object(&t).map_err(|_| RDFParseError::TermToRDFNodeFailed {
        term: t.to_string(),
    })?;
    Ok(Component::Node { shape })
}

fn cnv_not<RDF>(t: RDF::Term) -> PResult<Component>
where
    RDF: Rdf,
{
    let shape = RDF::term_as_object(&t).map_err(|_| RDFParseError::TermToRDFNodeFailed {
        term: t.to_string(),
    })?;
    Ok(Component::Not { shape })
}

fn parse_or_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    rdf_list().flat_map(|ls| cnv_or_list::<RDF>(ls))
}

fn cnv_or_list<RDF: Rdf>(ls: Vec<RDF::Term>) -> PResult<Component> {
    let shapes: Vec<_> = terms_as_nodes::<RDF>(ls)?;
    Ok(Component::Or { shapes })
}

fn term_to_subject<RDF>(term: &RDF::Term) -> std::result::Result<RDF::Subject, ShaclParserError>
where
    RDF: FocusRDF,
{
    RDF::term_as_subject(term).map_err(|_| ShaclParserError::ExpectedSubject {
        term: term.to_string(),
    })
}

fn terms_as_nodes<RDF: Rdf>(
    terms: Vec<RDF::Term>,
) -> std::result::Result<Vec<RDFNode>, RDFParseError> {
    terms
        .into_iter()
        .map(|t| {
            let term_name = t.to_string();
            RDF::term_as_object(&t)
                .map_err(|_| RDFParseError::TermToRDFNodeFailed { term: term_name })
        })
        .collect()
}

fn subjects_as_nodes<RDF: Rdf>(
    subjs: HashSet<RDF::Subject>,
) -> std::result::Result<Vec<RDFNode>, RDFParseError> {
    subjs
        .into_iter()
        .map(|s| {
            RDF::subject_as_object(&s).map_err(|_| RDFParseError::SubjToRDFNodeFailed {
                subj: s.to_string(),
            })
        })
        .collect()
}

/// Parses the property value of the focus node as a SHACL path
fn path<RDF>() -> impl RDFNodeParse<RDF, Output = SHACLPath>
where
    RDF: FocusRDF,
{
    property_value(sh_path()).then(shacl_path)
}

/// Parses the current focus node as a SHACL path
fn shacl_path<RDF>(term: RDF::Term) -> impl RDFNodeParse<RDF, Output = SHACLPath>
where
    RDF: FocusRDF,
{
    if let Ok(iri) = RDF::term_as_iri(&term) {
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
    property_bool(sh_closed())
}

fn min_count<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_int(sh_min_count())
        .map(|ns| ns.iter().map(|n| Component::MinCount(*n)).collect())
}

fn max_count<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_int(sh_max_count())
        .map(|ns| ns.iter().map(|n| Component::MaxCount(*n)).collect())
}

fn min_length<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_int(sh_min_length())
        .map(|ns| ns.iter().map(|n| Component::MinLength(*n)).collect())
}

fn min_inclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_literal(sh_min_inclusive()).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: SLiteral = n.as_literal();
                Component::MinInclusive(lit)
            })
            .collect()
    })
}

fn min_exclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_literal(sh_min_exclusive()).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: SLiteral = n.as_literal();
                Component::MinExclusive(lit)
            })
            .collect()
    })
}

fn max_inclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_literal(sh_max_inclusive()).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: SLiteral = n.as_literal();
                Component::MaxInclusive(lit)
            })
            .collect()
    })
}

fn max_exclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_literal(sh_max_exclusive()).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: SLiteral = n.as_literal();
                Component::MaxExclusive(lit)
            })
            .collect()
    })
}

fn max_length<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_int(sh_max_length())
        .map(|ns| ns.iter().map(|n| Component::MaxLength(*n)).collect())
}

fn datatype<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values_iri(sh_datatype()).map(|ns| {
        ns.iter()
            .map(|iri| Component::Datatype(IriRef::iri(iri.clone())))
            .collect()
    })
}

fn class<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_objects(sh_class()).map(|ns| ns.iter().map(|n| Component::Class(n.clone())).collect())
}

fn node_kind<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    property_values(sh_node_kind()).flat_map(|ns| {
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
    parse_components_for_iri(sh_has_value(), parse_has_value_values())
}

fn in_component<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_in(), parse_in_values())
}

fn language_in<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Component>> {
    parse_components_for_iri(sh_language_in(), parse_language_in_values())
}

fn pattern<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Component>> {
    property_values_string(sh_pattern()).flat_map(|strs| match strs.len() {
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
    let langs: Vec<Lang> = terms.iter().flat_map(R::term_as_lang).collect();
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
    } else if let Ok(iri) = RDF::term_as_iri(term) {
        let iri: RDF::IRI = iri;
        let iri_string = iri.as_str();
        let iri_s = IriS::new_unchecked(iri_string);
        Ok(Value::Iri(IriRef::Iri(iri_s)))
    } else if let Ok(literal) = RDF::term_as_literal(term) {
        let literal: RDF::Literal = literal;
        Ok(Value::Literal(literal.as_literal()))
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

fn parse_components_for_iri<RDF, P>(
    iri: &IriS,
    component_parser: P,
) -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
    P: RDFNodeParse<RDF, Output = Component>,
{
    parse_property_values(iri, component_parser)
}

fn or<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_or(), parse_or_values())
}

fn xone<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_xone(), parse_xone_values())
}

fn and<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_and(), parse_and_values())
}

fn not_parser<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_not(), parse_not_value())
}

fn node<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_node(), parse_node_value())
}

fn term_to_node_kind<RDF>(term: &RDF::Term) -> Result<NodeKind>
where
    RDF: Rdf,
{
    let term_name = term.to_string();
    let result_iri: Result<RDF::IRI> = <RDF::Term as TryInto<RDF::IRI>>::try_into(term.clone())
        .map_err(|_| ShaclParserError::ExpectedNodeKind {
            term: term_name.to_string(),
        });
    let iri = result_iri?;
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
    property_objects(sh_target_class()).flat_map(move |ts| {
        let result = ts.into_iter().map(Target::TargetClass).collect();
        Ok(result)
    })
}

fn targets_node<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target>>
where
    RDF: FocusRDF,
{
    property_objects(sh_target_node()).flat_map(move |ts| {
        let result = ts.into_iter().map(Target::TargetNode).collect();
        Ok(result)
    })
}

fn targets_implicit_class<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target>> {
    instances_of(rdfs_class())
        .and(instances_of(sh_property_shape()))
        .and(instances_of(sh_node_shape()))
        .and(get_focus())
        .flat_map(
            move |(((class, property_shapes), node_shapes), focus): (_, R::Term)| {
                let result: std::result::Result<Vec<Target>, RDFParseError> = class
                    .into_iter()
                    .filter(|t: &R::Subject| property_shapes.contains(t) || node_shapes.contains(t))
                    .map(Into::into)
                    .filter(|t: &R::Term| t.clone() == focus)
                    .map(|t: R::Term| {
                        let t_name = t.to_string();
                        let obj = t
                            .clone()
                            .try_into()
                            .map_err(|_| RDFParseError::TermToRDFNodeFailed { term: t_name })?;
                        Ok(Target::TargetImplicitClass(obj))
                    })
                    .collect();
                let ts = result?;
                Ok(ts)
            },
        )
}

fn targets_objects_of<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target>> {
    property_values_iri(sh_target_objects_of()).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|t: IriS| Target::TargetObjectsOf(t.into()))
            .collect();
        Ok(result)
    })
}

fn targets_subjects_of<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target>> {
    property_values_iri(sh_target_subjects_of()).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|t: IriS| Target::TargetSubjectsOf(t.into()))
            .collect();
        Ok(result)
    })
}
*/
#[cfg(test)]
mod tests {
    use super::ShaclParser;
    use iri_s::IriS;
    use shacl_ast::shape::Shape;
    use srdf::lang::Lang;
    use srdf::Object;
    use srdf::RDFFormat;
    use srdf::ReaderMode;
    use srdf::SRDFGraph;

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
            crate::rdf_to_shacl::shacl_parser::Component::LanguageIn { langs } => {
                assert_eq!(langs.len(), 2);
                assert_eq!(langs[0], Lang::new_unchecked("en"));
                assert_eq!(langs[1], Lang::new_unchecked("fr"));
            }
            _ => panic!("Shape has not a LanguageIn component"),
        }
    }
}
