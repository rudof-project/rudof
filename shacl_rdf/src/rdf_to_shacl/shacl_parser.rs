use super::shacl_parser_error::ShaclParserError;
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use shacl_ast::severity::Severity;
use shacl_ast::shacl_vocab::{
    sh_and, sh_class, sh_closed, sh_datatype, sh_has_value, sh_in, sh_language_in, sh_max_count,
    sh_max_exclusive, sh_max_inclusive, sh_max_length, sh_min_count, sh_min_exclusive,
    sh_min_inclusive, sh_min_length, sh_node, sh_node_kind, sh_node_shape, sh_not, sh_or,
    sh_pattern, sh_property_shape, sh_qualified_value_shapes_disjoint, sh_target_class,
    sh_target_node, sh_target_objects_of, sh_target_subjects_of, sh_xone,
};
use shacl_ast::{
    component::Component, node_kind::NodeKind, node_shape::NodeShape,
    property_shape::PropertyShape, schema::Schema, shape::Shape, target::Target, value::Value, *,
};
use srdf::{FnOpaque, rdf_type, rdfs_class};
use srdf::{
    FocusRDF, Iri as _, PResult, RDFNode, RDFNodeParse, RDFParseError, RDFParser, Rdf, SHACLPath,
    Term, Triple, combine_parsers, combine_parsers_vec, combine_vec, get_focus, has_type,
    instances_of, lang::Lang, literal::SLiteral, matcher::Any, not, object, ok, opaque, optional,
    parse_property_values, property_bool, property_iris, property_objects, property_value,
    property_values, property_values_bool, property_values_int, property_values_iri,
    property_values_literal, property_values_non_empty, property_values_string, rdf_list, term,
};
use srdf::{
    Literal, Object, property_integer, property_iri, property_string, property_value_as_list,
};
use srdf::{set_focus, shacl_path_parse};
use std::collections::{HashMap, HashSet};
use tracing::debug;

/// Result type for the ShaclParser
type Result<A> = std::result::Result<A, ShaclParserError>;

/// State used during the parsing process
/// This is used to keep track of pending shapes to be parsed
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
    RDF: FocusRDF,
{
    rdf_parser: RDFParser<RDF>,
    shapes: HashMap<RDFNode, Shape<RDF>>,
}

