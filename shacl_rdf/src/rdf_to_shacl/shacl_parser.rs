use super::shacl_parser_error::ShaclParserError;
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use rdf::rdf_core::parser::rdf_node_parser::ParserExt;
use rdf::rdf_core::parser::rdf_node_parser::constructors::{
    BoolsPropertyParser, IntegersPropertyParser,
};
use rdf::rdf_core::{
    Any, FocusRDF, RDFError, Rdf, SHACLPath,
    parser::{
        RDFParse,
        rdf_node_parser::{
            RDFNodeParse,
            constructors::{
                FocusParser, HasTypeParser, InstancesParser, IrisPropertyParser,
                ListParser, LiteralsPropertyParser, NonEmptyValuesPropertyParser, ObjectParser,
                ObjectsPropertyParser, SetFocusParser, ShaclPathParser, SingleBoolPropertyParser,
                SingleIntegerPropertyParser, SingleIriPropertyParser, SingleStringPropertyParser,
                SingleValuePropertyAsListParser, SingleValuePropertyParser, StringsPropertyParser,
                SuccessParser, TermParser, ValuesPropertyParser,
            },
        },
    },
    term::{
        Iri, Object, Term, Triple,
        literal::{ConcreteLiteral, Lang},
    },
    vocab::{rdf_type, rdfs_class},
};
use shacl_ast::reifier_info::ReifierInfo;
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
use std::collections::{HashMap, HashSet};
use tracing::trace;

/// State used during the parsing process
/// This is used to keep track of pending shapes to be parsed
struct State {
    pending: Vec<Object>,
}

impl State {
    fn from(pending: Vec<Object>) -> Self {
        State { pending }
    }

    fn pop_pending(&mut self) -> Option<Object> {
        self.pending.pop()
    }
}

pub struct ShaclParser<RDF>
where
    RDF: FocusRDF,
{
    rdf_parser: RDFParse<RDF>,
    shapes: HashMap<Object, Shape<RDF>>,
}

