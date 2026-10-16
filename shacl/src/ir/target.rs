use crate::ast::{ASTSchema, ASTTarget};
use crate::ir::error::IRError;
use crate::ir::node_expr::IRNodeExpr;
use crate::ir::{IRSchema, IRTarget};
use crate::types::Target;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab, ShaclVocab};

impl IRTarget {
    pub fn compile(target: &ASTTarget, ast: &ASTSchema, ir: &mut IRSchema) -> Result<Self, IRError> {
        let out = match target {
            Target::Node(expr) => Target::Node(IRNodeExpr::compile(expr, ast, ir)?),
            Target::Class(c) => Target::Class(c.clone()),
            Target::SubjectsOf(p) => Target::SubjectsOf(p.clone()),
            Target::ObjectsOf(p) => Target::ObjectsOf(p.clone()),
            Target::ImplicitClass(c) => Target::ImplicitClass(c.clone()),
            Target::WrongNode(o) => Target::WrongNode(o.clone()),
            Target::WrongClass(o) => Target::WrongClass(o.clone()),
            Target::WrongSubjectsOf(o) => Target::WrongSubjectsOf(o.clone()),
            Target::WrongObjectsOf(o) => Target::WrongObjectsOf(o.clone()),
            Target::WrongImplicitClass(o) => Target::WrongImplicitClass(o.clone()),
        };

        Ok(out)
    }

    pub fn register<RDF: BuildRDF>(&self, id: &Object, graph: &mut RDF) -> Result<(), RDF::Err> {
        let node: RDF::Subject = id.clone().try_into().map_err(|_| unreachable!())?;

        match self {
            // TODO - Missing node expr serialization (only serializing iris and constants to backwards compat)
            Target::Node(n) => match n.as_constant() {
                Some(constant) => graph.add_triple(node, ShaclVocab::sh_target_node(), constant),
                None => todo!("unable to serialize a composite sh:targetNode node expression"),
            },
            Target::Class(c) => graph.add_triple(node, ShaclVocab::sh_target_class(), c.clone()),
            Target::SubjectsOf(s) => graph.add_triple(node, ShaclVocab::sh_target_subjects_of(), s.clone()),
            Target::ObjectsOf(o) => graph.add_triple(node, ShaclVocab::sh_target_objects_of(), o.clone()),
            // TODO - Review this code and in SHACL 1.2, add sh_shape_class ?
            Target::ImplicitClass(_) => graph.add_triple(node, RdfVocab::rdf_type().clone(), RdfsVocab::rdfs_class()),
            Target::WrongNode(_) => todo!(),
            Target::WrongClass(_) => todo!(),
            Target::WrongSubjectsOf(_) => todo!(),
            Target::WrongObjectsOf(_) => todo!(),
            Target::WrongImplicitClass(_) => todo!(),
        }
    }
}