impl<RDF> ShaclParser<RDF>
where
    RDF: FocusRDF,
{
    pub fn new(rdf: RDF) -> ShaclParser<RDF> {
        ShaclParser {
            rdf_parser: RDFParser::new(rdf),
            shapes: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Schema<RDF>> {
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

    /// Shapes candidates are defined in Appendix A of SHACL spec (Syntax rules)
    /// The text is:
    /// A shape is an IRI or blank node s that fulfills at least one of the following conditions in the shapes graph:
    /// - s is a SHACL instance of sh:NodeShape or sh:PropertyShape.
    /// - s is subject of a triple that has sh:targetClass, sh:targetNode, sh:targetObjectsOf or sh:targetSubjectsOf as predicate.
    /// - s is subject of a triple that has a parameter as predicate.
    /// - s is a value of a shape-expecting, non-list-taking parameter such as sh:node,
    ///   or a member of a SHACL list that is a value of a shape-expecting and list-taking parameter such as sh:or.
    fn shapes_candidates(&mut self) -> Result<Vec<RDFNode>> {
        // instances of `sh:NodeShape`
        let node_shape_instances: HashSet<_> = self
            .rdf_parser
            .rdf
            .triples_matching(Any, Self::rdf_type_iri(), Self::sh_node_shape_iri())
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // instances of `sh:PropertyShape`
        let property_shapes_instances: HashSet<_> = self
            .rdf_parser
            .rdf
            .triples_matching(Any, Self::rdf_type_iri(), Self::sh_property_shape_iri())
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Instances of `sh:Shape`
        let shape_instances: HashSet<_> = self
            .rdf_parser
            .rdf
            .triples_matching(Any, Self::rdf_type_iri(), Self::sh_shape_iri())
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetClass
        let subjects_target_class: HashSet<_> = self
            .rdf_parser
            .rdf
            .triples_matching(Any, into_iri::<RDF>(sh_target_class()), Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetSubjectsOf
        let subjects_target_subjects_of: HashSet<_> = self
            .rdf_parser
            .rdf
            .triples_matching(Any, into_iri::<RDF>(sh_target_subjects_of()), Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetObjectsOf
        let subjects_target_objects_of: HashSet<_> = self
            .rdf_parser
            .rdf
            .triples_matching(Any, into_iri::<RDF>(sh_target_objects_of()), Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetNode
        let subjects_target_node: HashSet<_> = self
            .rdf_parser
            .rdf
            .triples_matching(Any, into_iri::<RDF>(sh_target_node()), Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Search shape expecting parameters: https://www.w3.org/TR/shacl12-core/#dfn-shape-expecting
        // elements of `sh:and` list
        let sh_and_values = self.get_sh_and_values()?;

        // elements of `sh:or` list
        let sh_or_values = self.get_sh_or_values()?;

        // elements of `sh:not` list
        let sh_not_values = self.get_sh_not_values()?;

        // subjects with property `sh:property`
        let subjects_property = self.objects_with_predicate(Self::sh_property_iri())?;

        // elements of `sh:node` list
        let sh_qualified_value_shape_nodes = self.get_sh_qualified_value_shape()?;

        // elements of `sh:node` list
        let sh_node_values = self.get_sh_node_values()?;

        // elements of `sh:xone` list
        let sh_xone_values = self.get_sh_xone_values()?;

        // I would prefer a code like: node_shape_instances.union(subjects_property).union(...)
        // But looking to the union API in HashSet, I think it can't be chained
        let mut candidates = HashSet::new();
        candidates.extend(node_shape_instances);
        candidates.extend(subjects_property);
        candidates.extend(sh_or_values);
        candidates.extend(sh_xone_values);
        candidates.extend(sh_and_values);
        candidates.extend(sh_not_values);
        candidates.extend(sh_qualified_value_shape_nodes);
        candidates.extend(sh_node_values);
        candidates.extend(property_shapes_instances);
        candidates.extend(shape_instances);
        candidates.extend(subjects_target_class);
        candidates.extend(subjects_target_subjects_of);
        candidates.extend(subjects_target_objects_of);
        candidates.extend(subjects_target_node);

        Ok(subjects_as_nodes::<RDF>(candidates)?)
    }

    fn get_sh_or_values(&mut self) -> Result<HashSet<RDF::Subject>> {
        let mut rs = HashSet::new();
        for subject in self.objects_with_predicate(Self::sh_or_iri())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = rdf_list().parse_impl(&mut self.rdf_parser.rdf)?;
            for v in vs {
                if let Ok(subj) = term_to_subject::<RDF>(&v, "sh:or") {
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
                if let Ok(subj) = &term_to_subject::<RDF>(&v, "sh:xone") {
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
                if let Ok(subj) = term_to_subject::<RDF>(&v, "sh:and") {
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

    fn get_sh_qualified_value_shape(&mut self) -> Result<HashSet<RDF::Subject>> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_qualified_value_shape_iri())? {
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
    }

    fn objects_with_predicate(&self, pred: RDF::IRI) -> Result<HashSet<RDF::Subject>> {
        let msg = format!("objects with predicate {pred}");
        let values_as_subjects = self
            .rdf_parser
            .rdf
            .triples_with_predicate(pred)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_object)
            .flat_map(|t| term_to_subject::<RDF>(&t.clone(), msg.as_str()))
            .collect();
        Ok(values_as_subjects)
    }

    /*fn values_of_list(&mut self, term: RDF::Term) -> Result<Vec<RDF::Term>> {
        let values = set_focus(&term).with(rdf_list()).parse_impl(&mut self.rdf_parser.rdf)?;
        Ok(values)
    }*/

    fn rdf_type_iri() -> RDF::IRI {
        rdf_type().clone().into()
    }

    fn sh_node_shape_iri() -> RDF::Term {
        RDF::iris_as_term(sh_node_shape())
    }

    fn sh_property_shape_iri() -> RDF::Term {
        RDF::iris_as_term(sh_property_shape())
    }

    fn sh_shape_iri() -> RDF::Term {
        RDF::iris_as_term(sh_shape())
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
        into_iri::<RDF>(sh_node())
    }

    fn sh_qualified_value_shape_iri() -> RDF::IRI {
        sh_qualified_value_shape().clone().into()
    }

    fn shape<'a>(state: &'a mut State) -> impl RDFNodeParse<RDF, Output = Shape<RDF>> + 'a
    where
        RDF: FocusRDF + 'a,
    {
        node_shape()
            .then(move |ns| ok(&Shape::NodeShape(Box::new(ns))))
            .or(property_shape(state).then(|ps| ok(&Shape::PropertyShape(Box::new(ps)))))
    }
}

fn components<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    // Note: the following code could be replaced by
    // combine_parsers(min_count(), max_count(),...)
    // But we found that the compiler takes too much memory when the number of parsers is large
    combine_parsers_vec(vec![
        // Value type
        class(),
        node_kind(),
        datatype(),
        // Cardinality
        min_count(),
        max_count(),
        // Value range
        min_inclusive(),
        min_exclusive(),
        max_inclusive(),
        max_exclusive(),
        // String based
        min_length(),
        max_length(),
        pattern(),
        // TODO: SHACL 1.2: single line ?
        // single_line(),
        language_in(),
        unique_lang(),
        // SHACL 1.2: List constraint components
        // member_shape(),
        // min_list_length(),
        // max_list_length(),
        // unique_members(),

        // Property pair
        equals(),
        disjoint(),
        less_than(),
        less_than_or_equals(),
        // Logical
        not_component(),
        and(),
        or(),
        xone(),
        // Shape based
        node(),
        // property is handled differently
        // Qualified value shape
        qualified_value_shape(),
        // Other
        closed_component(),
        has_value(),
        in_component(),
        // SPARQL based constraints and SPARQL based constraint components
        // TODO

        // TODO: deactivated is not a shape component...move this code elsewhere?
        deactivated(),
    ])
}

fn property_shape<'a, RDF>(
    _state: &'a mut State,
) -> impl RDFNodeParse<RDF, Output = PropertyShape<RDF>> + 'a
where
    RDF: FocusRDF + 'a,
{
    get_focus().then(move |focus: RDF::Term| {
        optional(has_type(sh_property_shape().clone()))
            .with(
                object()
                    .and(path())
                    .then(move |(id, path)| ok(&PropertyShape::new(id, path))),
            )
            // The following line is required because the path parser moves the focus node
            .then(move |ps| set_focus(&focus.clone()).with(ok(&ps)))
            .then(|ns| optional(severity()).flat_map(move |sev| Ok(ns.clone().with_severity(sev))))
            .then(|ps| targets().flat_map(move |ts| Ok(ps.clone().with_targets(ts))))
            .then(|ps| {
                property_shapes()
                    .flat_map(move |prop_shapes| Ok(ps.clone().with_property_shapes(prop_shapes)))
            })
            .then(move |ps| property_shape_components(ps))
    })
}

fn property_shape_components<RDF>(
    ps: PropertyShape<RDF>,
) -> impl RDFNodeParse<RDF, Output = PropertyShape<RDF>>
where
    RDF: FocusRDF,
{
    components().flat_map(move |cs| Ok(ps.clone().with_components(cs)))
}

fn node_shape<RDF>() -> impl RDFNodeParse<RDF, Output = NodeShape<RDF>>
where
    RDF: FocusRDF,
{
    not(property_values_non_empty(sh_path())).with(
        object()
            .then(move |t: RDFNode| ok(&NodeShape::new(t)))
            .then(|ns| optional(severity()).flat_map(move |sev| Ok(ns.clone().with_severity(sev))))
            .then(|ns| targets().flat_map(move |ts| Ok(ns.clone().with_targets(ts))))
            .then(|ns| {
                property_shapes().flat_map(move |ps| Ok(ns.clone().with_property_shapes(ps)))
            })
            .then(|ns| components().flat_map(move |cs| Ok(ns.clone().with_components(cs)))),
    )
}

fn severity<RDF: FocusRDF>() -> FnOpaque<RDF, Severity> {
    opaque!(property_iri(sh_severity()).map(|iri| match iri.as_str() {
        "http://www.w3.org/ns/shacl#Violation" => Severity::Violation,
        "http://www.w3.org/ns/shacl#Warning" => Severity::Warning,
        "http://www.w3.org/ns/shacl#Info" => Severity::Info,
        _ => Severity::Generic(IriRef::iri(iri)),
    }))
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

fn qualified_value_shape_disjoint_parser<RDF: FocusRDF>() -> FnOpaque<RDF, Option<bool>> {
    opaque!(optional(
        property_bool(sh_qualified_value_shapes_disjoint())
    ))
}

fn qualified_min_count_parser<RDF: FocusRDF>() -> FnOpaque<RDF, Option<isize>> {
    opaque!(optional(property_integer(sh_qualified_min_count())))
}

fn qualified_max_count_parser<RDF: FocusRDF>() -> FnOpaque<RDF, Option<isize>> {
    opaque!(optional(property_integer(sh_qualified_max_count())))
}

fn parse_qualified_value_shape<RDF: FocusRDF>(
    qvs: HashSet<RDFNode>,
) -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    qualified_value_shape_disjoint_parser()
        .and(qualified_min_count_parser())
        .and(qualified_max_count_parser())
        .and(qualified_value_shape_siblings())
        .flat_map(
            move |(((maybe_disjoint, maybe_mins), maybe_maxs), siblings)| {
                Ok(build_qualified_shape::<RDF>(
                    qvs.clone(),
                    maybe_disjoint,
                    maybe_mins,
                    maybe_maxs,
                    siblings,
                ))
            },
        )
}

fn qualified_value_shape_siblings<RDF: FocusRDF>() -> QualifiedValueShapeSiblings<RDF>
where
    RDF: FocusRDF,
{
    QualifiedValueShapeSiblings {
        _marker: std::marker::PhantomData,
        property_qualified_value_shape_path: SHACLPath::sequence(vec![
            SHACLPath::iri(sh_property().clone()),
            SHACLPath::iri(sh_qualified_value_shape().clone()),
        ]),
    }
}

/// This parser gets the siblings of a focus node
/// Siblings are the other qualified value shapes that share the same parent(s)
/// The defnition in the spec is: https://www.w3.org/TR/shacl12-core/#dfn-sibling-shapes
/// "Let Q be a shape in shapes graph G that declares a qualified cardinality constraint
/// (by having values for sh:qualifiedValueShape and at least one of sh:qualifiedMinCount
/// or sh:qualifiedMaxCount).
/// Let ps be the set of shapes in G that have Q as a value of sh:property.
/// If Q has true as a value for sh:qualifiedValueShapesDisjoint then the set of sibling
/// shapes for Q is defined as the set of all values of the SPARQL property path
/// sh:property/sh:qualifiedValueShape for any shape in ps minus the value of
/// sh:qualifiedValueShape of Q itself. The set of sibling shapes is empty otherwise."
struct QualifiedValueShapeSiblings<RDF: FocusRDF> {
    _marker: std::marker::PhantomData<RDF>,
    property_qualified_value_shape_path: SHACLPath,
}

impl<RDF> RDFNodeParse<RDF> for QualifiedValueShapeSiblings<RDF>
where
    RDF: FocusRDF,
{
    type Output = Vec<RDFNode>;

    fn parse_impl(&mut self, rdf: &mut RDF) -> PResult<Self::Output> {
        match rdf.get_focus() {
            Some(focus) => {
                let mut siblings = Vec::new();
                let maybe_disjoint = rdf.object_for(
                    focus,
                    &into_iri::<RDF>(sh_qualified_value_shapes_disjoint()),
                )?;
                if let Some(disjoint) = maybe_disjoint {
                    match disjoint {
                        Object::Literal(SLiteral::BooleanLiteral(true)) => {
                            debug!(
                                "QualifiedValueShapeSiblings: Focus node {focus} has disjoint=true"
                            );
                            let qvs = rdf.objects_for(
                                &focus,
                                &into_iri::<RDF>(sh_qualified_value_shape()),
                            )?;
                            if qvs.is_empty() {
                                debug!(
                                    "Focus node {focus} has disjoint=true but no qualifiedValueShape"
                                );
                            } else {
                                debug!("QVS of focus node {focus}: {qvs:?}");
                                let ps =
                                    rdf.subjects_for(&into_iri::<RDF>(sh_property()), &focus)?;
                                debug!("Property parents of focus node {focus}: {ps:?}");
                                for property_parent in ps {
                                    let candidate_siblings = rdf.objects_for_shacl_path(
                                        &property_parent,
                                        &self.property_qualified_value_shape_path,
                                    )?;
                                    debug!("Candidate siblings: {candidate_siblings:?}");
                                    for sibling in candidate_siblings {
                                        if !qvs.contains(&sibling) {
                                            let sibling_node = RDF::term_as_object(&sibling)?;
                                            siblings.push(sibling_node);
                                        }
                                    }
                                }
                            }
                        }
                        Object::Literal(SLiteral::BooleanLiteral(false)) => {}
                        _ => {
                            debug!(
                                "Value of disjoint: {disjoint} is not boolean (Should we raise an error here?)"
                            );
                        }
                    }
                }
                /*if let Some(true) =
                    rdf.get_object_for(focus, sh_qualified_value_shapes_disjoint())?
                {
                    for p in ps {
                        // TODO: Check that they have qualifiedValueShape also...
                        let qvs = rdf
                            .triples_matching(p.clone().into(), sh_property().clone().into(), Any)
                            .map_err(|e| RDFParseError::SRDFError { err: e.to_string() })?
                            .map(Triple::into_object)
                            .flat_map(|t| RDF::term_as_object(&t).ok());
                        for qv in qvs {
                            if &qv != focus {
                                siblings.push(qv);
                            }
                        }
                    }
                } else {
                };*/

                Ok(siblings)
            }
            None => {
                return Err(RDFParseError::NoFocusNode);
            }
        }
    }
}

fn build_qualified_shape<RDF: FocusRDF>(
    terms: HashSet<RDFNode>,
    qualified_value_shapes_disjoint: Option<bool>,
    qualified_min_count: Option<isize>,
    qualified_max_count: Option<isize>,
    siblings: Vec<RDFNode>,
) -> Vec<Component>
where
    RDF: Rdf,
{
    let mut result = Vec::new();
    for term in terms {
        let shape = Component::QualifiedValueShape {
            shape: term.clone(),
            qualified_min_count,
            qualified_max_count,
            qualified_value_shapes_disjoint,
            siblings: siblings.clone(),
        };
        result.push(shape);
    }
    result
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

fn term_to_subject<RDF>(
    term: &RDF::Term,
    context: &str,
) -> std::result::Result<RDF::Subject, ShaclParserError>
where
    RDF: FocusRDF,
{
    RDF::term_as_subject(term).map_err(|_| ShaclParserError::ExpectedSubject {
        term: term.to_string(),
        context: context.to_string(),
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
            RDF::subject_as_node(&s).map_err(|_| RDFParseError::SubjToRDFNodeFailed {
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
    property_value(sh_path()).then(shacl_path_parse)
}

fn targets<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target<RDF>>>
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

/*opaque! {
    fn min_count[RDF]()(RDF) -> Vec<Component>
    where [
    ] {
        property_values_int(sh_min_count())
        .map(|ns| ns.iter().map(|n| Component::MinCount(*n)).collect())
    }
}

opaque! {
    fn max_count[RDF]()(RDF) -> Vec<Component>
    where [
    ] {
        property_values_int(sh_max_count())
        .map(|ns| ns.iter().map(|n| Component::MaxCount(*n)).collect())
    }
}*/

fn min_count<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(
        property_values_int(sh_min_count())
            .map(|ns| ns.iter().map(|n| Component::MinCount(*n)).collect())
    )
}

fn max_count<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(
        property_values_int(sh_max_count())
            .map(|ns| ns.iter().map(|n| Component::MaxCount(*n)).collect())
    )
}

fn min_length<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(
        property_values_int(sh_min_length())
            .map(|ns| ns.iter().map(|n| Component::MinLength(*n)).collect())
    )
}

fn deactivated<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(
        property_values_bool(sh_deactivated())
            .map(|ns| ns.iter().map(|n| Component::Deactivated(*n)).collect())
    )
}

fn closed_component<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(optional(closed()).then(move |maybe_closed| {
        ignored_properties()
            .map(move |is| maybe_closed.map_or(vec![], |b| vec![Component::closed(b, is)]))
    }))
}

fn ignored_properties<RDF>() -> FnOpaque<RDF, HashSet<IriS>>
where
    RDF: FocusRDF,
{
    opaque!(
        optional(property_value_as_list(sh_ignored_properties())).flat_map(|is| {
            match is {
                None => Ok(HashSet::new()),
                Some(vs) => {
                    let mut hs = HashSet::new();
                    for v in vs {
                        if let Ok(iri) = RDF::term_as_iri(&v) {
                            let iri: RDF::IRI = iri;
                            let iri_string = iri.as_str();
                            let iri_s = IriS::new_unchecked(iri_string);
                            hs.insert(iri_s);
                        } else {
                            return Err(RDFParseError::ExpectedIRI {
                                term: v.to_string(),
                            });
                        }
                    }
                    Ok(hs)
                }
            }
        })
    )
}

fn min_inclusive<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(property_values_literal(sh_min_inclusive()).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: SLiteral = n.as_literal();
                Component::MinInclusive(lit)
            })
            .collect()
    }))
}

fn min_exclusive<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(property_values_literal(sh_min_exclusive()).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: SLiteral = n.as_literal();
                Component::MinExclusive(lit)
            })
            .collect()
    }))
}