impl<RDF> ShaclParser<RDF>
where
    RDF: FocusRDF,
{
    pub fn new(rdf: RDF) -> ShaclParser<RDF> {
        ShaclParser {
            rdf_parser: RDFParse::new(rdf),
            shapes: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Schema<RDF>, ShaclParserError> {
        let prefixmap: PrefixMap = self.rdf_parser.prefixmap().unwrap_or_default();

        let mut state = State::from(self.shapes_candidates()?);
        while let Some(node) = state.pop_pending() {
            if let std::collections::hash_map::Entry::Vacant(e) = self.shapes.entry(node.clone()) {
                self.rdf_parser.rdf_mut().set_focus(&node.clone().into());
                let shape = Self::shape(&mut state)
                    .parse_focused(self.rdf_parser.rdf_mut())
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
    fn shapes_candidates(&mut self) -> Result<Vec<Object>, ShaclParserError> {
        // instances of `sh:NodeShape`
        let node_shape_instances: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching(&Any, &Self::rdf_type_iri(), &Self::sh_node_shape_iri())
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // instances of `sh:PropertyShape`
        let property_shapes_instances: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching(&Any, &Self::rdf_type_iri(), &Self::sh_property_shape_iri())
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Instances of `sh:Shape`
        let shape_instances: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching(&Any, &Self::rdf_type_iri(), &Self::sh_shape_iri())
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetClass
        let subjects_target_class: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching(&Any, &into_iri::<RDF>(sh_target_class()), &Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetSubjectsOf
        let subjects_target_subjects_of: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching(&Any, &into_iri::<RDF>(sh_target_subjects_of()), &Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetObjectsOf
        let subjects_target_objects_of: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching(&Any, &into_iri::<RDF>(sh_target_objects_of()), &Any)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_subject)
            .collect();

        // Subjects of sh:targetNode
        let subjects_target_node: HashSet<_> = self
            .rdf_parser
            .rdf()
            .triples_matching(&Any, &into_iri::<RDF>(sh_target_node()), &Any)
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

        // elements of `sh:reifierShape` list
        let sh_reifier_shape_values = self.get_sh_reifier_shape_values()?;

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
        candidates.extend(sh_reifier_shape_values);
        candidates.extend(shape_instances);
        candidates.extend(subjects_target_class);
        candidates.extend(subjects_target_subjects_of);
        candidates.extend(subjects_target_objects_of);
        candidates.extend(subjects_target_node);

        Ok(subjects_as_nodes::<RDF>(candidates)?)
    }

    fn get_sh_or_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for subject in self.objects_with_predicate(Self::sh_or_iri())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = ListParser::new().parse_focused(self.rdf_parser.rdf_mut())?;
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

    fn get_sh_xone_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for subject in self.objects_with_predicate(Self::sh_xone_iri())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = ListParser::new().parse_focused(self.rdf_parser.rdf_mut())?;
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

    fn get_sh_reifier_shape_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_reifier_shape_iri())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn get_sh_and_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for subject in self.objects_with_predicate(Self::sh_and_iri())? {
            self.rdf_parser.set_focus(&subject.into());
            let vs = ListParser::new().parse_focused(self.rdf_parser.rdf_mut())?;
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

    fn get_sh_not_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_not_iri())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn get_sh_qualified_value_shape(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_qualified_value_shape_iri())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn get_sh_node_values(&mut self) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let mut rs = HashSet::new();
        for s in self.objects_with_predicate(Self::sh_node_iri())? {
            rs.insert(s);
        }
        Ok(rs)
    }

    fn objects_with_predicate(
        &self,
        pred: RDF::IRI,
    ) -> Result<HashSet<RDF::Subject>, ShaclParserError> {
        let msg = format!("objects with predicate {pred}");
        let values_as_subjects = self
            .rdf_parser
            .rdf()
            .triples_with_predicate(&pred)
            .map_err(|e| ShaclParserError::Custom { msg: e.to_string() })?
            .map(Triple::into_object)
            .flat_map(|t| term_to_subject::<RDF>(&t.clone(), msg.as_str()))
            .collect();
        Ok(values_as_subjects)
    }

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

    fn sh_reifier_shape_iri() -> RDF::IRI {
        sh_reifier_shape().clone().into()
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

    fn shape(state: &mut State) -> impl RDFNodeParse<RDF, Output = Shape<RDF>>
    where
        RDF: FocusRDF + 'static,
    {
        node_shape()
            .then(move |ns| SuccessParser::new(Shape::NodeShape(Box::new(ns))))
            .or(property_shape(state)
                .then(|ps| SuccessParser::new(Shape::PropertyShape(Box::new(ps)))))
    }
}

fn components<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    let parsers: Vec<Box<dyn RDFNodeParse<RDF, Output = Vec<Component>>>> = vec![
        // Value type
        Box::new(node_kind()),
        Box::new(datatype()),
        // Cardinality
        Box::new(min_count()),
        Box::new(max_count()),
        // Value range
        Box::new(min_inclusive()),
        Box::new(min_exclusive()),
        Box::new(max_inclusive()),
        Box::new(max_exclusive()),
        // String based
        Box::new(min_length()),
        Box::new(max_length()),
        Box::new(pattern()),
        // TODO: SHACL 1.2: single line ?
        // single_line(),
        Box::new(language_in()),
        Box::new(unique_lang()),
        // SHACL 1.2: List constraint components
        // member_shape(),
        // min_list_length(),
        // max_list_length(),
        // unique_members(),
        // Property pair
        Box::new(equals()),
        Box::new(disjoint()),
        Box::new(less_than()),
        Box::new(less_than_or_equals()),
        // Logical constraint components
        Box::new(not_component()),
        Box::new(and()),
        Box::new(or()),
        Box::new(xone()),
        // Shape based constraint components
        Box::new(node()),
        // property is handled differently
        Box::new(qualified_value_shape()),
        // Other
        Box::new(closed_component()),
        Box::new(has_value()),
        Box::new(in_component()),
        // SPARQL based constraints and SPARQL based constraint components
        // TODO

        // TODO: deactivated is not a shape component...move this code elsewhere?
        Box::new(deactivated()),
    ];

    Box::new(class()).combine_many(parsers)
}

fn property_shape<RDF>(_state: &mut State) -> impl RDFNodeParse<RDF, Output = PropertyShape<RDF>>
where
    RDF: FocusRDF + 'static,
{
    FocusParser::new().then(move |focus: RDF::Term| {
        HasTypeParser::new(sh_property_shape().clone())
            .optional()
            .with(
                ObjectParser::new()
                    .and(path())
                    .then(move |(id, path)| SuccessParser::new(PropertyShape::new(id, path))),
            )
            .then(move |ps| SetFocusParser::new(focus.clone()).with(SuccessParser::new(ps)))
            .then(|ps| {
                severity()
                    .optional()
                    .flat_map(move |sev| Ok(ps.clone().with_severity(sev)))
            })
            .then(|ps| {
                reifier_shape().flat_map(move |r_shape| Ok(ps.clone().with_reifier_shape(r_shape)))
            })
            .then(|ps| {
                targets().flat_map(move |ts| Ok(ps.clone().with_targets(ts)))
            })
            .then(|ps| {
                property_shapes()
                    .flat_map(move |prop_shapes| Ok(ps.clone().with_property_shapes(prop_shapes)))
                    .then(move |ps_with_props| property_shape_components(ps_with_props))
            })
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
    NonEmptyValuesPropertyParser::new(sh_path().clone()).not().with(
        ObjectParser::new()
            .then(move |t: Object| SuccessParser::new(NodeShape::new(t)))
            .then(|ns| {
                severity()
                    .optional()
                    .flat_map(move |sev| Ok(ns.clone().with_severity(sev)))
            })
            .then(|ns| {
                targets().flat_map(move |ts| Ok(ns.clone().with_targets(ts)))
            })
            .then(|ns| {
                property_shapes()
                    .flat_map(move |ps| Ok(ns.clone().with_property_shapes(ps)))
                    .then(|ns_with_ps| {
                        components().flat_map(move |cs| Ok(ns_with_ps.clone().with_components(cs)))
                    })
            }),
    )
}

fn severity<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Severity> {
    SingleIriPropertyParser::new(sh_severity().clone()).map(|iri| match iri.as_str() {
        "http://www.w3.org/ns/shacl#Violation" => Severity::Violation,
        "http://www.w3.org/ns/shacl#Warning" => Severity::Warning,
        "http://www.w3.org/ns/shacl#Info" => Severity::Info,
        _ => Severity::Generic(IriRef::iri(iri)),
    })
}

fn property_shapes<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Object>> {
    ObjectsPropertyParser::new(sh_property().clone()).map(|ps| ps.into_iter().collect())
}

fn parse_xone_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    ListParser::new().flat_map(|ls| cnv_xone_list::<RDF>(ls))
}

fn cnv_xone_list<RDF>(ls: Vec<RDF::Term>) -> Result<Component, RDFError>
where
    RDF: Rdf,
{
    let shapes: Vec<_> = terms_as_nodes::<RDF>(ls)?; // ls.into_iter().map(|t| t.try_into()).collect();
    Ok(Component::Xone { shapes })
}

fn parse_and_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    ListParser::new().flat_map(|ls| cnv_and_list::<RDF>(ls))
}

fn cnv_and_list<RDF>(ls: Vec<RDF::Term>) -> Result<Component, RDFError>
where
    RDF: Rdf,
{
    let shapes: Vec<_> = terms_as_nodes::<RDF>(ls)?;
    Ok(Component::And { shapes })
}

fn parse_not_value<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    TermParser::new().flat_map(|t| cnv_not::<RDF>(t))
}

fn parse_node_value<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    TermParser::new().flat_map(|t| cnv_node::<RDF>(t))
}

fn qualified_value_shape_disjoint_parser<RDF: FocusRDF>()
-> impl RDFNodeParse<RDF, Output = Option<bool>> {
    SingleBoolPropertyParser::new(sh_qualified_value_shapes_disjoint().clone()).optional()
}

fn qualified_min_count_parser<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Option<isize>> {
    SingleIntegerPropertyParser::new(sh_qualified_min_count().clone()).optional()
}

fn qualified_max_count_parser<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Option<isize>> {
    SingleIntegerPropertyParser::new(sh_qualified_max_count().clone()).optional()
}

fn parse_qualified_value_shape<RDF: FocusRDF>(
    qvs: HashSet<Object>,
) -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    qualified_value_shape_disjoint_parser()
        .and(qualified_min_count_parser())
        .and(qualified_max_count_parser())
        .and(qualified_value_shape_siblings())
        .flat_map(
            move |(((maybe_disjoint, maybe_mins), maybe_maxs), siblings)| {
                Ok(build_qualified_shape(
                    qvs.clone(),
                    maybe_disjoint,
                    maybe_mins,
                    maybe_maxs,
                    siblings,
                ))
            },
        )
}

fn qualified_value_shape_siblings<RDF: FocusRDF>() -> QualifiedValueShapeSiblings<RDF> {
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
    type Output = Vec<Object>;

    fn parse_focused(&self, rdf: &mut RDF) -> Result<Self::Output, RDFError> {
        match rdf.get_focus() {
            Some(focus) => {
                let mut siblings = Vec::new();
                let maybe_disjoint = rdf.object_for(
                    focus,
                    &into_iri::<RDF>(sh_qualified_value_shapes_disjoint()),
                )?;
                if let Some(disjoint) = maybe_disjoint {
                    match disjoint {
                        Object::Literal(ConcreteLiteral::BooleanLiteral(true)) => {
                            trace!(
                                "QualifiedValueShapeSiblings: Focus node {focus} has disjoint=true"
                            );
                            let qvs = rdf
                                .objects_for(focus, &into_iri::<RDF>(sh_qualified_value_shape()))?;
                            if qvs.is_empty() {
                                trace!(
                                    "Focus node {focus} has disjoint=true but no qualifiedValueShape"
                                );
                            } else {
                                trace!("QVS of focus node {focus}: {qvs:?}");
                                let ps =
                                    rdf.subjects_for(&into_iri::<RDF>(sh_property()), focus)?;
                                trace!("Property parents of focus node {focus}: {ps:?}");
                                for property_parent in ps {
                                    let candidate_siblings = rdf.objects_for_shacl_path(
                                        &property_parent,
                                        &self.property_qualified_value_shape_path,
                                    )?;
                                    trace!("Candidate siblings: {candidate_siblings:?}");
                                    for sibling in candidate_siblings {
                                        if !qvs.contains(&sibling) {
                                            let sibling_node = RDF::term_as_object(&sibling)?;
                                            siblings.push(sibling_node);
                                        }
                                    }
                                }
                            }
                        }
                        Object::Literal(ConcreteLiteral::BooleanLiteral(false)) => {}
                        _ => {
                            trace!(
                                "Value of disjoint: {disjoint} is not boolean (Should we raise an error here?)"
                            );
                        }
                    }
                }
                Ok(siblings)
            }
            None => Err(RDFError::NoFocusNodeError),
        }
    }
}

fn build_qualified_shape(
    terms: HashSet<Object>,
    disjoint: Option<bool>,
    q_min_count: Option<isize>,
    q_max_count: Option<isize>,
    siblings: Vec<Object>,
) -> Vec<Component> {
    let mut result = Vec::new();
    for term in terms {
        let shape = Component::QualifiedValueShape {
            shape: term.clone(),
            q_min_count,
            q_max_count,
            disjoint,
            siblings: siblings.clone(),
        };
        result.push(shape);
    }
    result
}

fn cnv_node<RDF>(t: RDF::Term) -> Result<Component, RDFError>
where
    RDF: Rdf,
{
    let shape = RDF::term_as_object(&t).map_err(|_| RDFError::FailedTermToRDFNodeError {
        term: t.to_string(),
    })?;
    Ok(Component::Node { shape })
}

fn cnv_not<RDF>(t: RDF::Term) -> Result<Component, RDFError>
where
    RDF: Rdf,
{
    let shape = RDF::term_as_object(&t).map_err(|_| RDFError::FailedTermToRDFNodeError {
        term: t.to_string(),
    })?;
    Ok(Component::Not { shape })
}

fn parse_or_values<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Component> {
    ListParser::new().flat_map(|ls| cnv_or_list::<RDF>(ls))
}

fn cnv_or_list<RDF: Rdf>(ls: Vec<RDF::Term>) -> Result<Component, RDFError> {
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

fn terms_as_nodes<RDF: Rdf>(terms: Vec<RDF::Term>) -> Result<Vec<Object>, RDFError> {
    terms
        .into_iter()
        .map(|t| {
            let term_name = t.to_string();
            RDF::term_as_object(&t)
                .map_err(|_| RDFError::FailedTermToRDFNodeError { term: term_name })
        })
        .collect()
}

fn subjects_as_nodes<RDF: Rdf>(subjs: HashSet<RDF::Subject>) -> Result<Vec<Object>, RDFError> {
    subjs
        .into_iter()
        .map(|s| {
            RDF::subject_as_node(&s).map_err(|_| RDFError::FailedSubjectToRDFNodeError {
                subject: s.to_string(),
            })
        })
        .collect()
}

/// Parses the property value of the focus node as a SHACL path
fn path<RDF>() -> impl RDFNodeParse<RDF, Output = SHACLPath>
where
    RDF: FocusRDF,
{
    SingleValuePropertyParser::new(sh_path().clone()) 
        .then(|path_term| ShaclPathParser::new(path_term))
}

fn targets<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target<RDF>>>
where
    RDF: FocusRDF,
{
    let others: Vec<Box<dyn RDFNodeParse<RDF, Output = Vec<Target<RDF>>>>> = vec![
        Box::new(targets_node()),
        Box::new(targets_implicit_class()),
        Box::new(targets_subjects_of()),
        Box::new(targets_objects_of()),
    ];

    Box::new(targets_class()).combine_many(others)
}

fn closed<RDF>() -> impl RDFNodeParse<RDF, Output = bool>
where
    RDF: FocusRDF,
{
    SingleBoolPropertyParser::new(sh_closed().clone())
}

fn min_count<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    IntegersPropertyParser::new(sh_min_count().clone())
        .map(|ns| ns.iter().map(|n| Component::MinCount(*n)).collect())
}

fn max_count<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    IntegersPropertyParser::new(sh_max_count().clone())
        .map(|ns| ns.iter().map(|n| Component::MaxCount(*n)).collect())
}

