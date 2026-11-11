use crate::ast::{ASTSchema, ASTTarget};
use crate::ir::error::IRError;
use crate::ir::node_expr::IRNodeExpr;
use crate::ir::dg::{DependencyGraph, PosNeg};
use crate::ir::{IRSchema, IRShape, IRTarget, ShapeLabelIdx, add_shape_edge};
use crate::types::Target;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab, ShaclVocab};
use std::collections::{HashMap, HashSet};

impl IRTarget {
    pub fn compile(target: &ASTTarget, ast: &ASTSchema, ir: &mut IRSchema) -> Result<Self, IRError> {
        let out = match target {
            Target::Node(expr) => Target::Node(IRNodeExpr::compile(expr, ast, ir)?),
            Target::Class(c) => Target::Class(c.clone()),
            Target::SubjectsOf(p) => Target::SubjectsOf(p.clone()),
            Target::ObjectsOf(p) => Target::ObjectsOf(p.clone()),
            Target::ImplicitClass(c) => Target::ImplicitClass(c.clone()),
            Target::Where(shape) => {
                // The shape is compiled here, and looked up by its id when the target is computed
                ir.register_shape(shape, None, ast)?;
                Target::Where(shape.clone())
            },
            Target::WrongNode(o) => Target::WrongNode(o.clone()),
            Target::WrongClass(o) => Target::WrongClass(o.clone()),
            Target::WrongSubjectsOf(o) => Target::WrongSubjectsOf(o.clone()),
            Target::WrongObjectsOf(o) => Target::WrongObjectsOf(o.clone()),
            Target::WrongImplicitClass(o) => Target::WrongImplicitClass(o.clone()),
            Target::WrongWhere(o) => Target::WrongWhere(o.clone()),
        };

        Ok(out)
    }

    /// Records the shapes the target declaration refers to as dependencies of the shape at `idx`.
    pub(crate) fn add_edges(
        &self,
        idx: ShapeLabelIdx,
        dg: &mut DependencyGraph,
        posneg: PosNeg,
        ir: &IRSchema,
        cache: &mut HashSet<ShapeLabelIdx>,
    ) {
        match self {
            Target::Node(expr) => expr.add_edges(idx, dg, posneg, ir, cache),
            Target::Where(shape) => {
                if let Some(to) = ir.get_idx(shape) {
                    add_shape_edge(idx, *to, dg, posneg.change(), ir, cache);
                }
            },
            _ => {},
        }
    }

    pub fn register<RDF: BuildRDF>(
        &self,
        id: &Object,
        graph: &mut RDF,
        shapes: &HashMap<ShapeLabelIdx, IRShape>,
    ) -> Result<(), IRError> {
        let node: RDF::Subject = id.clone().try_into().unwrap_or_else(|_| unreachable!());

        let (predicate, value) = match self {
            Target::Node(n) => (ShaclVocab::sh_target_node(), n.register(graph, shapes)?),
            // An ill-formed target is written back as it was read
            Target::WrongNode(o) => (ShaclVocab::sh_target_node(), RDF::object_as_term(o)),
            Target::Class(c) | Target::WrongClass(c) => (ShaclVocab::sh_target_class(), RDF::object_as_term(c)),
            Target::SubjectsOf(s) => (ShaclVocab::sh_target_subjects_of(), s.clone().into()),
            Target::ObjectsOf(o) => (ShaclVocab::sh_target_objects_of(), o.clone().into()),
            // TODO - Review this code and in SHACL 1.2, add sh_shape_class ?
            Target::ImplicitClass(_) | Target::WrongImplicitClass(_) => {
                (RdfVocab::rdf_type().clone(), RdfsVocab::rdfs_class().into())
            },
            Target::WrongSubjectsOf(o) => (ShaclVocab::sh_target_subjects_of(), RDF::object_as_term(o)),
            Target::WrongObjectsOf(o) => (ShaclVocab::sh_target_objects_of(), RDF::object_as_term(o)),
            Target::Where(o) | Target::WrongWhere(o) => (ShaclVocab::sh_target_where(), RDF::object_as_term(o)),
        };

        graph
            .add_triple(node, predicate, value)
            .map_err(IRError::add_triple::<RDF>)
    }
}
