use crate::ast::{ASTComponent, ASTSchema};
use crate::ir::components::{
    And, Class, Closed, Datatype, Deactivated, Disjoint, Equals, HasValue, In, LanguageIn, LessThan, LessThanOrEquals,
    MaxCount, MaxExclusive, MaxInclusive, MaxLength, MinCount, MinExclusive, MinInclusive, MinLength, Node, Nodekind,
    Not, Or, Pattern, QualifiedValueShape, UniqueLang, Xone,
};
use crate::ir::dg::{DependencyGraph, PosNeg};
use crate::ir::error::IRError;
use crate::ir::schema::IRSchema;
use crate::ir::shape::IRShape;
use crate::ir::shape_label_idx::ShapeLabelIdx;
use crate::ir::{convert_iri_ref, convert_value};
use crate::types::NodeKind;
use iri_s::IriS;
use itertools::Itertools;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum IRComponent {
    Class(Class),
    Datatype(Datatype),
    NodeKind(Nodekind),
    MinCount(MinCount),
    MaxCount(MaxCount),
    MinExclusive(MinExclusive),
    MaxExclusive(MaxExclusive),
    MinInclusive(MinInclusive),
    MaxInclusive(MaxInclusive),
    MinLength(MinLength),
    MaxLength(MaxLength),
    Pattern(Pattern),
    UniqueLang(UniqueLang),
    LanguageIn(LanguageIn),
    Equals(Equals),
    Disjoint(Disjoint),
    LessThan(LessThan),
    LessThanOrEquals(LessThanOrEquals),
    Or(Or),
    And(And),
    Not(Not),
    Xone(Xone),
    Node(Node),
    HasValue(HasValue),
    In(In),
    QualifiedValueShape(QualifiedValueShape),
    Closed(Closed),
    Deactivated(Deactivated),
}

