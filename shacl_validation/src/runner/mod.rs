use std::collections::HashSet;

use iri_s::IriS;
use prefixmap::IriRef;
use shacl_ast::component::Component;
use shacl_ast::property_shape::PropertyShape;
use shacl_ast::target::Target;
use srdf::SHACLPath;
use srdf::SRDFBasic;

use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

pub mod sparql_runner;
pub mod srdf_runner;

type Result<T> = std::result::Result<T, ValidateError>;

pub trait ValidatorRunner<S: SRDFBasic> {
    fn evaluate(
        &self,
        store: &S,
        component: &Component,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<()>;

    fn focus_nodes(&self, store: &S, targets: &[Target]) -> Result<HashSet<S::Term>> {
        let mut ans = HashSet::new();
        for target in targets.iter() {
            match target {
                Target::TargetNode(e) => {
                    let node = &S::object_as_term(e);
                    self.target_node(store, node, &mut ans)?
                }
                Target::TargetClass(class) => {
                    let class = &S::object_as_term(class);
                    self.target_class(store, class, &mut ans)?
                }
                Target::TargetSubjectsOf(e) => self.target_subject_of(store, e, &mut ans)?,
                Target::TargetObjectsOf(e) => self.target_object_of(store, e, &mut ans)?,
            }
        }
        Ok(ans)
    }

    fn target_node(
        &self,
        store: &S,
        node: &S::Term,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn target_class(
        &self,
        store: &S,
        class: &S::Term,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn target_subject_of(
        &self,
        store: &S,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn target_object_of(
        &self,
        store: &S,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn path(
        &self,
        store: &S,
        shape: &PropertyShape,
        focus: S::Term,
        values: &mut HashSet<S::Term>,
    ) -> Result<()> {
        match shape.path() {
            SHACLPath::Predicate { pred } => self.predicate(store, shape, pred, focus, values),
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
        predicate: &IriS,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn alternative(
        &self,
        store: &S,
        shape: &PropertyShape,
        paths: &[SHACLPath],
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn sequence(
        &self,
        store: &S,
        shape: &PropertyShape,
        paths: &[SHACLPath],
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn inverse(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn zero_or_more(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn one_or_more(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;

    fn zero_or_one(
        &self,
        store: &S,
        shape: &PropertyShape,
        path: &SHACLPath,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()>;
}