fn min_length<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    IntegersPropertyParser::new(sh_min_length().clone())
        .map(|ns| ns.iter().map(|n| Component::MinLength(*n)).collect())
}

fn deactivated<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    BoolsPropertyParser::new(sh_deactivated().clone())
        .map(|ns| ns.iter().map(|n| Component::Deactivated(*n)).collect())
}

fn reifier_shape<RDF>() -> impl RDFNodeParse<RDF, Output = Option<ReifierInfo>>
where
    RDF: FocusRDF,
{
    ValuesPropertyParser::new(sh_reifier_shape().clone()).then(move |vs| {
        SingleBoolPropertyParser::new(sh_reification_required().clone())
            .optional()
            .map(move |requires_reifier| {
                let reifier_shape = vs
                    .iter()
                    .filter_map(|v| RDF::term_as_object(v).ok())
                    .collect();
                if vs.is_empty() {
                    None
                } else {
                    Some(ReifierInfo::new(
                        requires_reifier.unwrap_or(false),
                        reifier_shape,
                    ))
                }
            })
    })
}

fn closed_component<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    closed().optional().then(move |maybe_closed| {
        ignored_properties()
            .map(move |is| maybe_closed.map_or(vec![], |b| vec![Component::closed(b, is)]))
    })
}

fn ignored_properties<RDF>() -> impl RDFNodeParse<RDF, Output = HashSet<IriS>>
where
    RDF: FocusRDF,
{
    SingleValuePropertyAsListParser::new(sh_ignored_properties().clone())
        .optional()
        .flat_map(|is| match is {
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
                        return Err(RDFError::ExpectedIRIError {
                            term: v.to_string(),
                        });
                    }
                }
                Ok(hs)
            }
        })
}