impl IRComponent {
    /// Compiles an AST SHACL component to an IR SHACL Component
    /// It returns None for components that are not represented in the IR,
    /// such as sh:closed and sh:deactivated.
    /// It returns a vector of (PosNeg, ShapeLabelIdx) pairs for components that are represented in the IR.
    /// The vector is list of dependant shapes for cases with recursion
    // TODO - Update comment to match current behaviour
    pub fn compile(component: &ASTComponent, ast: &ASTSchema, ir: &mut IRSchema) -> Result<Self, IRError> {
        let result = match component.clone() {
            ASTComponent::Class(object) => IRComponent::Class(Class::new(object)),
            ASTComponent::Datatype(iri) => IRComponent::Datatype(Datatype::new(convert_iri_ref(iri)?)),
            ASTComponent::NodeKind(nk) => IRComponent::NodeKind(Nodekind::new(nk)),
            ASTComponent::MinCount(n) => IRComponent::MinCount(MinCount::new(n)),
            ASTComponent::MaxCount(n) => IRComponent::MaxCount(MaxCount::new(n)),
            ASTComponent::MinExclusive(lit) => IRComponent::MinExclusive(MinExclusive::new(lit)),
            ASTComponent::MaxExclusive(lit) => IRComponent::MaxExclusive(MaxExclusive::new(lit)),
            ASTComponent::MinInclusive(lit) => IRComponent::MinInclusive(MinInclusive::new(lit)),
            ASTComponent::MaxInclusive(lit) => IRComponent::MaxInclusive(MaxInclusive::new(lit)),
            ASTComponent::MinLength(l) => IRComponent::MinLength(MinLength::new(l)),
            ASTComponent::MaxLength(l) => IRComponent::MaxLength(MaxLength::new(l)),
            ASTComponent::Pattern { pattern, flags } => {
                let pattern = Pattern::new(pattern, flags)?;
                IRComponent::Pattern(pattern)
            },
            ASTComponent::UniqueLang(lang) => IRComponent::UniqueLang(UniqueLang::new(lang)),
            ASTComponent::LanguageIn(langs) => IRComponent::LanguageIn(LanguageIn::new(langs)),
            ASTComponent::Equals(iri) => IRComponent::Equals(Equals::new(convert_iri_ref(iri)?)),
            ASTComponent::Disjoint(iri) => IRComponent::Disjoint(Disjoint::new(convert_iri_ref(iri)?)),
            ASTComponent::LessThan(iri) => IRComponent::LessThan(LessThan::new(convert_iri_ref(iri)?)),
            ASTComponent::LessThanOrEquals(iri) => {
                IRComponent::LessThanOrEquals(LessThanOrEquals::new(convert_iri_ref(iri)?))
            },
            ASTComponent::Or(objs) => {
                let idxs = ir.register_shapes(objs, ast)?;
                IRComponent::Or(Or::new(idxs))
            },
            ASTComponent::And(objs) => {
                let idxs = ir.register_shapes(objs, ast)?;
                IRComponent::And(And::new(idxs))
            },
            ASTComponent::Not(obj) => {
                let idx = ir.register_shape(&obj, None, ast)?;
                IRComponent::Not(Not::new(idx))
            },
            ASTComponent::Xone(objs) => {
                let idxs = ir.register_shapes(objs, ast)?;
                IRComponent::Xone(Xone::new(idxs))
            },
            ASTComponent::Closed {
                is_closed,
                ignored_properties,
            } => IRComponent::Closed(Closed::new(is_closed, ignored_properties.into_iter().collect_vec())),
            ASTComponent::Node(obj) => {
                let idx = ir.register_shape(&obj, None, ast)?;
                IRComponent::Node(Node::new(idx))
            },
            ASTComponent::HasValue(val) => {
                let term = convert_value(val)?;
                IRComponent::HasValue(HasValue::new(term))
            },
            ASTComponent::In(vals) => {
                let terms = vals.into_iter().map(convert_value).collect::<Result<Vec<_>, _>>()?;
                IRComponent::In(In::new(terms))
            },
            ASTComponent::QualifiedValueShape {
                shape,
                q_min_count,
                q_max_count,
                disjoint,
                siblings,
            } => {
                let idx = ir.register_shape(&shape, None, ast)?;
                let compiled_siblings = ir.register_shapes(siblings, ast)?;

                IRComponent::QualifiedValueShape(QualifiedValueShape::new(
                    idx,
                    q_min_count,
                    q_max_count,
                    disjoint,
                    compiled_siblings,
                ))
            },
            ASTComponent::Deactivated(d) => {
                // TODO - Change for node expr
                IRComponent::Deactivated(Deactivated::new(d))
            },
        };

        Ok(result)
    }
}

