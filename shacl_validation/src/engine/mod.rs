use shacl_ast::compiled::property_shape::CompiledPropertyShape;
use shacl_ast::compiled::target::CompiledTarget;
use srdf::Rdf;
use srdf::SHACLPath;

use crate::focus_nodes::FocusNodes;
use crate::validate_error::ValidateError;

pub mod native;
pub mod sparql;

pub trait Engine<R: Rdf> {
    fn focus_nodes(
        store: &R,
        targets: &[CompiledTarget<R>],
    ) -> Result<FocusNodes<R>, ValidateError> {
        let targets = targets
            .iter()
            .flat_map(|target| match target {
                CompiledTarget::Node(node) => Self::target_node(store, node),
                CompiledTarget::Class(class) => Self::target_class(store, class),
                CompiledTarget::SubjectsOf(predicate) => Self::target_subject_of(store, predicate),
                CompiledTarget::ObjectsOf(predicate) => Self::target_object_of(store, predicate),
                CompiledTarget::ImplicitClass(class) => Self::implicit_target_class(store, class),
            })
            .flatten();

        Ok(FocusNodes::new(targets))
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(store: &R, node: &R::Term) -> Result<FocusNodes<R>, ValidateError>;

    fn target_class(store: &R, class: &R::Term) -> Result<FocusNodes<R>, ValidateError>;

    fn target_subject_of(store: &R, predicate: &R::IRI) -> Result<FocusNodes<R>, ValidateError>;

    fn target_object_of(store: &R, predicate: &R::IRI) -> Result<FocusNodes<R>, ValidateError>;

    fn implicit_target_class(store: &R, class: &R::Term) -> Result<FocusNodes<R>, ValidateError>;

    fn path(
        store: &R,
        shape: &CompiledPropertyShape<R>,
        focus_node: &R::Term,
    ) -> Result<FocusNodes<R>, ValidateError> {
        match shape.path() {
            SHACLPath::Predicate { pred } => {
                Self::predicate(store, shape, &pred.clone().into(), focus_node)
            }
            SHACLPath::Alternative { paths } => Self::alternative(store, shape, paths, focus_node),
            SHACLPath::Sequence { paths } => Self::sequence(store, shape, paths, focus_node),
            SHACLPath::Inverse { path } => Self::inverse(store, shape, path, focus_node),
            SHACLPath::ZeroOrMore { path } => Self::zero_or_more(store, shape, path, focus_node),
            SHACLPath::OneOrMore { path } => Self::one_or_more(store, shape, path, focus_node),
            SHACLPath::ZeroOrOne { path } => Self::zero_or_one(store, shape, path, focus_node),
        }
    }

    fn predicate(
        store: &R,
        shape: &CompiledPropertyShape<R>,
        predicate: &R::IRI,
        focus_node: &R::Term,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn alternative(
        store: &R,
        shape: &CompiledPropertyShape<R>,
        paths: &[SHACLPath],
        focus_node: &R::Term,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn sequence(
        store: &R,
        shape: &CompiledPropertyShape<R>,
        paths: &[SHACLPath],
        focus_node: &R::Term,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn inverse(
        store: &R,
        shape: &CompiledPropertyShape<R>,
        path: &SHACLPath,
        focus_node: &R::Term,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn zero_or_more(
        store: &R,
        shape: &CompiledPropertyShape<R>,
        path: &SHACLPath,
        focus_node: &R::Term,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn one_or_more(
        store: &R,
        shape: &CompiledPropertyShape<R>,
        path: &SHACLPath,
        focus_node: &R::Term,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn zero_or_one(
        store: &R,
        shape: &CompiledPropertyShape<R>,
        path: &SHACLPath,
        focus_node: &R::Term,
    ) -> Result<FocusNodes<R>, ValidateError>;
}