fn min_inclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    LiteralsPropertyParser::new(sh_min_inclusive().clone()).map(|ns| {
        ns.iter()
            .map(|lit| Component::MinInclusive(lit.clone()))
            .collect()
    })
}

fn min_exclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    LiteralsPropertyParser::new(sh_min_exclusive().clone()).map(|ns| {
        ns.iter()
            .map(|lit| Component::MinExclusive(lit.clone()))
            .collect()
    })
}

fn max_inclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    LiteralsPropertyParser::new(sh_max_inclusive().clone()).map(|ns| {
        ns.iter()
            .map(|lit| Component::MaxInclusive(lit.clone()))
            .collect()
    })
}

fn max_exclusive<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    LiteralsPropertyParser::new(sh_max_exclusive().clone()).map(|ns| {
        ns.iter()
            .map(|lit| Component::MaxExclusive(lit.clone()))
            .collect()
    })
}

fn equals<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    IrisPropertyParser::new(sh_equals().clone()).map(|ns| {
        ns.iter()
            .map(|n| {
                let iri: IriRef = IriRef::iri(n.clone());
                Component::Equals(iri)
            })
            .collect()
    })
}

fn disjoint<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    IrisPropertyParser::new(sh_disjoint().clone()).map(|ns| {
        ns.iter()
            .map(|n| {
                let iri: IriRef = IriRef::iri(n.clone());
                Component::Disjoint(iri)
            })
            .collect()
    })
}

