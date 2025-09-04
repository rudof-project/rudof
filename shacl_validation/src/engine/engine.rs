use iri_s::IriS;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::property_shape::PropertyShapeIR;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::compiled::target::CompiledTarget;
use srdf::NeighsRDF;
use srdf::RDFNode;
use srdf::SHACLPath;

use crate::focus_nodes::FocusNodes;
use crate::helpers::srdf::get_objects_for_shacl_path;
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
    ) -> Result<Vec<ValidationResult>, ValidateError>;

    fn focus_nodes(
        &self,
        store: &S,
        targets: &[CompiledTarget],
    ) -> Result<FocusNodes<S>, ValidateError> {
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
        Ok(FocusNodes::from_iter(ts.into_iter()))
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &RDFNode) -> Result<FocusNodes<S>, ValidateError>;

    fn target_class(&self, store: &S, class: &RDFNode) -> Result<FocusNodes<S>, ValidateError>;

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &IriS,
    ) -> Result<FocusNodes<S>, ValidateError>;

    fn target_object_of(&self, store: &S, predicate: &IriS)
    -> Result<FocusNodes<S>, ValidateError>;

    fn implicit_target_class(
        &self,
        store: &S,
        shape: &RDFNode,
    ) -> Result<FocusNodes<S>, ValidateError>;

    fn path(
        &self,
        store: &S,
        shape: &PropertyShapeIR,
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        let nodes = get_objects_for_shacl_path(store, focus_node, shape.path())?;
        Ok(FocusNodes::new(nodes))
    }
}
