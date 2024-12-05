use shacl_ast::compiled::component::CompiledComponent;
use shacl_ast::compiled::property_shape::CompiledPropertyShape;
use shacl_ast::compiled::shape::CompiledShape;
use shacl_ast::shacl_path::SHACLPath;
use shacl_ast::target::Target;
use srdf::model::rdf::Iri;
use srdf::model::rdf::Object;
use srdf::model::rdf::Predicate;
use srdf::model::rdf::Rdf;

use crate::focus_nodes::FocusNodes;
use crate::store::Store;
use crate::validate_error::ValidateError;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use crate::Subsetting;

pub mod native;
pub mod sparql;

pub trait Engine<R: Rdf> {
    fn evaluate(
        &self,
        store: &Store<R>,
        shape: &CompiledShape<R>,
        component: &CompiledComponent<R>,
        value_nodes: &ValueNodes<R>,
        subsetting: &Subsetting,
    ) -> Result<Vec<ValidationResult<R>>, ValidateError>;

    fn focus_nodes(
        &self,
        store: &Store<R>,
        shape: &CompiledShape<R>,
        targets: &[Target<R>],
    ) -> Result<FocusNodes<R>, ValidateError> {
        let explicit = targets
            .iter()
            .flat_map(|target| match target {
                Target::TargetNode(node) => match self.target_node(store, node) {
                    Ok(target_node) => Some(target_node),
                    Err(_) => None,
                },
                Target::TargetClass(class) => match self.target_class(store, class) {
                    Ok(target_node) => Some(target_node),
                    Err(_) => None,
                },
                Target::TargetSubjectsOf(predicate) => {
                    match self.target_subject_of(store, predicate) {
                        Ok(target_subject_of) => Some(target_subject_of),
                        Err(_) => None,
                    }
                }
                Target::TargetObjectsOf(predicate) => {
                    match self.target_object_of(store, predicate) {
                        Ok(target_node) => Some(target_node),
                        Err(_) => None,
                    }
                }
            })
            .flatten();

        // we have to also look for implicit class FocusNodes, which are a "special"
        // kind of target declarations...
        let implicit = self.implicit_target_class(store, shape)?;

        Ok(FocusNodes::new(implicit.into_iter().chain(explicit)))
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(
        &self,
        store: &Store<R>,
        node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn target_class(
        &self,
        store: &Store<R>,
        class: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn target_subject_of(
        &self,
        store: &Store<R>,
        predicate: &Predicate<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn target_object_of(
        &self,
        store: &Store<R>,
        predicate: &Predicate<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn implicit_target_class(
        &self,
        store: &Store<R>,
        shape: &CompiledShape<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn path(
        &self,
        store: &Store<R>,
        shape: &CompiledPropertyShape<R>,
        focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError> {
        match shape.path() {
            SHACLPath::Predicate { pred } => self.predicate(store, shape, pred, focus_node),
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
        store: &Store<R>,
        shape: &CompiledPropertyShape<R>,
        predicate: &Iri<R::Triple>,
        focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn alternative(
        &self,
        store: &Store<R>,
        shape: &CompiledPropertyShape<R>,
        paths: &[SHACLPath<R::Triple>],
        focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn sequence(
        &self,
        store: &Store<R>,
        shape: &CompiledPropertyShape<R>,
        paths: &[SHACLPath<R::Triple>],
        focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn inverse(
        &self,
        store: &Store<R>,
        shape: &CompiledPropertyShape<R>,
        path: &SHACLPath<R::Triple>,
        focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn zero_or_more(
        &self,
        store: &Store<R>,
        shape: &CompiledPropertyShape<R>,
        path: &SHACLPath<R::Triple>,
        focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn one_or_more(
        &self,
        store: &Store<R>,
        shape: &CompiledPropertyShape<R>,
        path: &SHACLPath<R::Triple>,
        focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;

    fn zero_or_one(
        &self,
        store: &Store<R>,
        shape: &CompiledPropertyShape<R>,
        path: &SHACLPath<R::Triple>,
        focus_node: &Object<R>,
    ) -> Result<FocusNodes<R>, ValidateError>;
}
