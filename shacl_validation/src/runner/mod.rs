use shacl_ast::property_shape::PropertyShape;
use shacl_ast::target::Target;
use srdf::SHACLPath;
use srdf::SRDFBasic;

use crate::shape::FocusNode;
use crate::shape::ValueNode;
use crate::validate_error::ValidateError;

pub mod default_runner;
pub mod query_runner;

pub type Result<T> = std::result::Result<T, ValidateError>;

pub trait ValidatorRunner<S: SRDFBasic> {
    fn focus_nodes(&self, store: &S, shape: &S::Term, targets: &[Target]) -> Result<FocusNode<S>> {
        let mut target_nodes = FocusNode::<S>::new();
        for target in targets.iter() {
            match target {
                Target::TargetNode(node) => {
                    let node = &S::object_as_term(node);
                    self.target_node(store, node, &mut target_nodes)?
                }
                Target::TargetClass(class) => {
                    let class = &S::object_as_term(class);
                    self.target_class(store, class, &mut target_nodes)?
                }
                Target::TargetSubjectsOf(predicate) => {
                    let predicate = S::iri_s2iri(&predicate.get_iri()?);
                    self.target_subject_of(store, &predicate, &mut target_nodes)?
                }
                Target::TargetObjectsOf(predicate) => {
                    let predicate = S::iri_s2iri(&predicate.get_iri()?);
                    self.target_object_of(store, &predicate, &mut target_nodes)?
                }
            }
        }
        // we have to also look for implicit class targets, which are a "special"
        // kind of target declarations...
        self.implicit_target_class(store, shape, &mut target_nodes)?;
        Ok(target_nodes)
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &S::Term, focus_nodes: &mut FocusNode<S>) -> Result<()>;

    fn target_class(
        &self,
        store: &S,
        class: &S::Term,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()>;

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &S::IRI,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()>;

    fn target_object_of(
        &self,
        store: &S,
        predicate: &S::IRI,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()>;

    fn implicit_target_class(
        &self,
        store: &S,
        shape: &S::Term,
        focus_nodes: &mut FocusNode<S>,
    ) -> Result<()>;

    fn path(
        &self,
        store: &S,
        shape: &PropertyShape,
        focus_node: &S::Term,
        values_nodes: &mut ValueNode<S>,
    ) -> Result<()> {
        match shape.path() {
            SHACLPath::Predicate { pred } => {
                let predicate = S::iri_s2iri(pred);
                self.predicate(store, shape, &predicate, focus_node, values_nodes)
            }
            SHACLPath::Alternative { paths } => {
                self.alternative(store, shape, paths, focus_node, values_nodes)
            }
            SHACLPath::Sequence { paths } => {
                self.sequence(store, shape, paths, focus_node, values_nodes)
            }
            SHACLPath::Inverse { path } => {
                self.inverse(store, shape, path, focus_node, values_nodes)
            }
            SHACLPath::ZeroOrMore { path } => {
                self.zero_or_more(store, shape, path, focus_node, values_nodes)
            }
            SHACLPath::OneOrMore { path } => {
                self.one_or_more(store, shape, path, focus_node, values_nodes)
            }
            SHACLPath::ZeroOrOne { path } => {
                self.zero_or_one(store, shape, path, focus_node, values_nodes)
            }
        }
    }

    fn predicate(
        &self,
        store: &S,
        shape: &PropertyShape,
        predicate: &S::IRI,
        focus_node: &S::Term,
        value_nodes: &mut ValueNode<S>,
    ) -> Result<()>;

    fn alternative(
        &self,
        store: &S,
        shape: &PropertyShape,
        paths: &[SHACLPath],
        focus_node: &S::Term,
        value_nodes: &mut ValueNode<S>,
    ) -> Result<()>;

    fn sequence(
        &self,
        store: &S,
        shape: &PropertyShape,
        paths: &[SHACLPath],
        focus_node: &S::Term,
        value_nodes: &mut ValueNode<S>,
    ) -> Result<()>;

    fn inverse(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
        value_nodes: &mut ValueNode<S>,
    ) -> Result<()>;

    fn zero_or_more(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
        value_nodes: &mut ValueNode<S>,
    ) -> Result<()>;

    fn one_or_more(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
        value_nodes: &mut ValueNode<S>,
    ) -> Result<()>;

    fn zero_or_one(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
        value_nodes: &mut ValueNode<S>,
    ) -> Result<()>;
}