impl IRComponent {
    // TODO - Add closed and deactivated
    pub fn register<RDF: BuildRDF>(
        &self,
        id: &Object,
        graph: &mut RDF,
        shape_map: &HashMap<ShapeLabelIdx, IRShape>,
    ) -> Result<(), RDF::Err> {
        match self {
            IRComponent::Class(c) => register_term(&c.class_rule().clone().into(), ShaclVocab::sh_class(), id, graph),
            IRComponent::Datatype(iri) => register_iri(iri.datatype(), ShaclVocab::sh_datatype(), id, graph),
            IRComponent::NodeKind(nk) => {
                let iri = match nk.node_kind() {
                    NodeKind::Iri => ShaclVocab::sh_iri_ref(),
                    _ => unimplemented!(),
                };
                register_iri(iri, ShaclVocab::sh_datatype(), id, graph)
            },
            IRComponent::MinCount(mc) => {
                register_integer(mc.min_count() as isize, ShaclVocab::sh_min_count(), id, graph)
            },
            IRComponent::MaxCount(mc) => {
                register_integer(mc.max_count() as isize, ShaclVocab::sh_max_count(), id, graph)
            },
            IRComponent::MinExclusive(me) => {
                register_literal(me.min_exclusive(), ShaclVocab::sh_min_exclusive(), id, graph)
            },
            IRComponent::MaxExclusive(me) => {
                register_literal(me.max_exclusive(), ShaclVocab::sh_max_exclusive(), id, graph)
            },
            IRComponent::MinInclusive(mi) => {
                register_literal(mi.min_inclusive(), ShaclVocab::sh_min_inclusive(), id, graph)
            },
            IRComponent::MaxInclusive(mi) => {
                register_literal(mi.max_inclusive(), ShaclVocab::sh_max_inclusive(), id, graph)
            },
            IRComponent::MinLength(ml) => register_integer(ml.min_length(), ShaclVocab::sh_min_length(), id, graph),
            IRComponent::MaxLength(ml) => register_integer(ml.max_length(), ShaclVocab::sh_max_length(), id, graph),
            IRComponent::Pattern(p) => {
                if let Some(flags) = p.flags() {
                    register_literal(&ConcreteLiteral::str(flags), ShaclVocab::sh_flags(), id, graph)?;
                }
                register_literal(&ConcreteLiteral::str(p.pattern()), ShaclVocab::sh_pattern(), id, graph)
            },
            IRComponent::UniqueLang(ul) => register_boolean(ul.unique_lang(), ShaclVocab::sh_unique_lang(), id, graph),
            IRComponent::LanguageIn(li) => li.langs().iter().try_for_each(|l| {
                register_literal(
                    &ConcreteLiteral::str(&l.to_string()),
                    ShaclVocab::sh_language_in(),
                    id,
                    graph,
                )
            }),
            IRComponent::Equals(eq) => register_iri(eq.iri(), ShaclVocab::sh_equals(), id, graph),
            IRComponent::Disjoint(d) => register_iri(d.iri(), ShaclVocab::sh_disjoint(), id, graph),
            IRComponent::LessThan(lt) => register_iri(lt.iri(), ShaclVocab::sh_less_than(), id, graph),
            IRComponent::LessThanOrEquals(lte) => {
                register_iri(lte.iri(), ShaclVocab::sh_less_than_or_equals(), id, graph)
            },
            IRComponent::Or(or) => {
                or.shapes().iter().try_for_each(|idx| {
                    // TODO - Throw error instead of unwrap
                    let shape = shape_map.get(idx).unwrap();
                    register_term(&shape.id().clone().into(), ShaclVocab::sh_or(), id, graph)
                })
            },
            IRComponent::And(and) => {
                and.shapes().iter().try_for_each(|idx| {
                    // TODO - Throw error instead of unwrap
                    let shape = shape_map.get(idx).unwrap();
                    register_term(&shape.id().clone().into(), ShaclVocab::sh_and(), id, graph)
                })
            },
            IRComponent::Not(not) => register_term(
                // TODO - Throw error instead of unwrap
                &shape_map.get(not.shape()).unwrap().id().clone().into(),
                ShaclVocab::sh_not(),
                id,
                graph,
            ),
            IRComponent::Xone(xone) => {
                xone.shapes().iter().try_for_each(|idx| {
                    // TODO - Throw error instead of unwrap
                    let shape = shape_map.get(idx).unwrap();
                    register_term(&shape.id().clone().into(), ShaclVocab::sh_xone(), id, graph)
                })
            },
            IRComponent::Node(n) => register_term(
                // TODO - Throw error instead of unwrap
                &shape_map.get(n.shape()).unwrap().id().clone().into(),
                ShaclVocab::sh_node(),
                id,
                graph,
            ),
            IRComponent::HasValue(hv) => match hv.value() {
                Object::Iri(iri) => register_iri(iri, ShaclVocab::sh_has_value(), id, graph),
                Object::Literal(lit) => register_literal(lit, ShaclVocab::sh_has_value(), id, graph),
                _ => unreachable!(),
            },
            IRComponent::In(i) => {
                // TODO - Review this code
                i.values().iter().try_for_each(|v| match v {
                    Object::Iri(iri) => register_iri(iri, ShaclVocab::sh_in(), id, graph),
                    Object::Literal(lit) => register_literal(lit, ShaclVocab::sh_in(), id, graph),
                    _ => unreachable!(),
                })
            },
            IRComponent::QualifiedValueShape(qvs) => {
                if let Some(value) = qvs.qualified_min_count() {
                    register_integer(value, ShaclVocab::sh_qualified_min_count(), id, graph)?;
                }

                if let Some(value) = qvs.qualified_max_count() {
                    register_integer(value, ShaclVocab::sh_qualified_max_count(), id, graph)?;
                }

                if let Some(value) = qvs.qualified_value_shapes_disjoint() {
                    register_boolean(value, ShaclVocab::sh_qualified_value_shapes_disjoint(), id, graph)?;
                }

                // TODO - Throw error instead of unwrap
                let shape = shape_map.get(qvs.shape()).unwrap();
                register_term(
                    &shape.id().clone().into(),
                    ShaclVocab::sh_qualified_value_shape(),
                    id,
                    graph,
                )
            },
            IRComponent::Closed(closed) => {
                register_boolean(closed.is_closed(), ShaclVocab::sh_closed(), id, graph)?;

                closed
                    .ignored_properties()
                    .iter()
                    .try_for_each(|iri| register_iri(iri, ShaclVocab::sh_ignored_properties(), id, graph))
            },
            IRComponent::Deactivated(deactivated) => {
                // TODO - Adapt for node expression
                register_boolean(deactivated.is_deactivated(), ShaclVocab::sh_deactivated(), id, graph)
            },
        }
    }
}