fn less_than<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    IrisPropertyParser::new(sh_less_than().clone()).map(|ns| {
        ns.iter()
            .map(|n| {
                let iri: IriRef = IriRef::iri(n.clone());
                Component::LessThan(iri)
            })
            .collect()
    })
}

fn less_than_or_equals<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    IrisPropertyParser::new(sh_less_than_or_equals().clone()).map(|ns| {
        ns.iter()
            .map(|n| {
                let iri: IriRef = IriRef::iri(n.clone());
                Component::LessThanOrEquals(iri)
            })
            .collect()
    })
}

fn max_length<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    IntegersPropertyParser::new(sh_max_length().clone())
        .map(|ns| ns.iter().map(|n| Component::MaxLength(*n)).collect())
}

fn datatype<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    IrisPropertyParser::new(sh_datatype().clone()).map(|ns| {
        ns.iter()
            .map(|iri| Component::Datatype(IriRef::iri(iri.clone())))
            .collect()
    })
}

fn class<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    ObjectsPropertyParser::new(sh_class().clone())
        .map(|ns| ns.iter().map(|n| Component::Class(n.clone())).collect())
}

fn node_kind<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    ValuesPropertyParser::new(sh_node_kind().clone()).flat_map(|ns| {
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
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_has_value(), parse_has_value_values())
}

