use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;
use crate::compiled::{compile_shape, compile_shapes, convert_value};
use crate::components::Disjoint;
use crate::components::Equals;
use crate::components::HasValue;
use crate::components::In;
use crate::components::LanguageIn;
use crate::components::LessThan;
use crate::components::LessThanOrEquals;
use crate::components::MaxCount;
use crate::components::MaxExclusive;
use crate::components::MaxInclusive;
use crate::components::MaxLength;
use crate::components::MinCount;
use crate::components::MinExclusive;
use crate::components::MinInclusive;
use crate::components::MinLength;
use crate::components::Node;
use crate::components::Nodekind;
use crate::components::Not;
use crate::components::Or;
use crate::components::Pattern;
use crate::components::QualifiedValueShape;
use crate::components::UniqueLang;
use crate::components::Xone;
use crate::components::{And, Class, Datatype};
use crate::dependency_graph::{DependencyGraph, PosNeg};
use crate::schema_ir::SchemaIR;
use crate::shape_label_idx::ShapeLabelIdx;
use iri_s::IriS;
use rudof_rdf::rdf_core::Rdf;
use shacl_ast::component::Component;
use shacl_ast::{ShaclSchema, ShaclVocab};
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ComponentIR {
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

impl ComponentIR {
    /// Compiles an AST SHACL component to an IR SHACL Component
    /// It returns None for components that are not represented in the IR,
    /// such as sh:closed and sh:deactivated.
    /// It returns a vector of (PosNeg, ShapeLabelIdx) pairs for components that are represented in the IR.
    /// The vector is list of dependant shapes for cases with recursion
    pub fn compile<S: Rdf>(
        component: Component,
        schema: &ShaclSchema<S>,
        schema_ir: &mut SchemaIR,
    ) -> Result<Option<ComponentIR>, Box<CompiledShaclError>> {
        let value = match component {
            Component::Class(object) => {
                let class_rule = object;
                Some(ComponentIR::Class(Class::new(class_rule)))
            },
            Component::Datatype(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(ComponentIR::Datatype(Datatype::new(iri_ref)))
            },
            Component::NodeKind(node_kind) => Some(ComponentIR::NodeKind(Nodekind::new(node_kind))),
            Component::MinCount(count) => Some(ComponentIR::MinCount(MinCount::new(count))),
            Component::MaxCount(count) => Some(ComponentIR::MaxCount(MaxCount::new(count))),
            Component::MinExclusive(literal) => Some(ComponentIR::MinExclusive(MinExclusive::new(literal))),
            Component::MaxExclusive(literal) => Some(ComponentIR::MaxExclusive(MaxExclusive::new(literal))),
            Component::MinInclusive(literal) => Some(ComponentIR::MinInclusive(MinInclusive::new(literal))),
            Component::MaxInclusive(literal) => Some(ComponentIR::MaxInclusive(MaxInclusive::new(literal))),
            Component::MinLength(length) => Some(ComponentIR::MinLength(MinLength::new(length))),
            Component::MaxLength(length) => Some(ComponentIR::MaxLength(MaxLength::new(length))),
            Component::Pattern { pattern, flags } => {
                let pattern = Pattern::new(pattern, flags)?;
                Some(ComponentIR::Pattern(pattern))
            },
            Component::UniqueLang(lang) => Some(ComponentIR::UniqueLang(UniqueLang::new(lang))),
            Component::LanguageIn(langs) => Some(ComponentIR::LanguageIn(LanguageIn::new(langs))),
            Component::Equals(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(ComponentIR::Equals(Equals::new(iri_ref)))
            },
            Component::Disjoint(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(ComponentIR::Disjoint(Disjoint::new(iri_ref)))
            },
            Component::LessThan(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(ComponentIR::LessThan(LessThan::new(iri_ref)))
            },
            Component::LessThanOrEquals(iri_ref) => {
                let iri_ref = convert_iri_ref(iri_ref)?;
                Some(ComponentIR::LessThanOrEquals(LessThanOrEquals::new(iri_ref)))
            },
            Component::Or(shapes) => {
                let values = compile_shapes::<S>(shapes, schema, schema_ir)?;
                let ors = values.into_iter().collect::<Vec<_>>();
                Some(ComponentIR::Or(Or::new(ors)))
            },
            Component::And(shapes) => {
                let values = compile_shapes::<S>(shapes, schema, schema_ir)?;
                let ands = values.into_iter().collect::<Vec<_>>();
                Some(ComponentIR::And(And::new(ands)))
            },
            Component::Not(shape) => {
                let shape = compile_shape::<S>(&shape, schema, schema_ir)?;
                Some(ComponentIR::Not(Not::new(shape)))
            },
            Component::Xone(shapes) => {
                let values = compile_shapes::<S>(shapes, schema, schema_ir)?;
                let xones = values.into_iter().collect::<Vec<_>>();
                Some(ComponentIR::Xone(Xone::new(xones)))
            },
            Component::Closed { .. } => None,
            Component::Node(shape) => {
                let shape = compile_shape::<S>(&shape, schema, schema_ir)?;
                Some(ComponentIR::Node(Node::new(shape)))
            },
            Component::HasValue(value) => {
                let term = convert_value(value)?;
                Some(ComponentIR::HasValue(HasValue::new(term)))
            },
            Component::In(values) => {
                let terms = values.into_iter().map(convert_value).collect::<Result<Vec<_>, _>>()?;
                Some(ComponentIR::In(In::new(terms)))
            },
            Component::QualifiedValueShape {
                shape,
                q_min_count,
                q_max_count,
                disjoint,
                siblings,
            } => {
                let shape = compile_shape::<S>(&shape, schema, schema_ir)?;
                let mut compiled_siblings = Vec::new();
                for sibling in siblings.iter() {
                    let compiled_sibling = compile_shape::<S>(sibling, schema, schema_ir)?;
                    compiled_siblings.push(compiled_sibling);
                }
                Some(ComponentIR::QualifiedValueShape(QualifiedValueShape::new(
                    shape,
                    q_min_count,
                    q_max_count,
                    disjoint,
                    compiled_siblings,
                )))
            },
            Component::Deactivated(_b) => None,
        };
        Ok(value)
    }

    pub(crate) fn add_edges(
        &self,
        shape_idx: ShapeLabelIdx,
        dg: &mut DependencyGraph,
        posneg: PosNeg,
        schema_ir: &SchemaIR,
        visited: &mut HashSet<ShapeLabelIdx>,
    ) {
        match self {
            ComponentIR::Class(_c) => {},
            ComponentIR::Datatype(_d) => {},
            ComponentIR::NodeKind(_nk) => {},
            ComponentIR::MinCount(_mc) => {},
            ComponentIR::MaxCount(_mc) => {},
            ComponentIR::MinExclusive(_me) => {},
            ComponentIR::MaxExclusive(_me) => {},
            ComponentIR::MinInclusive(_mi) => {},
            ComponentIR::MaxInclusive(_mi) => {},
            ComponentIR::MinLength(_ml) => {},
            ComponentIR::MaxLength(_ml) => {},
            ComponentIR::Pattern(_p) => {},
            ComponentIR::UniqueLang(_ul) => {},
            ComponentIR::LanguageIn(_li) => {},
            ComponentIR::Equals(_e) => {},
            ComponentIR::Disjoint(_d) => {},
            ComponentIR::LessThan(_lt) => {},
            ComponentIR::LessThanOrEquals(_lte) => {},
            ComponentIR::Or(o) => {
                for idx in o.shapes() {
                    if let Some(shape) = schema_ir.get_shape_from_idx(idx) {
                        dg.add_edge(shape_idx, *idx, posneg);
                        if visited.contains(idx) {
                            continue;
                        } else {
                            visited.insert(*idx);
                            shape.add_edges(*idx, dg, posneg, schema_ir, visited);
                        }
                    }
                }
            },
            ComponentIR::And(a) => {
                for idx in a.shapes() {
                    if let Some(shape) = schema_ir.get_shape_from_idx(idx) {
                        dg.add_edge(shape_idx, *idx, posneg);
                        if visited.contains(idx) {
                            continue;
                        } else {
                            visited.insert(*idx);
                            shape.add_edges(*idx, dg, posneg, schema_ir, visited);
                        }
                    }
                }
            },
            ComponentIR::Not(n) => {
                let idx = n.shape();
                if let Some(shape) = schema_ir.get_shape_from_idx(idx) {
                    dg.add_edge(shape_idx, *idx, posneg.change());
                    if visited.contains(idx) {
                    } else {
                        visited.insert(*idx);
                        shape.add_edges(*idx, dg, posneg.change(), schema_ir, visited);
                    }
                }
            },
            ComponentIR::Xone(x) => {
                for idx in x.shapes() {
                    if let Some(shape) = schema_ir.get_shape_from_idx(idx) {
                        dg.add_edge(shape_idx, *idx, posneg);
                        if visited.contains(idx) {
                            continue;
                        } else {
                            visited.insert(*idx);
                            shape.add_edges(*idx, dg, posneg, schema_ir, visited);
                        }
                    }
                }
            },
            ComponentIR::Node(n) => {
                let idx = n.shape();
                if let Some(shape) = schema_ir.get_shape_from_idx(idx) {
                    dg.add_edge(shape_idx, *idx, posneg);
                    if visited.contains(idx) {
                    } else {
                        visited.insert(*idx);
                        shape.add_edges(*idx, dg, posneg, schema_ir, visited);
                    }
                }
            },
            ComponentIR::HasValue(_hv) => {},
            ComponentIR::In(_i) => {},
            ComponentIR::QualifiedValueShape(qvs) => {
                dg.add_edge(shape_idx, *qvs.shape(), posneg);
                /*for sibling in qvs.siblings() {
                    dg.add_edge(shape_idx, *sibling, posneg);
                }*/
            },
        }
    }
}

impl From<&ComponentIR> for IriS {
    fn from(value: &ComponentIR) -> Self {
        match value {
            ComponentIR::Class(_) => ShaclVocab::sh_class().clone(),
            ComponentIR::Datatype(_) => ShaclVocab::sh_datatype().clone(),
            ComponentIR::NodeKind(_) => ShaclVocab::sh_node_kind().clone(),
            ComponentIR::MinCount(_) => ShaclVocab::sh_min_count().clone(),
            ComponentIR::MaxCount(_) => ShaclVocab::sh_max_count().clone(),
            ComponentIR::MinExclusive(_) => ShaclVocab::sh_min_exclusive().clone(),
            ComponentIR::MaxExclusive(_) => ShaclVocab::sh_max_exclusive().clone(),
            ComponentIR::MinInclusive(_) => ShaclVocab::sh_min_inclusive().clone(),
            ComponentIR::MaxInclusive(_) => ShaclVocab::sh_max_inclusive().clone(),
            ComponentIR::MinLength(_) => ShaclVocab::sh_min_length().clone(),
            ComponentIR::MaxLength(_) => ShaclVocab::sh_max_length().clone(),
            ComponentIR::Pattern { .. } => ShaclVocab::sh_pattern().clone(),
            ComponentIR::UniqueLang(_) => ShaclVocab::sh_unique_lang().clone(),
            ComponentIR::LanguageIn { .. } => ShaclVocab::sh_language_in().clone(),
            ComponentIR::Equals(_) => ShaclVocab::sh_equals().clone(),
            ComponentIR::Disjoint(_) => ShaclVocab::sh_disjoint().clone(),
            ComponentIR::LessThan(_) => ShaclVocab::sh_less_than().clone(),
            ComponentIR::LessThanOrEquals(_) => ShaclVocab::sh_less_than_or_equals().clone(),
            ComponentIR::Or { .. } => ShaclVocab::sh_or().clone(),
            ComponentIR::And { .. } => ShaclVocab::sh_and().clone(),
            ComponentIR::Not { .. } => ShaclVocab::sh_not().clone(),
            ComponentIR::Xone { .. } => ShaclVocab::sh_xone().clone(),
            ComponentIR::Node { .. } => ShaclVocab::sh_node().clone(),
            ComponentIR::HasValue { .. } => ShaclVocab::sh_has_value().clone(),
            ComponentIR::In { .. } => ShaclVocab::sh_in().clone(),
            ComponentIR::QualifiedValueShape { .. } => ShaclVocab::sh_qualified_value_shape().clone(),
        }
    }
}

impl Display for ComponentIR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComponentIR::Class(cls) => write!(f, " {cls}"),
            ComponentIR::Datatype(dt) => write!(f, " {dt}"),
            ComponentIR::NodeKind(nk) => write!(f, " {nk}"),
            ComponentIR::MinCount(n) => write!(f, " {n}"),
            ComponentIR::MaxCount(n) => write!(f, " {n}"),
            ComponentIR::MinExclusive(n) => write!(f, " {n}"),
            ComponentIR::MaxExclusive(n) => write!(f, " {n}"),
            ComponentIR::MinInclusive(n) => write!(f, " {n}"),
            ComponentIR::MaxInclusive(n) => write!(f, " {n}"),
            ComponentIR::MinLength(n) => write!(f, " {n}"),
            ComponentIR::MaxLength(n) => write!(f, " {n}"),
            ComponentIR::Pattern(pat) => write!(f, " {pat}"),
            ComponentIR::UniqueLang(ul) => write!(f, " {ul}"),
            ComponentIR::LanguageIn(l) => write!(f, " {l}"),
            ComponentIR::Equals(p) => write!(f, " {p}"),
            ComponentIR::Disjoint(p) => write!(f, " {p}"),
            ComponentIR::LessThan(p) => write!(f, " {p}"),
            ComponentIR::LessThanOrEquals(p) => write!(f, " {p}"),
            ComponentIR::Or(or) => write!(f, " {or}"),
            ComponentIR::And(and) => write!(f, " {and}"),
            ComponentIR::Not(not) => write!(f, " {not}"),
            ComponentIR::Xone(xone) => write!(f, " {xone}"),
            ComponentIR::Node(node) => write!(f, " {node}"),
            ComponentIR::HasValue(value) => write!(f, " HasValue({value})"),
            ComponentIR::In(vs) => write!(f, " {vs}"),
            ComponentIR::QualifiedValueShape(qvs) => {
                write!(f, " {qvs}")
            },
        }
    }
}