impl IRComponent {
    pub fn add_edges(
        &self,
        idx: ShapeLabelIdx,
        dg: &mut DependencyGraph,
        posneg: PosNeg,
        ir: &IRSchema,
        cache: &mut HashSet<ShapeLabelIdx>,
    ) {
        match self {
            IRComponent::Class(_) => {},
            IRComponent::Datatype(_) => {},
            IRComponent::NodeKind(_) => {},
            IRComponent::MinCount(_) => {},
            IRComponent::MaxCount(_) => {},
            IRComponent::MinExclusive(_) => {},
            IRComponent::MaxExclusive(_) => {},
            IRComponent::MinInclusive(_) => {},
            IRComponent::MaxInclusive(_) => {},
            IRComponent::MinLength(_) => {},
            IRComponent::MaxLength(_) => {},
            IRComponent::Pattern(_) => {},
            IRComponent::UniqueLang(_) => {},
            IRComponent::LanguageIn(_) => {},
            IRComponent::Equals(_) => {},
            IRComponent::Disjoint(_) => {},
            IRComponent::LessThan(_) => {},
            IRComponent::LessThanOrEquals(_) => {},
            IRComponent::Or(or) => {
                for shape_idx in or.shapes() {
                    if let Some(shape) = ir.get_shape_from_idx(shape_idx) {
                        dg.add_edge(idx, *shape_idx, posneg);
                        if cache.contains(shape_idx) {
                            continue;
                        }
                        cache.insert(*shape_idx);
                        shape.add_edges(*shape_idx, dg, posneg, ir, cache);
                    }
                }
            },
            IRComponent::And(and) => {
                for shape_idx in and.shapes() {
                    if let Some(shape) = ir.get_shape_from_idx(shape_idx) {
                        dg.add_edge(idx, *shape_idx, posneg);
                        if cache.contains(shape_idx) {
                            continue;
                        }
                        cache.insert(*shape_idx);
                        shape.add_edges(*shape_idx, dg, posneg, ir, cache);
                    }
                }
            },
            IRComponent::Not(not) => {
                let shape_idx = not.shape();
                if let Some(shape) = ir.get_shape_from_idx(shape_idx) {
                    dg.add_edge(idx, *shape_idx, posneg.change());
                    if !cache.contains(shape_idx) {
                        cache.insert(*shape_idx);
                        shape.add_edges(*shape_idx, dg, posneg.change(), ir, cache);
                    }
                }
            },
            IRComponent::Xone(xone) => {
                for shape_idx in xone.shapes() {
                    if let Some(shape) = ir.get_shape_from_idx(shape_idx) {
                        dg.add_edge(idx, *shape_idx, posneg);
                        if cache.contains(shape_idx) {
                            continue;
                        }
                        cache.insert(*shape_idx);
                        shape.add_edges(*shape_idx, dg, posneg, ir, cache);
                    }
                }
            },
            IRComponent::Node(node) => {
                let shape_idx = node.shape();
                if let Some(shape) = ir.get_shape_from_idx(shape_idx) {
                    dg.add_edge(idx, *shape_idx, posneg);
                    if !cache.contains(shape_idx) {
                        cache.insert(*shape_idx);
                        shape.add_edges(*shape_idx, dg, posneg, ir, cache);
                    }
                }
            },
            IRComponent::HasValue(_) => {},
            IRComponent::In(_) => {},
            IRComponent::QualifiedValueShape(qvs) => {
                dg.add_edge(idx, *qvs.shape(), posneg);
                // for sibling in qvs.siblings() {
                //     dg.add_edge(idx, *sibling, posneg);
                // }
            },
            IRComponent::Closed(_) => {},
            IRComponent::Deactivated(_) => {},
        }
    }
}