fn in_component<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_in(), parse_in_values())
}

fn language_in<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    parse_components_for_iri(sh_language_in(), parse_language_in_values())
}

fn pattern<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>> {
    flags().optional().then(move |maybe_flags| {
        StringsPropertyParser::new(sh_pattern().clone()).flat_map(move |strs| match strs.len() {
            0 => Ok(Vec::new()),
            1 => {
                let pattern = strs.first().unwrap().clone();
                let flags = maybe_flags.clone();
                Ok(vec![Component::Pattern { pattern, flags }])
            }
            _ => todo!(), // Error...
        })
    })
}

fn flags<RDF: FocusRDF>() -> impl RDFNodeParse<RDF, Output = String> {
    SingleStringPropertyParser::new(sh_flags().clone())
}

fn parse_in_values<RDF>() -> impl RDFNodeParse<RDF, Output = Component>
where
    RDF: FocusRDF,
{
    ListParser::new().flat_map(cnv_in_list::<RDF>)
}

fn parse_has_value_values<RDF>() -> impl RDFNodeParse<RDF, Output = Component>
where
    RDF: FocusRDF,
{
    TermParser::new().flat_map(|t| cnv_has_value::<RDF>(t))
}

fn parse_language_in_values<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Component> {
    ListParser::new().flat_map(cnv_language_in_list::<R>)
}

