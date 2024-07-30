use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

use ::srdf::RDFFormat;
use ::srdf::SHACLPath;
use ::srdf::SRDFGraph;
use iri_s::IriS;
use oxiri::Iri;
use prefixmap::IriRef;
use shacl_ast::component::Component;
use shacl_ast::target::Target;
use shacl_ast::Schema;
use shacl_ast::ShaclParser;

use crate::helper::term::Term;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

pub(crate) mod oxigraph;
pub(crate) mod srdf;

type Result<S> = std::result::Result<S, ValidateError>;

pub(crate) trait ValidatorRunner {
    fn new(path: &str, rdf_format: RDFFormat, base: Option<&str>) -> Result<Self>
    where
        Self: Sized;

    fn evaluate(
        &self,
        component: &Component,
        value_nodes: HashSet<Term>,
        report: &mut ValidationReport,
    ) -> Result<()>;

    fn target_node(&self, node: &Term, focus_nodes: &mut HashSet<Term>) -> Result<()>;

    fn target_class(&self, class: &Term, focus_nodes: &mut HashSet<Term>) -> Result<()>;

    fn target_subject_of(&self, predicate: &IriRef, focus_nodes: &mut HashSet<Term>) -> Result<()>;

    fn target_object_of(&self, predicate: &IriRef, focus_nodes: &mut HashSet<Term>) -> Result<()>;

    fn predicate(&self, predicate: &IriS, value_nodes: &mut HashSet<Term>) -> Result<()>;

    fn alternative(&self, paths: &Vec<SHACLPath>, value_nodes: &mut HashSet<Term>) -> Result<()>;

    fn sequence(&self, paths: &Vec<SHACLPath>, value_nodes: &mut HashSet<Term>) -> Result<()>;

    fn inverse(&self, path: &Box<SHACLPath>, value_nodes: &mut HashSet<Term>) -> Result<()>;

    fn zero_or_more(&self, path: &Box<SHACLPath>, value_nodes: &mut HashSet<Term>) -> Result<()>;

    fn one_or_more(&self, path: &Box<SHACLPath>, value_nodes: &mut HashSet<Term>) -> Result<()>;

    fn zero_or_one(&self, path: &Box<SHACLPath>, value_nodes: &mut HashSet<Term>) -> Result<()>;

    fn load_shapes_graph(
        &self,
        path: &str,
        rdf_format: RDFFormat,
        base: Option<&str>,
    ) -> Result<Schema> {
        let rdf = SRDFGraph::from_path(
            Path::new(&path),
            &RDFFormat::Turtle,
            match base {
                Some(base) => Some(Iri::from_str(&base)?),
                None => None,
            },
        )?;

        match ShaclParser::new(rdf).parse() {
            Ok(schema) => Ok(schema),
            Err(_) => Err(ValidateError::ShaclParser),
        }
    }

    fn focus_nodes(&self, targets: &[Target]) -> Result<HashSet<Term>> {
        let mut ans = HashSet::new();
        for target in targets.into_iter() {
            match target {
                Target::TargetNode(e) => self.target_node(&e.to_owned().into(), &mut ans)?,
                Target::TargetClass(e) => self.target_class(&e.to_owned().into(), &mut ans)?,
                Target::TargetSubjectsOf(pred) => self.target_subject_of(pred, &mut ans)?,
                Target::TargetObjectsOf(pred) => self.target_object_of(pred, &mut ans)?,
            }
        }
        Ok(ans)
    }

    fn path(&self, path: &SHACLPath) -> Result<HashSet<Term>> {
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
}
