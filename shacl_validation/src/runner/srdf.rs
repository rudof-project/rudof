use std::collections::HashSet;

use iri_s::IriS;
use prefixmap::IriRef;
use shacl_ast::component::Component;
use srdf::RDFFormat;
use srdf::SHACLPath;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::constraints::ConstraintComponent;
use crate::helper::term::Term;
use crate::validation_report::report::ValidationReport;

use super::ValidatorRunner;

pub(crate) struct SRDFRunner<S: SRDF> {
    store: S,
}

impl<S: SRDF + SRDFBasic> ValidatorRunner for SRDFRunner<S> {
    fn new(path: &str, rdf_format: RDFFormat, base: Option<&str>) -> super::Result<Self> {
        todo!()
    }

    fn evaluate(
        &self,
        component: &Component,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> super::Result<()> {
        Ok(component.evaluate(&self.store, value_nodes, report)?)
    }

    fn target_node(&self, node: &Term, focus_nodes: &mut HashSet<Term>) -> super::Result<()> {
        todo!()
    }

    fn target_class(&self, class: &Term, focus_nodes: &mut HashSet<Term>) -> super::Result<()> {
        todo!()
    }

    fn target_subject_of(
        &self,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }

    fn target_object_of(
        &self,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }

    fn predicate(&self, predicate: &IriS, value_nodes: &mut HashSet<Term>) -> super::Result<()> {
        todo!()
    }

    fn alternative(
        &self,
        paths: &Vec<SHACLPath>,
        value_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }

    fn sequence(
        &self,
        paths: &Vec<SHACLPath>,
        value_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }

    fn inverse(&self, path: &Box<SHACLPath>, value_nodes: &mut HashSet<Term>) -> super::Result<()> {
        todo!()
    }

    fn zero_or_more(
        &self,
        path: &Box<SHACLPath>,
        value_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }

    fn one_or_more(
        &self,
        path: &Box<SHACLPath>,
        value_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }

    fn zero_or_one(
        &self,
        path: &Box<SHACLPath>,
        value_nodes: &mut HashSet<Term>,
    ) -> super::Result<()> {
        todo!()
    }
}
