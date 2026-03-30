use crate::ast::{ASTComponent, ASTSchema};
use crate::ir::components::{And, Class, Datatype, Disjoint, Equals, HasValue, In, LanguageIn, LessThan, LessThanOrEquals, MaxCount, MaxExclusive, MaxInclusive, MaxLength, MinCount, MinExclusive, MinInclusive, MinLength, Node, Nodekind, Not, Or, Pattern, QualifiedValueShape, UniqueLang, Xone};
use crate::ir::dependency_graph::{DependencyGraph, PosNeg};
use crate::ir::error::IRError;
use crate::ir::schema::IRSchema;
use crate::ir::shape::IRShape;
use crate::ir::shape_label_idx::ShapeLabelIdx;
use crate::ir::{convert_iri_ref, convert_value};
use crate::types::NodeKind;
use iri_s::IriS;
use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::ShaclVocab;
use rudof_rdf::rdf_core::BuildRDF;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
// TODO - Add closed and deactivated
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
}

impl IRComponent {

    /// Compiles an AST SHACL component to an IR SHACL Component
    /// It returns None for components that are not represented in the IR,
    /// such as sh:closed and sh:deactivated.
    /// It returns a vector of (PosNeg, ShapeLabelIdx) pairs for components that are represented in the IR.
    /// The vector is list of dependant shapes for cases with recursion
    // TODO - Update comment to match current behaviour
    // TODO - Remove Option<Self> and replace with Self, so the serialization can be done and the node expr can be computed
    pub fn compile(component: &ASTComponent, ast: &ASTSchema, ir: &mut IRSchema) -> Result<Option<Self>, IRError> {
        let result = match component.clone() {
            ASTComponent::Class(object) => Some(IRComponent::Class(Class::new(object))),
            ASTComponent::Datatype(iri) => Some(IRComponent::Datatype(Datatype::new(convert_iri_ref(iri)?))),
            ASTComponent::NodeKind(nk) => Some(IRComponent::NodeKind(Nodekind::new(nk))),
            ASTComponent::MinCount(n) => Some(IRComponent::MinCount(MinCount::new(n))),
            ASTComponent::MaxCount(n) => Some(IRComponent::MaxCount(MaxCount::new(n))),
            ASTComponent::MinExclusive(lit) => Some(IRComponent::MinExclusive(MinExclusive::new(lit))),
            ASTComponent::MaxExclusive(lit) => Some(IRComponent::MaxExclusive(MaxExclusive::new(lit))),
            ASTComponent::MinInclusive(lit) => Some(IRComponent::MinInclusive(MinInclusive::new(lit))),
            ASTComponent::MaxInclusive(lit) => Some(IRComponent::MaxInclusive(MaxInclusive::new(lit))),
            ASTComponent::MinLength(l) => Some(IRComponent::MinLength(MinLength::new(l))),
            ASTComponent::MaxLength(l) => Some(IRComponent::MaxLength(MaxLength::new(l))),
            ASTComponent::Pattern { pattern, flags } => {
                let pattern = Pattern::new(pattern, flags)?;
                Some(IRComponent::Pattern(pattern))
            },
            ASTComponent::UniqueLang(lang) => Some(IRComponent::UniqueLang(UniqueLang::new(lang))),
            ASTComponent::LanguageIn(langs) => Some(IRComponent::LanguageIn(LanguageIn::new(langs))),
            ASTComponent::Equals(iri) => Some(IRComponent::Equals(Equals::new(convert_iri_ref(iri)?))),
            ASTComponent::Disjoint(iri) => Some(IRComponent::Disjoint(Disjoint::new(convert_iri_ref(iri)?))),
            ASTComponent::LessThan(iri) => Some(IRComponent::LessThan(LessThan::new(convert_iri_ref(iri)?))),
            ASTComponent::LessThanOrEquals(iri) => Some(IRComponent::LessThanOrEquals(LessThanOrEquals::new(convert_iri_ref(iri)?))),
            ASTComponent::Or(objs) => {
                let idxs = ir.register_shapes(objs, ast)?;
                Some(IRComponent::Or(Or::new(idxs)))
            },
            ASTComponent::And(objs) => {
                let idxs = ir.register_shapes(objs, ast)?;
                Some(IRComponent::And(And::new(idxs)))
            }
            ASTComponent::Not(obj) => {
                let idx = ir.register_shape(&obj, None, ast)?;
                Some(IRComponent::Not(Not::new(idx)))
            }
            ASTComponent::Xone(objs) => {
                let idxs = ir.register_shapes(objs, ast)?;
                Some(IRComponent::Xone(Xone::new(idxs)))
            }
            ASTComponent::Closed { .. } => None, // TODO - Change for serialization
            ASTComponent::Node(obj) => {
                let idx = ir.register_shape(&obj, None, ast)?;
                Some(IRComponent::Node(Node::new(idx)))
            }
            ASTComponent::HasValue(val) => {
                let term = convert_value(val)?;
                Some(IRComponent::HasValue(HasValue::new(term)))
            }
            ASTComponent::In(vals) => {
                let terms = vals.into_iter().map(convert_value).collect::<Result<Vec<_>, _>>()?;
                Some(IRComponent::In(In::new(terms)))
            }
            ASTComponent::QualifiedValueShape {
                shape,
                q_min_count,
                q_max_count,
                disjoint,
                siblings
            } => {
                let idx = ir.register_shape(&shape, None, ast)?;
                let compiled_siblings = ir.register_shapes(siblings, ast)?;

                Some(IRComponent::QualifiedValueShape(QualifiedValueShape::new(
                    idx,
                    q_min_count,
                    q_max_count,
                    disjoint,
                    compiled_siblings
                )))
            }
            ASTComponent::Deactivated(_) => None, // TODO - Change for node expr
        };

        Ok(result)
    }
}

