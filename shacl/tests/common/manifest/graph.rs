use crate::common::TestSuiteError;
use oxrdf::{NamedNode, NamedOrBlankNode, Term};
use rudof_rdf::rdf_core::term::Triple;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, TestManifestVocab};
use rudof_rdf::rdf_core::{Any, NeighsRDF, RDFFormat};
use shacl::validator::store::{Graph, Store};
use sparql_service::RdfData;
use std::path::Path;

/// A test manifest's graph,
pub(crate) struct ManifestGraph {
    base: String,
    store: RdfData,
    entries: Vec<Term>,
}

impl ManifestGraph {
    pub fn new(path: &Path) -> Result<Self, TestSuiteError> {
        let base = path.canonicalize()?;
        let base = match base.to_str() {
            None => panic!("Path not found!!"),
            Some(path) => format!("file:/{path}"),
        };

        let graph = Graph::from_path(path, &RDFFormat::Turtle, Some(&base))
            .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

        let mut manifest = Self {
            store: graph.store().clone(),
            base,
            entries: Vec::new(),
        };

        let subject = NamedOrBlankNode::NamedNode(NamedNode::new_unchecked(manifest.base.clone()));
        manifest.entries = manifest.parse_entries(&subject)?;

        Ok(manifest)
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn store(&self) -> &RdfData {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut RdfData {
        &mut self.store
    }

    pub fn entries(&self) -> &[Term] {
        &self.entries
    }

    pub fn value(&self, subject: &NamedOrBlankNode, property: &NamedNode) -> Result<Option<Term>, TestSuiteError> {
        Ok(self
            .store
            .triples_matching(subject, property, &Any)?
            .map(Triple::into_object)
            .next())
    }

    pub fn required_value(&self, subject: &NamedOrBlankNode, property: &NamedNode) -> Result<Term, TestSuiteError> {
        self.value(subject, property)?
            .ok_or_else(|| TestSuiteError::Validation(format!("{subject} has no {property}")))
    }

    pub fn list(&self, head: Term) -> Result<Vec<Term>, TestSuiteError> {
        let mut members = Vec::new();
        let mut current = head;

        while let Ok(subject) = TryInto::<NamedOrBlankNode>::try_into(current.clone()) {
            let rdf_first: NamedNode = RdfVocab::rdf_first().into();
            match self.value(&subject, &rdf_first)? {
                None => break,
                Some(member) => members.push(member),
            };

            let rdf_rest: NamedNode = RdfVocab::rdf_rest().into();
            current = match self.value(&subject, &rdf_rest)? {
                None => break,
                Some(rest) => rest,
            };
        }

        Ok(members)
    }

    fn parse_entries(&self, subject: &NamedOrBlankNode) -> Result<Vec<Term>, TestSuiteError> {
        let mf_entries: NamedNode = TestManifestVocab::mf_entries().into();

        match self.value(subject, &mf_entries)? {
            None => Ok(Vec::new()),
            Some(head) => self.list(head),
        }
    }
}
