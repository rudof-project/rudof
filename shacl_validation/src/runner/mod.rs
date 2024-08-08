use std::collections::HashSet;

use shacl_ast::component::Component;
use shacl_ast::property_shape::PropertyShape;
use shacl_ast::target::Target;
use shacl_ast::Schema;
use srdf::SHACLPath;
use srdf::SRDFBasic;

use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

pub mod sparql_runner;
pub mod srdf_runner;

type Result<T> = std::result::Result<T, ValidateError>;
pub type FocusNode<S> = HashSet<<S as SRDFBasic>::Term>;

pub trait ValidatorRunner<S: SRDFBasic> {
    fn evaluate(
        &self,
        store: &S,
        schema: &Schema,
        component: &Component,
        value_nodes: &HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<bool>;

    fn focus_nodes(&self, store: &S, targets: &[Target]) -> Result<FocusNode<S>> {
        let mut ans = FocusNode::<S>::new();
        for target in targets.iter() {
            match target {
                Target::TargetNode(node) => {
                    let node = &S::object_as_term(node);
                    self.target_node(store, node, &mut ans)?
                }
                Target::TargetClass(class) => {
                    let class = &S::object_as_term(class);
                    self.target_class(store, class, &mut ans)?
                }
                Target::TargetSubjectsOf(predicate) => {
                    let predicate = S::iri_s2iri(&predicate.get_iri()?);
                    self.target_subject_of(store, &predicate, &mut ans)?
                }
                Target::TargetObjectsOf(predicate) => {
                    let predicate = S::iri_s2iri(&predicate.get_iri()?);
                    self.target_object_of(store, &predicate, &mut ans)?
                }
            }
        }
        Ok(ans)
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

    fn path(
        &self,
        store: &S,
        shape: &PropertyShape,
        focus: &S::Term,
        values: &mut HashSet<S::Term>,
    ) -> Result<()> {
        match shape.path() {
            SHACLPath::Predicate { pred } => {
                let predicate = S::iri_s2iri(pred);
                self.predicate(store, shape, &predicate, focus, values)
            }
            SHACLPath::Alternative { paths } => {
                self.alternative(store, shape, paths, focus, values)
            }
            SHACLPath::Sequence { paths } => self.sequence(store, shape, paths, focus, values),
            SHACLPath::Inverse { path } => self.inverse(store, shape, path, focus, values),
            SHACLPath::ZeroOrMore { path } => self.zero_or_more(store, shape, path, focus, values),
            SHACLPath::OneOrMore { path } => self.one_or_more(store, shape, path, focus, values),
            SHACLPath::ZeroOrOne { path } => self.zero_or_one(store, shape, path, focus, values),
        }
    }

    fn predicate(
        &self,
        store: &S,
        shape: &PropertyShape,
        predicate: &S::IRI,
        focus_node: &S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn alternative(
        &self,
        store: &S,
        shape: &PropertyShape,
        paths: &[SHACLPath],
        focus_node: &S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn sequence(
        &self,
        store: &S,
        shape: &PropertyShape,
        paths: &[SHACLPath],
        focus_node: &S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn inverse(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn zero_or_more(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn one_or_more(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn zero_or_one(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: &S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;
}