fn max_inclusive<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(property_values_literal(sh_max_inclusive()).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: SLiteral = n.as_literal();
                Component::MaxInclusive(lit)
            })
            .collect()
    }))
}

fn max_exclusive<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(property_values_literal(sh_max_exclusive()).map(|ns| {
        ns.iter()
            .map(|n: &<RDF as Rdf>::Literal| {
                let lit: SLiteral = n.as_literal();
                Component::MaxExclusive(lit)
            })
            .collect()
    }))
}

fn equals<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(property_values_iri(sh_equals()).map(|ns| {
        ns.iter()
            .map(|n| {
                let iri: IriRef = IriRef::iri(n.clone());
                Component::Equals(iri)
            })
            .collect()
    }))
}

fn disjoint<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(property_values_iri(sh_disjoint()).map(|ns| {
        ns.iter()
            .map(|n| {
                let iri: IriRef = IriRef::iri(n.clone());
                Component::Disjoint(iri)
            })
            .collect()
    }))
}

fn less_than<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(property_values_iri(sh_less_than()).map(|ns| {
        ns.iter()
            .map(|n| {
                let iri: IriRef = IriRef::iri(n.clone());
                Component::LessThan(iri)
            })
            .collect()
    }))
}

fn less_than_or_equals<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(property_values_iri(sh_less_than_or_equals()).map(|ns| {
        ns.iter()
            .map(|n| {
                let iri: IriRef = IriRef::iri(n.clone());
                Component::LessThanOrEquals(iri)
            })
            .collect()
    }))
}