fn cnv_has_value<RDF>(term: RDF::Term) -> Result<Component, RDFError>
where
    RDF: Rdf,
{
    let value = term_to_value::<RDF>(&term, "parsing hasValue")?;
    Ok(Component::HasValue { value })
}

fn cnv_language_in_list<R: FocusRDF>(terms: Vec<R::Term>) -> Result<Component, RDFError> {
    let langs: Vec<Lang> = terms.iter().flat_map(R::term_as_lang).collect();
    Ok(Component::LanguageIn { langs })
}

fn term_to_value<RDF>(term: &RDF::Term, msg: &str) -> Result<Value, RDFError>
where
    RDF: Rdf,
{
    if term.is_blank_node() {
        Err(RDFError::ExpectedIriOrBlankNodeError {
            term: term.to_string(),
            error: msg.to_string(),
        })
    } else if let Ok(iri) = RDF::term_as_iri(term) {
        let iri: RDF::IRI = iri;
        let iri_string = iri.as_str();
        let iri_s = IriS::new_unchecked(iri_string);
        Ok(Value::Iri(IriRef::Iri(iri_s)))
    } else if let Ok(literal) = RDF::term_as_literal(term) {
        let literal: RDF::Literal = literal;
        let slit: ConcreteLiteral =
            literal
                .clone()
                .try_into()
                .map_err(|_e| RDFError::LiteralAsSLiteral {
                    literal: (literal.to_string()),
                })?;
        Ok(Value::Literal(slit))
    } else {
        println!("Unexpected code in term_to_value: {term}: {msg}");
        todo!()
    }
}

fn cnv_in_list<RDF>(ls: Vec<RDF::Term>) -> Result<Component, RDFError>
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
    component_parser.map_property(iri.clone())
}

fn or<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_or(), parse_or_values())
}

fn xone<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_xone(), parse_xone_values())
}

fn and<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_and(), parse_and_values())
}

fn not_component<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_not(), parse_not_value())
}

fn node<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
// impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    parse_components_for_iri(sh_node(), parse_node_value())
}

fn qualified_value_shape<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    ObjectsPropertyParser::new(sh_qualified_value_shape().clone()).then(|qvs: Vec<Object>| {
        let set: HashSet<Object> = qvs.into_iter().collect();
        parse_qualified_value_shape::<RDF>(set)
    })
}

