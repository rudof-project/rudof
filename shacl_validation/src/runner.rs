use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

use iri_s::IriS;
use oxiri::Iri;
use prefixmap::IriRef;
use shacl_ast::component::Component;
use shacl_ast::target::Target;
use srdf::RDFFormat;
use srdf::SHACLPath;
use srdf::SRDFBasic;
use srdf::SRDFGraph;
use srdf::SRDFSparql;
use srdf::SRDF;

use crate::constraints::ConstraintComponent;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

type Result<T> = std::result::Result<T, ValidateError>;

pub trait ValidatorRunner<S: SRDF + SRDFBasic> {
    fn store(&self) -> &S;

    fn evaluate(
        &self,
        component: &Component,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<()> {
        let component: Box<dyn ConstraintComponent<S>> = component.into();
        Ok(component.evaluate(&self.store(), value_nodes, report)?)
    }

    fn focus_nodes(&self, targets: &[Target]) -> Result<HashSet<S::Term>> {
        let mut ans = HashSet::new();
        for target in targets.into_iter() {
            match target {
                Target::TargetNode(e) => self.target_node(&S::object_as_term(e), &mut ans)?,
                Target::TargetClass(e) => self.target_class(&S::object_as_term(e), &mut ans)?,
                Target::TargetSubjectsOf(e) => self.target_subject_of(e, &mut ans)?,
                Target::TargetObjectsOf(e) => self.target_object_of(e, &mut ans)?,
            }
        }
        Ok(ans)
    }

    fn target_node(&self, node: &S::Term, focus_nodes: &mut HashSet<S::Term>) -> Result<()> {
        todo!()
    }

    fn target_class(&self, class: &S::Term, focus_nodes: &mut HashSet<S::Term>) -> Result<()> {
        todo!()
    }

    fn target_subject_of(
        &self,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn target_object_of(
        &self,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn path(&self, path: &SHACLPath) -> Result<HashSet<S::Term>> {
        let mut ans = HashSet::new();
        match path {
            SHACLPath::Predicate { pred } => self.predicate(pred, &mut ans),
            SHACLPath::Alternative { paths } => self.alternative(paths, &mut ans),
            SHACLPath::Sequence { paths } => self.sequence(paths, &mut ans),
            SHACLPath::Inverse { path } => self.inverse(path, &mut ans),
            SHACLPath::ZeroOrMore { path } => self.zero_or_more(path, &mut ans),
            SHACLPath::OneOrMore { path } => self.one_or_more(path, &mut ans),
            SHACLPath::ZeroOrOne { path } => self.zero_or_one(path, &mut ans),
        };
        Ok(ans)
    }

    fn predicate(&self, predicate: &IriS, value_nodes: &mut HashSet<S::Term>) -> Result<()> {
        todo!()
    }

    fn alternative(
        &self,
        paths: &Vec<SHACLPath>,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn sequence(&self, paths: &Vec<SHACLPath>, value_nodes: &mut HashSet<S::Term>) -> Result<()> {
        todo!()
    }

    fn inverse(&self, path: &Box<SHACLPath>, value_nodes: &mut HashSet<S::Term>) -> Result<()> {
        todo!()
    }

    fn zero_or_more(
        &self,
        path: &Box<SHACLPath>,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn one_or_more(&self, path: &Box<SHACLPath>, value_nodes: &mut HashSet<S::Term>) -> Result<()> {
        todo!()
    }

    fn zero_or_one(&self, path: &Box<SHACLPath>, value_nodes: &mut HashSet<S::Term>) -> Result<()> {
        todo!()
    }
}

pub struct GraphValidatorRunner {
    store: SRDFGraph,
}

impl GraphValidatorRunner {
    pub fn new(path: &str, rdf_format: RDFFormat, base: Option<&str>) -> Self {
        let store = match SRDFGraph::from_path(
            Path::new(path),
            &RDFFormat::Turtle,
            match base {
                Some(base) => match Iri::from_str(&base) {
                    Ok(iri) => Some(iri),
                    Err(_) => None,
                },
                None => None,
            },
        ) {
            Ok(rdf) => rdf,
            Err(_) => todo!(),
        };
        GraphValidatorRunner { store }
    }
}

impl ValidatorRunner<SRDFGraph> for GraphValidatorRunner {
    fn store(&self) -> &SRDFGraph {
        &self.store
    }
}

pub struct SparqlValidatorRunner {
    store: SRDFSparql,
}

impl SparqlValidatorRunner {
    pub fn new(path: &str) -> Self {
        let store = match SRDFSparql::new(&IriS::new_unchecked(path)) {
            Ok(rdf) => rdf,
            Err(_) => todo!(),
        };
        SparqlValidatorRunner { store }
    }
}

impl ValidatorRunner<SRDFSparql> for SparqlValidatorRunner {
    fn store(&self) -> &SRDFSparql {
        &self.store
    }
}