fn max_length<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(
        property_values_int(sh_max_length())
            .map(|ns| ns.iter().map(|n| Component::MaxLength(*n)).collect())
    )
}

fn datatype<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(property_values_iri(sh_datatype()).map(|ns| {
        ns.iter()
            .map(|iri| Component::Datatype(IriRef::iri(iri.clone())))
            .collect()
    }))
}

fn class<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(
        property_objects(sh_class())
            .map(|ns| ns.iter().map(|n| Component::Class(n.clone())).collect())
    )
}

fn node_kind<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(property_values(sh_node_kind()).flat_map(|ns| {
        let nks: Vec<_> = ns
            .iter()
            .flat_map(|term| {
                let nk = term_to_node_kind::<RDF>(term)?;
                Ok::<Component, ShaclParserError>(Component::NodeKind(nk))
            })
            .collect();
        Ok(nks)
    }))
}

fn has_value<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(parse_components_for_iri(
        sh_has_value(),
        parse_has_value_values()
    ))
}

fn in_component<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(parse_components_for_iri(sh_in(), parse_in_values()))
}

fn language_in<RDF: FocusRDF>() -> FnOpaque<RDF, Vec<Component>> {
    // impl RDFNodeParse<R, Output = Vec<Component>> {
    opaque!(parse_components_for_iri(
        sh_language_in(),
        parse_language_in_values()
    ))
}

