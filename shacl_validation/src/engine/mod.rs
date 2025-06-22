use iri_s::IriS;
use shacl_ir::compiled::component::CompiledComponent;
use shacl_ir::compiled::property_shape::CompiledPropertyShape;
use shacl_ir::compiled::shape::CompiledShape;
use shacl_ir::compiled::target::CompiledTarget;
use srdf::RDFNode;
use srdf::Rdf;
use srdf::SHACLPath;

use crate::focus_nodes::FocusNodes;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;

pub mod native;
pub mod sparql;

pub trait Engine<S: Rdf> {
    fn evaluate(
        &self,
        store: &S,
        shape: &CompiledShape<S>,
        component: &CompiledComponent<S>,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&CompiledShape<S>>,
        maybe_path: Option<SHACLPath>,
    ) -> Result<Vec<ValidationResult>, ValidateError>;

    fn focus_nodes(
        &self,
        store: &S,
        targets: &[CompiledTarget],
    ) -> Result<FocusNodes<S>, ValidateError> {
        // TODO: here it would be nice to return an error...
        let targets = targets
            .iter()
            .flat_map(|target| match target {
                CompiledTarget::Node(node) => self.target_node(store, node),
                CompiledTarget::Class(class) => self.target_class(store, class),
                CompiledTarget::SubjectsOf(predicate) => self.target_subject_of(store, predicate),
                CompiledTarget::ObjectsOf(predicate) => self.target_object_of(store, predicate),
                CompiledTarget::ImplicitClass(node) => self.implicit_target_class(store, node),
            })
            .flatten();

        Ok(FocusNodes::new(targets))
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
        shape: &CompiledPropertyShape<S>,
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError> {
        match shape.path() {
            SHACLPath::Predicate { pred } => {
                self.predicate(store, shape, &pred.clone().into(), focus_node)
            }
            SHACLPath::Alternative { paths } => self.alternative(store, shape, paths, focus_node),
            SHACLPath::Sequence { paths } => self.sequence(store, shape, paths, focus_node),
            SHACLPath::Inverse { path } => self.inverse(store, shape, path, focus_node),
            SHACLPath::ZeroOrMore { path } => self.zero_or_more(store, shape, path, focus_node),
            SHACLPath::OneOrMore { path } => self.one_or_more(store, shape, path, focus_node),
            SHACLPath::ZeroOrOne { path } => self.zero_or_one(store, shape, path, focus_node),
        }
    }

    fn predicate(
        &self,
        store: &S,
        shape: &CompiledPropertyShape<S>,
        predicate: &S::IRI,
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError>;

    fn alternative(
        &self,
        store: &S,
        shape: &CompiledPropertyShape<S>,
        paths: &[SHACLPath],
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError>;

    fn sequence(
        &self,
        store: &S,
        shape: &CompiledPropertyShape<S>,
        paths: &[SHACLPath],
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError>;

    fn inverse(
        &self,
        store: &S,
        shape: &CompiledPropertyShape<S>,
        path: &SHACLPath,
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError>;

    fn zero_or_more(
        &self,
        store: &S,
        shape: &CompiledPropertyShape<S>,
        path: &SHACLPath,
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError>;

    fn one_or_more(
        &self,
        store: &S,
        shape: &CompiledPropertyShape<S>,
        path: &SHACLPath,
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError>;

    fn zero_or_one(
        &self,
        store: &S,
        shape: &CompiledPropertyShape<S>,
        path: &SHACLPath,
        focus_node: &S::Term,
    ) -> Result<FocusNodes<S>, ValidateError>;
}