impl IRComponent {

    // TODO - Add closed and deactivated
    pub fn register<RDF: BuildRDF>(&self, id: &Object, graph: &mut RDF, shape_map: &HashMap<ShapeLabelIdx, IRShape>) -> Result<(), RDF::Err> {
        match self {
            IRComponent::Class(c) => register_term(&c.class_rule().clone().into(), ShaclVocab::sh_class().clone(), id, graph),
            IRComponent::Datatype(iri) => register_iri(iri.datatype(), ShaclVocab::sh_datatype().clone(), id, graph),
            IRComponent::NodeKind(nk) => {
                let iri = match nk.node_kind() {
                    NodeKind::Iri => ShaclVocab::sh_iri(),
                    _ => unimplemented!(),
                };
                register_iri(iri, ShaclVocab::sh_datatype().clone(), id, graph)
            }
            IRComponent::MinCount(mc) => register_integer(mc.min_count() as isize, ShaclVocab::sh_min_count().clone(), id, graph),
            IRComponent::MaxCount(mc) => register_integer(mc.max_count() as isize, ShaclVocab::sh_max_count().clone(), id, graph),
            IRComponent::MinExclusive(me) => register_literal(me.min_exclusive(), ShaclVocab::sh_min_exclusive().clone(), id, graph),
            IRComponent::MaxExclusive(me) => register_literal(me.max_exclusive(), ShaclVocab::sh_max_exclusive().clone(), id, graph),
            IRComponent::MinInclusive(mi) => register_literal(mi.min_inclusive(), ShaclVocab::sh_min_inclusive().clone(), id, graph),
            IRComponent::MaxInclusive(mi) => register_literal(mi.max_inclusive(), ShaclVocab::sh_max_inclusive().clone(), id, graph),
            IRComponent::MinLength(ml) => register_integer(ml.min_length(), ShaclVocab::sh_min_length().clone(), id, graph),
            IRComponent::MaxLength(ml) => register_integer(ml.max_length(), ShaclVocab::sh_max_length().clone(), id, graph),
            IRComponent::Pattern(p) => {
                if let Some(flags) = p.flags() {
                    register_literal(&ConcreteLiteral::str(flags), ShaclVocab::sh_flags().clone(), id, graph)?;
                }
                register_literal(&ConcreteLiteral::str(p.pattern()), ShaclVocab::sh_pattern().clone(), id, graph)
            },
            IRComponent::UniqueLang(ul) => register_boolean(ul.unique_lang(), ShaclVocab::sh_unique_lang().clone(), id, graph),
            IRComponent::LanguageIn(li) => {
                li.langs().iter().try_for_each(|l| {
                    register_literal(&ConcreteLiteral::str(&l.to_string()), ShaclVocab::sh_language_in().clone(), id, graph)
                })
            }
            IRComponent::Equals(eq) => register_iri(eq.iri(), ShaclVocab::sh_equals().clone(), id, graph),
            IRComponent::Disjoint(d) => register_iri(d.iri(), ShaclVocab::sh_disjoint().clone(), id, graph),
            IRComponent::LessThan(lt) => register_iri(lt.iri(), ShaclVocab::sh_less_than().clone(), id, graph),
            IRComponent::LessThanOrEquals(lte) => register_iri(lte.iri(), ShaclVocab::sh_less_than_or_equals().clone(), id, graph),
            IRComponent::Or(or) => {
                or.shapes().iter().try_for_each(|idx| {
                    // TODO - Throw error instead of unwrap
                    let shape = shape_map.get(idx).unwrap();
                    register_term(&shape.id().clone().into(), ShaclVocab::sh_or().clone(), id, graph)
                })
            }
            IRComponent::And(and) => {
                and.shapes().iter().try_for_each(|idx| {
                    // TODO - Throw error instead of unwrap
                    let shape = shape_map.get(idx).unwrap();
                    register_term(&shape.id().clone().into(), ShaclVocab::sh_and().clone(), id, graph)
                })
            }
            IRComponent::Not(not) => register_term(
                // TODO - Throw error instead of unwrap
                &shape_map
                    .get(not.shape())
                    .unwrap()
                    .id()
                    .clone()
                    .into(),
                ShaclVocab::sh_not().clone(),
                id,
                graph),
            IRComponent::Xone(xone) => {
                xone.shapes().iter().try_for_each(|idx| {
                    // TODO - Throw error instead of unwrap
                    let shape = shape_map.get(idx).unwrap();
                    register_term(&shape.id().clone().into(), ShaclVocab::sh_xone().clone(), id, graph)
                })
            }
            IRComponent::Node(n) => register_term(
                // TODO - Throw error instead of unwrap
                &shape_map
                    .get(n.shape())
                    .unwrap()
                    .id()
                    .clone()
                    .into(),
                ShaclVocab::sh_node().clone(),
                id,
                graph
            ),
            IRComponent::HasValue(hv) => match hv.value() {
                Object::Iri(iri) => register_iri(iri, ShaclVocab::sh_has_value().clone(), id, graph),
                Object::Literal(lit) => register_literal(lit, ShaclVocab::sh_has_value().clone(), id, graph),
                _ => unreachable!(),
            }
            IRComponent::In(i) => {
                // TODO - Review this code
                i.values().iter().try_for_each(|v| match v {
                    Object::Iri(iri) => register_iri(iri, ShaclVocab::sh_in().clone(), id, graph),
                    Object::Literal(lit) => register_literal(lit, ShaclVocab::sh_in().clone(), id, graph),
                    _ => unreachable!(),
                })
            }
            IRComponent::QualifiedValueShape(qvs) => {
                if let Some(value) = qvs.qualified_min_count() {
                    register_integer(value, ShaclVocab::sh_qualified_min_count().clone(), id, graph)?;
                }

                if let Some(value) = qvs.qualified_max_count() {
                    register_integer(value, ShaclVocab::sh_qualified_max_count().clone(), id, graph)?;
                }

                if let Some(value) = qvs.qualified_value_shapes_disjoint() {
                    register_boolean(value, ShaclVocab::sh_qualified_value_shapes_disjoint().clone(), id, graph)?;
                }

                // TODO - Throw error instead of unwrap
                let shape = shape_map.get(qvs.shape()).unwrap();
                register_term(&shape.id().clone().into(), ShaclVocab::sh_qualified_value_shape().clone(), id, graph)
            }
        }
    }
}