fn pattern<RDF: FocusRDF>() -> FnOpaque<RDF, Vec<Component>> {
    opaque!(optional(flags()).then(move |maybe_flags| {
        property_values_string(sh_pattern()).flat_map(move |strs| match strs.len() {
            0 => Ok(Vec::new()),
            1 => {
                let pattern = strs.first().unwrap().clone();
                let flags = maybe_flags.clone();
                Ok(vec![Component::Pattern { pattern, flags }])
            }
            _ => todo!(), // Error...
        })
    }))
}

fn flags<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = String> {
    property_string(sh_flags())
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
    term().flat_map(|t| cnv_has_value::<RDF>(t))
}

fn parse_language_in_values<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Component> {
    rdf_list().flat_map(cnv_language_in_list::<R>)
}

fn cnv_has_value<RDF>(term: RDF::Term) -> std::result::Result<Component, RDFParseError>
where
    RDF: Rdf,
{
    let value = term_to_value::<RDF>(&term, "parsing hasValue")?;
    Ok(Component::HasValue { value })
}

fn cnv_language_in_list<R: FocusRDF>(
    terms: Vec<R::Term>,
) -> std::result::Result<Component, RDFParseError> {
    let langs: Vec<Lang> = terms.iter().flat_map(R::term_as_lang).collect();
    Ok(Component::LanguageIn { langs })
}

