use std::collections::HashSet;
use std::path::Path;
use std::str::FromStr;

use iri_s::IriS;
use oxiri::Iri;
use prefixmap::IriRef;
use shacl_ast::component::Component;
use shacl_ast::property_shape::PropertyShape;
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

pub trait ValidatorRunner<S: SRDF + SRDFBasic + 'static> {
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
        if S::term_is_bnode(node) {
            Err(ValidateError::TargetNodeBlankNode)
        } else {
            focus_nodes.insert(node.to_owned());
            Ok(())
        }
    }

    fn target_class(&self, class: &S::Term, focus_nodes: &mut HashSet<S::Term>) -> Result<()> {
        if let Some(_) = S::term_as_iri(class) {
            let subjects = match self
                .store()
                .subjects_with_predicate_object(&S::iri_s2iri(&srdf::RDF_TYPE), &class)
            {
                Ok(subjects) => subjects,
                Err(_) => return Err(ValidateError::SRDF),
            };
            let ans = subjects
                .into_iter()
                .map(|subject| S::subject_as_term(&subject))
                .collect::<HashSet<_>>();
            Ok(focus_nodes.extend(ans))
        } else {
            Err(ValidateError::TargetClassNotIri)
        }
    }

    fn target_subject_of(
        &self,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        let triples = match self
            .store()
            .triples_with_predicate(&S::iri_s2iri(&predicate.get_iri()?))
        {
            Ok(triples) => triples,
            Err(_) => return Err(ValidateError::SRDF),
        };
        let ans = triples
            .into_iter()
            .map(|triple| S::subject_as_term(&triple.subj()))
            .collect::<HashSet<_>>();
        Ok(focus_nodes.extend(ans))
    }

    fn target_object_of(
        &self,
        predicate: &IriRef,
        focus_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        let triples = match self
            .store()
            .triples_with_predicate(&S::iri_s2iri(&predicate.get_iri()?))
        {
            Ok(triples) => triples,
            Err(_) => return Err(ValidateError::SRDF),
        };
        let ans: HashSet<<S as SRDFBasic>::Term> = triples
            .into_iter()
            .map(|triple| triple.obj())
            .collect::<HashSet<_>>();
        Ok(focus_nodes.extend(ans))
    }

    fn path(
        &self,
        shape: &PropertyShape,
        node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        match shape.path() {
            SHACLPath::Predicate { pred } => self.predicate(shape, pred, node, value_nodes),
            SHACLPath::Alternative { paths } => self.alternative(shape, paths, node, value_nodes),
            SHACLPath::Sequence { paths } => self.sequence(shape, paths, node, value_nodes),
            SHACLPath::Inverse { path } => self.inverse(shape, path, node, value_nodes),
            SHACLPath::ZeroOrMore { path } => self.zero_or_more(shape, path, node, value_nodes),
            SHACLPath::OneOrMore { path } => self.one_or_more(shape, path, node, value_nodes),
            SHACLPath::ZeroOrOne { path } => self.zero_or_one(shape, path, node, value_nodes),
        }
    }

    fn predicate(
        &self,
        shape: &PropertyShape,
        predicate: &IriS,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        // let value_nodes = shape.get_value_nodes(self.store(), &focus_node, shape.path());
        // value_nodes.extend();
        // Ok(())
        todo!()
    }

    fn alternative(
        &self,
        shape: &PropertyShape,
        paths: &Vec<SHACLPath>,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        // paths
        //     .iter()
        //     .flat_map(|path| self.get_value_nodes(data_graph, &focus_node, path))
        //     .collect::<HashSet<_>>()
        todo!()
    }

    fn sequence(
        &self,
        shape: &PropertyShape,
        paths: &Vec<SHACLPath>,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn inverse(
        &self,
        shape: &PropertyShape,
        path: &Box<SHACLPath>,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn zero_or_more(
        &self,
        shape: &PropertyShape,
        path: &Box<SHACLPath>,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn one_or_more(
        &self,
        shape: &PropertyShape,
        path: &Box<SHACLPath>,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }

    fn zero_or_one(
        &self,
        shape: &PropertyShape,
        path: &Box<SHACLPath>,
        focus_node: S::Term,
        value_nodes: &mut HashSet<S::Term>,
    ) -> Result<()> {
        todo!()
    }
}

pub struct GraphValidatorRunner {
    store: SRDFGraph,
}

impl GraphValidatorRunner {
    pub fn new(path: &Path, rdf_format: RDFFormat, base: Option<&str>) -> Result<Self> {
        let store = match SRDFGraph::from_path(
            path,
            &rdf_format,
            match base {
                Some(base) => match Iri::from_str(&base) {
                    Ok(iri) => Some(iri),
                    Err(_) => None,
                },
                None => None,
            },
        ) {
            Ok(rdf) => rdf,
            Err(_) => return Err(ValidateError::GraphCreation),
        };
        Ok(GraphValidatorRunner { store })
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
    pub fn new(path: &String) -> Result<Self> {
        let store = match SRDFSparql::new(&IriS::new_unchecked(path)) {
            Ok(rdf) => rdf,
            Err(_) => return Err(ValidateError::SPARQLCreation),
        };
        Ok(SparqlValidatorRunner { store })
    }
}

impl ValidatorRunner<SRDFSparql> for SparqlValidatorRunner {
    fn store(&self) -> &SRDFSparql {
        &self.store
    }
}
