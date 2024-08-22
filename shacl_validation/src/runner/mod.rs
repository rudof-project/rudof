use shacl_ast::property_shape::PropertyShape;
use shacl_ast::target::Target;
use srdf::SHACLPath;
use srdf::SRDFBasic;

use crate::context::EvaluationContext;
use crate::context::ValidationContext;
use crate::validate_error::ValidateError;
use crate::validation_report::result::LazyValidationIterator;
use crate::Targets;
use crate::ValueNodes;

pub mod default_runner;
pub mod query_runner;

pub trait ValidatorRunner<S: SRDFBasic> {
    fn evaluate(
        &self,
        validation_context: &ValidationContext<S>,
        evaluation_context: EvaluationContext,
        value_nodes: &ValueNodes<S>,
    ) -> Result<LazyValidationIterator<S>, ValidateError>;

    fn focus_nodes(
        &self,
        store: &S,
        shape: &S::Term,
        targets: &[Target],
    ) -> Result<Targets<S>, ValidateError> {
        let explicit = targets
            .iter()
            .filter_map(move |target| match target {
                Target::TargetNode(node) => {
                    match self.target_node(&store, &S::object_as_term(node)) {
                        Ok(target_node) => Some(target_node),
                        Err(_) => None,
                    }
                }
                Target::TargetClass(class) => {
                    match self.target_class(&store, &S::object_as_term(class)) {
                        Ok(target_node) => Some(target_node),
                        Err(_) => None,
                    }
                }
                Target::TargetSubjectsOf(predicate) => {
                    let predicate = match predicate.get_iri() {
                        Ok(predicate) => S::iri_s2iri(&predicate),
                        Err(_) => return None,
                    };
                    match self.target_subject_of(&store, &predicate) {
                        Ok(target_subject_of) => Some(target_subject_of),
                        Err(_) => None,
                    }
                }
                Target::TargetObjectsOf(predicate) => {
                    let predicate = match predicate.get_iri() {
                        Ok(predicate) => S::iri_s2iri(&predicate),
                        Err(_) => return None,
                    };
                    match self.target_object_of(&store, &predicate) {
                        Ok(target_node) => Some(target_node),
                        Err(_) => None,
                    }
                }
            })
            .flatten();

        // we have to also look for implicit class targets, which are a "special"
        // kind of target declarations...
        let implicit = self.implicit_target_class(&store, shape)?;

        Ok(Targets::new(implicit.into_iter().chain(explicit)))
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &S::Term) -> Result<Targets<S>, ValidateError>;

    fn target_class(&self, store: &S, class: &S::Term) -> Result<Targets<S>, ValidateError>;

    fn target_subject_of(&self, store: &S, predicate: &S::IRI)
        -> Result<Targets<S>, ValidateError>;

    fn target_object_of(&self, store: &S, predicate: &S::IRI) -> Result<Targets<S>, ValidateError>;

    fn implicit_target_class(
        &self,
        store: &S,
        shape: &S::Term,
    ) -> Result<Targets<S>, ValidateError>;

    fn path(
        &self,
        store: &S,
        shape: &PropertyShape,
        focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError> {
        match shape.path() {
            SHACLPath::Predicate { pred } => {
                let predicate = S::iri_s2iri(pred);
                self.predicate(store, shape, &predicate, focus_node)
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
        shape: &PropertyShape,
        predicate: &S::IRI,
        focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError>;

    fn alternative(
        &self,
        store: &S,
        shape: &PropertyShape,
        paths: &[SHACLPath],
        focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError>;

    fn sequence(
        &self,
        store: &S,
        shape: &PropertyShape,
        paths: &[SHACLPath],
        focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError>;

    fn inverse(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError>;

    fn zero_or_more(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError>;

    fn one_or_more(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError>;

    fn zero_or_one(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
    ) -> Result<Targets<S>, ValidateError>;
}