// For reference
// impl Component {
//     pub fn write<B: BuildRDF>(&self, rdf_node: &Object, rdf: &mut B) -> Result<(), B::Err> {
//         match self {
//             Self::Closed {
//                 is_closed,
//                 ignored_properties,
//             } => {
//                 Self::write_boolean(*is_closed, ShaclVocab::SH_CLOSED, rdf_node, rdf)?;
//
//                 ignored_properties.iter().try_for_each(|iri| {
//                     let iri_ref = IriRef::Iri(iri.clone());
//                     Self::write_iri(&iri_ref, ShaclVocab::SH_IGNORED_PROPERTIES, rdf_node, rdf)
//                 })?;
//             },
//             Self::Deactivated(value) => {
//                 Self::write_boolean(*value, ShaclVocab::SH_DEACTIVATED, rdf_node, rdf)?;
//                 // TODO - For Node Expr, do not delete
//                 // if let NodeExpr::Literal(ConcreteLiteral::BooleanLiteral(lit)) = value {
//                 //     Self::write_boolean(*lit, ShaclVocab::SH_DEACTIVATED, rdf_node, rdf)
//                 // } else {
//                 //     todo!() // TODO - Launch error, since sh:deactivated only accepts boolean literals
//                 // }?
//             },
//         }
//         Ok(())
//     }
// }