fn register_integer<RDF: BuildRDF>(
    value: isize,
    predicate: IriS,
    node: &Object,
    graph: &mut RDF,
) -> Result<(), RDF::Err> {
    let value: i128 = value.try_into().unwrap();
    let literal: RDF::Literal = value.into();
    register_term(&literal.into(), predicate, node, graph)
}

fn register_boolean<RDF: BuildRDF>(
    value: bool,
    predicate: IriS,
    node: &Object,
    graph: &mut RDF,
) -> Result<(), RDF::Err> {
    let literal: RDF::Literal = value.into();
    register_term(&literal.into(), predicate, node, graph)
}

fn register_literal<RDF: BuildRDF>(
    value: &ConcreteLiteral,
    predicate: IriS,
    node: &Object,
    graph: &mut RDF,
) -> Result<(), RDF::Err> {
    let literal: RDF::Literal = value.lexical_form().into();
    register_term(&literal.into(), predicate, node, graph)
}

fn register_iri<RDF: BuildRDF>(value: &IriS, predicate: IriS, node: &Object, graph: &mut RDF) -> Result<(), RDF::Err> {
    register_term(&value.clone().into(), predicate, node, graph)
}

fn register_term<RDF: BuildRDF>(
    value: &RDF::Term,
    predicate: IriS,
    node: &Object,
    graph: &mut RDF,
) -> Result<(), RDF::Err> {
    let node: RDF::Subject = node.clone().try_into().map_err(|_| unreachable!())?;
    graph.add_triple(node, predicate, value.clone())
}