fn term_to_value<RDF>(term: &RDF::Term, msg: &str) -> std::result::Result<Value, RDFParseError>
where
    RDF: Rdf,
{
    if term.is_blank_node() {
        Err(RDFParseError::BlankNodeNoValue {
            bnode: term.to_string(),
            msg: msg.to_string(),
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
        println!("Unexpected code in term_to_value: {term}: {msg}");
        todo!()
    }
}

fn cnv_in_list<RDF>(ls: Vec<RDF::Term>) -> std::result::Result<Component, RDFParseError>
where
    RDF: Rdf,
{
    let values = ls
        .iter()
        .flat_map(|t| term_to_value::<RDF>(t, "parsing in list"))
        .collect();
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

fn or<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(parse_components_for_iri(sh_or(), parse_or_values()))
}

fn xone<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(parse_components_for_iri(sh_xone(), parse_xone_values()))
}

fn and<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(parse_components_for_iri(sh_and(), parse_and_values()))
}

fn not_component<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(parse_components_for_iri(sh_not(), parse_not_value()))
}

fn node<RDF>() -> FnOpaque<RDF, Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(parse_components_for_iri(sh_node(), parse_node_value()))
}

fn qualified_value_shape<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(
        property_objects(sh_qualified_value_shape())
            .then(|qvs| { parse_qualified_value_shape::<RDF>(qvs) })
    )
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