impl IRComponent {
    pub fn add_edges(&self, idx: ShapeLabelIdx, dg: &mut DependencyGraph, posneg: PosNeg, ir: &IRSchema, cache: &mut HashSet<ShapeLabelIdx>) {
        match self {
            IRComponent::Class(_) => {}
            IRComponent::Datatype(_) => {}
            IRComponent::NodeKind(_) => {}
            IRComponent::MinCount(_) => {}
            IRComponent::MaxCount(_) => {}
            IRComponent::MinExclusive(_) => {}
            IRComponent::MaxExclusive(_) => {}
            IRComponent::MinInclusive(_) => {}
            IRComponent::MaxInclusive(_) => {}
            IRComponent::MinLength(_) => {}
            IRComponent::MaxLength(_) => {}
            IRComponent::Pattern(_) => {}
            IRComponent::UniqueLang(_) => {}
            IRComponent::LanguageIn(_) => {}
            IRComponent::Equals(_) => {}
            IRComponent::Disjoint(_) => {}
            IRComponent::LessThan(_) => {}
            IRComponent::LessThanOrEquals(_) => {}
            IRComponent::Or(or) => {
                for shape_idx in or.shapes() {
                    if let Some(shape) = ir.get_shape_from_idx(shape_idx) {
                        dg.add_edge(idx, *shape_idx, posneg);
                        if cache.contains(shape_idx) { continue; }
                        cache.insert(*shape_idx);
                        shape.add_edges(*shape_idx, dg, posneg, ir, cache);
                    }
                }
            }
            IRComponent::And(and) => {
                for shape_idx in and.shapes() {
                    if let Some(shape) = ir.get_shape_from_idx(shape_idx) {
                        dg.add_edge(idx, *shape_idx, posneg);
                        if cache.contains(shape_idx) { continue; }
                        cache.insert(*shape_idx);
                        shape.add_edges(*shape_idx, dg, posneg, ir, cache);
                    }
                }
            }
            IRComponent::Not(not) => {
                let shape_idx = not.shape();
                if let Some(shape) = ir.get_shape_from_idx(shape_idx) {
                    dg.add_edge(idx, *shape_idx, posneg.change());
                    if !cache.contains(shape_idx) {
                        cache.insert(*shape_idx);
                        shape.add_edges(*shape_idx, dg, posneg.change(), ir, cache);
                    }
                }
            }
            IRComponent::Xone(xone) => {
                for shape_idx in xone.shapes() {
                    if let Some(shape) = ir.get_shape_from_idx(shape_idx) {
                        dg.add_edge(idx, *shape_idx, posneg);
                        if cache.contains(shape_idx) { continue; }
                        cache.insert(*shape_idx);
                        shape.add_edges(*shape_idx, dg, posneg, ir, cache);
                    }
                }
            }
            IRComponent::Node(node) => {
                let shape_idx = node.shape();
                if let Some(shape) = ir.get_shape_from_idx(shape_idx) {
                    dg.add_edge(idx, *shape_idx, posneg);
                    if !cache.contains(shape_idx) {
                        cache.insert(*shape_idx);
                        shape.add_edges(*shape_idx, dg, posneg, ir, cache);
                    }
                }
            }
            IRComponent::HasValue(_) => {}
            IRComponent::In(_) => {}
            IRComponent::QualifiedValueShape(qvs) => {
                dg.add_edge(idx, *qvs.shape(), posneg);
                // for sibling in qvs.siblings() {
                //     dg.add_edge(idx, *sibling, posneg);
                // }
            }
        }
    }
}