impl From<&IRComponent> for IriS {
    fn from(value: &IRComponent) -> Self {
        match value {
            IRComponent::Class(_) => ShaclVocab::sh_class_constraint_component(),
            IRComponent::Datatype(_) => ShaclVocab::sh_datatype_constraint_component(),
            IRComponent::NodeKind(_) => ShaclVocab::sh_node_kind_constraint_component(),
            IRComponent::MinCount(_) => ShaclVocab::sh_min_count_constraint_component(),
            IRComponent::MaxCount(_) => ShaclVocab::sh_max_count_constraint_component(),
            IRComponent::MinExclusive(_) => ShaclVocab::sh_min_exclusive_constraint_component(),
            IRComponent::MaxExclusive(_) => ShaclVocab::sh_max_exclusive_constraint_component(),
            IRComponent::MinInclusive(_) => ShaclVocab::sh_min_inclusive_constraint_component(),
            IRComponent::MaxInclusive(_) => ShaclVocab::sh_max_inclusive_constraint_component(),
            IRComponent::MinLength(_) => ShaclVocab::sh_min_length_constraint_component(),
            IRComponent::MaxLength(_) => ShaclVocab::sh_max_length_constraint_component(),
            IRComponent::Pattern(_) => ShaclVocab::sh_pattern_constraint_component(),
            IRComponent::UniqueLang(_) => ShaclVocab::sh_unique_lang_constraint_component(),
            IRComponent::LanguageIn(_) => ShaclVocab::sh_language_in_constraint_component(),
            IRComponent::Equals(_) => ShaclVocab::sh_equals_constraint_component(),
            IRComponent::Disjoint(_) => ShaclVocab::sh_disjoint_constraint_component(),
            IRComponent::LessThan(_) => ShaclVocab::sh_less_than_constraint_component(),
            IRComponent::LessThanOrEquals(_) => ShaclVocab::sh_less_than_or_equals_constraint_component(),
            IRComponent::Or(_) => ShaclVocab::sh_or_constraint_component(),
            IRComponent::And(_) => ShaclVocab::sh_and_constraint_component(),
            IRComponent::Not(_) => ShaclVocab::sh_not_constraint_component(),
            IRComponent::Xone(_) => ShaclVocab::sh_xone_constraint_component(),
            IRComponent::Node(_) => ShaclVocab::sh_node_constraint_component(),
            IRComponent::HasValue(_) => ShaclVocab::sh_has_value_constraint_component(),
            IRComponent::In(_) => ShaclVocab::sh_in_constraint_component(),
            IRComponent::QualifiedValueShape(_) => ShaclVocab::sh_qualified_value_shape_constraint_component(),
            IRComponent::Closed(_) => ShaclVocab::sh_closed_constraint_component(),
            IRComponent::Deactivated(_) => ShaclVocab::sh_deactivated_constraint_component(),
        }
    }
}

impl Display for IRComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IRComponent::Class(cls) => write!(f, " {cls}"),
            IRComponent::Datatype(dt) => write!(f, " {dt}"),
            IRComponent::NodeKind(nk) => write!(f, " {nk}"),
            IRComponent::MinCount(n) => write!(f, " {n}"),
            IRComponent::MaxCount(mn) => write!(f, " {mn}"),
            IRComponent::MinExclusive(n) => write!(f, " {n}"),
            IRComponent::MaxExclusive(n) => write!(f, " {n}"),
            IRComponent::MinInclusive(n) => write!(f, " {n}"),
            IRComponent::MaxInclusive(n) => write!(f, " {n}"),
            IRComponent::MinLength(n) => write!(f, " {n}"),
            IRComponent::MaxLength(n) => write!(f, " {n}"),
            IRComponent::Pattern(pt) => write!(f, " {pt}"),
            IRComponent::UniqueLang(ul) => write!(f, " {ul}"),
            IRComponent::LanguageIn(l) => write!(f, " {l}"),
            IRComponent::Equals(e) => write!(f, " {e}"),
            IRComponent::Disjoint(p) => write!(f, " {p}"),
            IRComponent::LessThan(p) => write!(f, " {p}"),
            IRComponent::LessThanOrEquals(p) => write!(f, " {p}"),
            IRComponent::Or(or) => write!(f, " {or}"),
            IRComponent::And(and) => write!(f, " {and}"),
            IRComponent::Not(not) => write!(f, " {not}"),
            IRComponent::Xone(xone) => write!(f, " {xone}"),
            IRComponent::Node(n) => write!(f, " {n}"),
            IRComponent::HasValue(v) => write!(f, " HasValue({v})"),
            IRComponent::In(vs) => write!(f, " {vs}"),
            IRComponent::QualifiedValueShape(qvs) => write!(f, " {qvs}"),
            IRComponent::Closed(closed) => write!(f, "{closed}"),
            IRComponent::Deactivated(deactivated) => write!(f, "{deactivated}"),
        }
    }
}
