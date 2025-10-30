use iri_s::IriS;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::property_shape::PropertyShapeIR;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::compiled::target::CompiledTarget;
use shacl_ir::schema_ir::SchemaIR;
use srdf::NeighsRDF;
use srdf::RDFNode;
use srdf::SHACLPath;

use crate::focus_nodes::FocusNodes;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

pub trait Engine<S: NeighsRDF> {
    fn evaluate(
        &self,
        store: &S,
        shape: &ShapeIR,
        component: &ComponentIR,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, Box<ValidateError>>;

    fn focus_nodes(
        &self,
        store: &S,
        targets: &[CompiledTarget],
    ) -> Result<FocusNodes<S>, Box<ValidateError>> {
        let targets_iter: Vec<FocusNodes<S>> = targets
            .iter()
            .flat_map(|target| match target {
                CompiledTarget::Node(node) => self.target_node(store, node),
                CompiledTarget::Class(class) => self.target_class(store, class),
                CompiledTarget::SubjectsOf(predicate) => self.target_subject_of(store, predicate),
                CompiledTarget::ObjectsOf(predicate) => self.target_object_of(store, predicate),
                CompiledTarget::ImplicitClass(node) => self.implicit_target_class(store, node),
                CompiledTarget::WrongTargetNode(_) => todo!(),
                CompiledTarget::WrongTargetClass(_) => todo!(),
                CompiledTarget::WrongSubjectsOf(_) => todo!(),
                CompiledTarget::WrongObjectsOf(_) => todo!(),
                CompiledTarget::WrongImplicitClass(_) => todo!(),
            })
            .collect();
        let ts = targets_iter.into_iter().flatten();
        Ok(FocusNodes::from_iter(ts))
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &RDFNode) -> Result<FocusNodes<S>, Box<ValidateError>>;

    fn target_class(&self, store: &S, class: &RDFNode)
    -> Result<FocusNodes<S>, Box<ValidateError>>;

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &IriS,
    ) -> Result<FocusNodes<S>, Box<ValidateError>>;

    fn target_object_of(
        &self,
        store: &S,
        predicate: &IriS,
    ) -> Result<FocusNodes<S>, Box<ValidateError>>;

    fn implicit_target_class(
        &self,
        store: &S,
        shape: &RDFNode,
    ) -> Result<FocusNodes<S>, Box<ValidateError>>;

    fn path(
        &self,
        store: &S,
        shape: &PropertyShapeIR,
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, Box<ValidateError>> {
        let nodes = store
            .objects_for_shacl_path(focus_node, shape.path())
            .map_err(|e| ValidateError::ObjectsSHACLPath {
                focus_node: focus_node.to_string(),
                shacl_path: shape.path().to_string(),
                error: e.to_string(),
            })?;
        Ok(FocusNodes::new(nodes))
    }
}