fn register_integer<RDF: BuildRDF>(value: isize, predicate: IriS, node: &Object, graph: &mut RDF) -> Result<(), RDF::Err> {
    let value: i128 = value.try_into().unwrap();
    let literal: RDF::Literal = value.into();
    register_term(&literal.into(), predicate, node, graph)
}

fn register_boolean<RDF: BuildRDF>(value: bool, predicate: IriS, node: &Object, graph: &mut RDF) -> Result<(), RDF::Err> {
    let literal: RDF::Literal = value.into();
    register_term(&literal.into(), predicate, node, graph)
}

fn register_literal<RDF: BuildRDF>(value: &ConcreteLiteral, predicate: IriS, node: &Object, graph: &mut RDF) -> Result<(), RDF::Err> {
    let literal: RDF::Literal = value.lexical_form().into();
    register_term(&literal.into(), predicate, node, graph)
}

fn register_iri<RDF: BuildRDF>(value: &IriS, predicate: IriS, node: &Object, graph: &mut RDF) -> Result<(), RDF::Err> {
    register_term(&value.clone().into(), predicate, node, graph)
}

fn register_term<RDF: BuildRDF>(value: &RDF::Term, predicate: IriS, node: &Object, graph: &mut RDF) -> Result<(), RDF::Err> {
    let node: RDF::Subject = node.clone().try_into().map_err(|_| unreachable!())?;
    graph.add_triple(node, predicate, value.clone())
}

impl From<&IRComponent> for IriS {
    fn from(value: &IRComponent) -> Self {
        match value {
            IRComponent::Class(_) => ShaclVocab::sh_class().clone(),
            IRComponent::Datatype(_) => ShaclVocab::sh_datatype().clone(),
            IRComponent::NodeKind(_) => ShaclVocab::sh_node_kind().clone(),
            IRComponent::MinCount(_) => ShaclVocab::sh_min_count().clone(),
            IRComponent::MaxCount(_) => ShaclVocab::sh_max_count().clone(),
            IRComponent::MinExclusive(_) => ShaclVocab::sh_min_exclusive().clone(),
            IRComponent::MaxExclusive(_) => ShaclVocab::sh_max_exclusive().clone(),
            IRComponent::MinInclusive(_) => ShaclVocab::sh_min_inclusive().clone(),
            IRComponent::MaxInclusive(_) => ShaclVocab::sh_max_inclusive().clone(),
            IRComponent::MinLength(_) => ShaclVocab::sh_min_length().clone(),
            IRComponent::MaxLength(_) => ShaclVocab::sh_max_length().clone(),
            IRComponent::Pattern(_) => ShaclVocab::sh_pattern().clone(),
            IRComponent::UniqueLang(_) => ShaclVocab::sh_unique_lang().clone(),
            IRComponent::LanguageIn(_) => ShaclVocab::sh_language_in().clone(),
            IRComponent::Equals(_) => ShaclVocab::sh_equals().clone(),
            IRComponent::Disjoint(_) => ShaclVocab::sh_disjoint().clone(),
            IRComponent::LessThan(_) => ShaclVocab::sh_less_than().clone(),
            IRComponent::LessThanOrEquals(_) => ShaclVocab::sh_less_than_or_equals().clone(),
            IRComponent::Or(_) => ShaclVocab::sh_or().clone(),
            IRComponent::And(_) => ShaclVocab::sh_and().clone(),
            IRComponent::Not(_) => ShaclVocab::sh_not().clone(),
            IRComponent::Xone(_) => ShaclVocab::sh_xone().clone(),
            IRComponent::Node(_) => ShaclVocab::sh_node().clone(),
            IRComponent::HasValue(_) => ShaclVocab::sh_has_value().clone(),
            IRComponent::In(_) => ShaclVocab::sh_in().clone(),
            IRComponent::QualifiedValueShape(_) => ShaclVocab::sh_qualified_value_shape().clone(),
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
        }
    }
}
