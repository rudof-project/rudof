use crate::error::ValidationError;
use crate::ir::{IRNodeExpr, IRSchema, IRShape, IRTarget};
use crate::types::Target;
use crate::validator::engine::Engine;
use crate::validator::nodes::FocusNodes;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::RdfVocab;
use rudof_rdf::rdf_core::{NeighsRDF, Rdf};
use std::collections::HashMap;
use std::fmt::Debug;

pub(crate) trait FocusNodesOps<RDF: Rdf> {
    fn focus_nodes(
        &self,
        store: &RDF,
        runner: &mut dyn Engine<RDF>,
        shapes_graph: &IRSchema,
    ) -> Result<FocusNodes<RDF>, ValidationError>;
}

impl<RDF: NeighsRDF + Debug> FocusNodesOps<RDF> for IRShape {
    fn focus_nodes(
        &self,
        store: &RDF,
        runner: &mut dyn Engine<RDF>,
        shapes_graph: &IRSchema,
    ) -> Result<FocusNodes<RDF>, ValidationError> {
        let mut nodes = Vec::new();
        for target in self.targets() {
            nodes.extend(target_nodes(target, store, runner, shapes_graph)?.iter().cloned());
        }

        Ok(FocusNodes::from_iter(nodes))
    }
}

/// The nodes `target` declares as a target from `store`.
fn target_nodes<RDF: NeighsRDF + Debug>(
    target: &IRTarget,
    store: &RDF,
    runner: &mut dyn Engine<RDF>,
    shapes_graph: &IRSchema,
) -> Result<FocusNodes<RDF>, ValidationError> {
    match target {
        Target::Node(e) => target_node_expr(e, store, runner, shapes_graph),
        Target::Class(c) => runner.target_class(store, c),
        Target::SubjectsOf(p) => runner.target_subject_of(store, p),
        Target::ObjectsOf(p) => runner.target_object_of(store, p),
        Target::ImplicitClass(n) => runner.implicit_target_class(store, n),
        Target::WrongNode(node) => runner.target_node(store, node),
        Target::WrongClass(_) | Target::WrongImplicitClass(_) => Err(ValidationError::TargetClassNotIri),
        Target::WrongSubjectsOf(o) | Target::WrongObjectsOf(o) => {
            Err(ValidationError::CastError(o.to_string(), "an IRI".to_string()))
        },
    }
}

/// Evaluates a `sh:targetNode` node expression, whose output nodes are the target.
///
/// A target is declared before there is any focus node, so an expression that reads one
/// has nothing to read, and is reported instead of being given an arbitrary node.
fn target_node_expr<RDF: NeighsRDF + Debug>(
    expr: &IRNodeExpr,
    store: &RDF,
    runner: &mut dyn Engine<RDF>,
    shapes_graph: &IRSchema,
) -> Result<FocusNodes<RDF>, ValidationError> {
    // SHACL Core only has the constant expressions, which every engine resolves on its own
    if let Some(node) = expr.as_constant() {
        return runner.target_node(store, &node);
    }
    if expr.reads_focus() {
        return Err(ValidationError::TargetNodeExprReadsFocus(expr.to_string()));
    }

    // Never read, as the expression does not read the focus node
    let focus = RDF::object_as_term(&Object::iri(RdfVocab::rdf_nil()));
    let scope = HashMap::new();

    // The expression is evaluated once, outside of any other one
    runner.node_expr_state().steps = 0;
    let nodes = expr.evaluate(store, &focus, &scope, runner, shapes_graph)?;

    Ok(FocusNodes::from_iter(nodes))
}