fn targets_class<RDF>() -> FnOpaque<RDF, Vec<Target<RDF>>>
where
    RDF: FocusRDF,
{
    opaque!(property_iris(sh_target_class()).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|iri| Target::TargetClass(RDFNode::iri(iri)))
            .collect();
        Ok(result)
    }))
}

fn targets_node<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target<RDF>>>
where
    RDF: FocusRDF,
{
    property_objects(sh_target_node()).flat_map(move |ts| {
        let result = ts.into_iter().map(Target::TargetNode).collect();
        Ok(result)
    })
}

fn targets_implicit_class<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target<R>>> {
    instances_of(rdfs_class())
        .and(instances_of(sh_property_shape()))
        .and(instances_of(sh_node_shape()))
        .and(get_focus())
        .flat_map(
            move |(((class, property_shapes), node_shapes), focus): (_, R::Term)| {
                let result: std::result::Result<Vec<Target<R>>, RDFParseError> = class
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

fn targets_objects_of<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target<R>>> {
    property_values_iri(sh_target_objects_of()).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|t: IriS| Target::TargetObjectsOf(t.into()))
            .collect();
        Ok(result)
    })
}

fn targets_subjects_of<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target<R>>> {
    property_values_iri(sh_target_subjects_of()).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|t: IriS| Target::TargetSubjectsOf(t.into()))
            .collect();
        Ok(result)
    })
}

#[cfg(test)]
mod tests {
    use super::ShaclParser;
    use iri_s::IriS;
    use shacl_ast::shape::Shape;
    use srdf::Object;
    use srdf::RDFFormat;
    use srdf::ReaderMode;
    use srdf::SRDFGraph;
    use srdf::lang::Lang;

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

fn unique_lang<RDF>() -> FnOpaque<RDF, Vec<Component>>
where
    RDF: FocusRDF,
{
    opaque!(
        property_values_bool(sh_unique_lang())
            .map(|ns| ns.iter().map(|n| Component::UniqueLang(*n)).collect())
    )
}

fn into_iri<RDF: Rdf>(iri: &IriS) -> RDF::IRI {
    iri.clone().into()
}