fn term_to_node_kind<RDF>(term: &RDF::Term) -> Result<NodeKind, ShaclParserError>
where
    RDF: Rdf,
{
    let term_name = term.to_string();
    let result_iri: Result<RDF::IRI, ShaclParserError> =
        <RDF::Term as TryInto<RDF::IRI>>::try_into(term.clone()).map_err(|_| {
            ShaclParserError::ExpectedNodeKind {
                term: term_name.to_string(),
            }
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

fn targets_class<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target<RDF>>>
where
    RDF: FocusRDF,
{
    IrisPropertyParser::new(sh_target_class().clone()).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|iri| Target::TargetClass(Object::iri(iri)))
            .collect();
        Ok(result)
    })
}

fn targets_node<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Target<RDF>>>
where
    RDF: FocusRDF,
{
    ObjectsPropertyParser::new(sh_target_node().clone()).flat_map(move |ts| {
        let result = ts.into_iter().map(Target::TargetNode).collect();
        Ok(result)
    })
}

fn targets_implicit_class<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target<R>>> {
    InstancesParser::new(rdfs_class().clone())
        .and(InstancesParser::new(sh_property_shape().clone()))
        .and(InstancesParser::new(sh_node_shape().clone()))
        .and(FocusParser::new())
        .flat_map(
            move |(((class, property_shapes), node_shapes), focus): (_, R::Term)| {
                let result: std::result::Result<Vec<Target<R>>, RDFError> = class
                    .into_iter()
                    .filter(|t: &R::Subject| property_shapes.contains(t) || node_shapes.contains(t))
                    .map(Into::into)
                    .filter(|t: &R::Term| t.clone() == focus)
                    .map(|t: R::Term| {
                        let t_name = t.to_string();
                        let obj = t
                            .clone()
                            .try_into()
                            .map_err(|_| RDFError::FailedTermToRDFNodeError { term: t_name })?;
                        Ok(Target::TargetImplicitClass(obj))
                    })
                    .collect();
                let ts = result?;
                Ok(ts)
            },
        )
}

fn targets_objects_of<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target<R>>> {
    IrisPropertyParser::new(sh_target_objects_of().clone()).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|t: IriS| Target::TargetObjectsOf(t.into()))
            .collect();
        Ok(result)
    })
}

fn targets_subjects_of<R: FocusRDF>() -> impl RDFNodeParse<R, Output = Vec<Target<R>>> {
    IrisPropertyParser::new(sh_target_subjects_of().clone()).flat_map(move |ts| {
        let result = ts
            .into_iter()
            .map(|t: IriS| Target::TargetSubjectsOf(t.into()))
            .collect();
        Ok(result)
    })
}

fn unique_lang<RDF>() -> impl RDFNodeParse<RDF, Output = Vec<Component>>
where
    RDF: FocusRDF,
{
    BoolsPropertyParser::new(sh_unique_lang().clone())
        .map(|ns| ns.iter().map(|n| Component::UniqueLang(*n)).collect())
}

fn into_iri<RDF: Rdf>(iri: &IriS) -> RDF::IRI {
    iri.clone().into()
}

#[cfg(test)]
mod tests {
    use super::ShaclParser;
    use iri_s::IriS;
    use rdf::rdf_core::{
        RDFFormat,
        term::{Object, literal::Lang},
    };
    use rdf::rdf_impl::{InMemoryGraph, ReaderMode};
    use shacl_ast::shape::Shape;

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

        let graph = InMemoryGraph::from_str(shape, &rdf_format, None, &reader_mode).unwrap();
        let schema = ShaclParser::new(graph).parse().unwrap();
        let shape = match schema.get_shape(&shape_id).unwrap() {
            Shape::NodeShape(ns) => ns,
            _ => panic!("Shape is not a NodeShape"),
        };

        match shape.components().first().unwrap() {
            crate::rdf_to_shacl::shacl_parser::Component::LanguageIn { langs } => {
                assert_eq!(langs.len(), 2);
                assert_eq!(langs[0], Lang::new("en").unwrap());
                assert_eq!(langs[1], Lang::new("fr").unwrap());
            }
            _ => panic!("Shape has not a LanguageIn component"),
        }
    }
}
