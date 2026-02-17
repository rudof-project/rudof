use crate::common::shacl_test::ShaclTest;
use crate::common::testsuite_error::TestSuiteError;
use oxrdf::{NamedNode, NamedOrBlankNode as OxSubject, Term as OxTerm};
use rdf::rdf_core::{
    Any, NeighsRDF, RDFFormat,
    term::Triple,
    vocab::{rdf_first, rdf_rest},
};
use shacl_rdf::ShaclParser;
use shacl_validation::shacl_validation_vocab;
use shacl_validation::store::Store;
use shacl_validation::store::graph::Graph;
use shacl_validation::validation_report::report::ValidationReport;
use sparql_service::RdfData;
use std::{collections::HashSet, path::Path};

pub struct Manifest {
    base: String,
    store: RdfData,
    entries: HashSet<OxTerm>,
}

impl Manifest {
    pub fn new(path: &Path) -> Result<Self, Box<TestSuiteError>> {
        let base = Path::new(path)
            .canonicalize()
            .map_err(Box::new)
            .map_err(|e| TestSuiteError::Validation { error: e.to_string() })?;
        let base = match base.to_str() {
            Some(path) => format!("file:/{path}"),
            None => panic!("Path not found!!"),
        };

        let subject = OxSubject::NamedNode(NamedNode::new_unchecked(base.clone()));

        let graph = Graph::from_path(
            Path::new(path),
            RDFFormat::Turtle,
            Some(&base),
            // &ReaderMode::Lax,
        )
        .map_err(|e| TestSuiteError::Validation { error: e.to_string() })?;

        let mut store = graph.store().clone();

        let entries = Manifest::parse_entries(&mut store, subject)?;
        Ok(Self { base, store, entries })
    }

    fn parse_entries(store: &mut RdfData, subject: OxSubject) -> Result<HashSet<OxTerm>, Box<TestSuiteError>> {
        let mut entry_terms = HashSet::new();

        let mf_entries: NamedNode = shacl_validation_vocab::mf_entries().clone().into();
        let entry_subject = store
            .triples_matching(&subject, &mf_entries, &Any)
            .map_err(|e| Box::new(e.into()))?
            .map(Triple::into_object)
            .next();

        if let Some(mut subject) = entry_subject {
            loop {
                let inner_subject: OxSubject = subject.clone().try_into().unwrap();
                let rdf_first: NamedNode = rdf_first().clone().into();
                match store
                    .triples_matching(&inner_subject, &rdf_first, &Any)
                    .map_err(|e| Box::new(e.into()))?
                    .map(Triple::into_object)
                    .next()
                {
                    Some(terms) => entry_terms.insert(terms),
                    None => break,
                };

                let rdf_rest: NamedNode = rdf_rest().clone().into();
                subject = match store
                    .triples_matching(&inner_subject, &rdf_rest, &Any)
                    .map_err(|e| Box::new(e.into()))?
                    .map(Triple::into_object)
                    .next()
                {
                    Some(term) => term,
                    None => break,
                };
            }
        }

        Ok(entry_terms)
    }

    pub fn collect_tests(&mut self) -> Result<Vec<ShaclTest<RdfData>>, Box<TestSuiteError>> {
        let mut entries = Vec::new();
        for entry in &self.entries {
            let entry: OxSubject = match entry.clone() {
                OxTerm::NamedNode(nn) => OxSubject::NamedNode(nn),
                OxTerm::BlankNode(bn) => OxSubject::BlankNode(bn),
                _ => {
                    return Err(Box::new(TestSuiteError::Validation {
                        error: "Invalid entry term in manifest".to_string(),
                    }));
                },
            };

            let mf_action: NamedNode = shacl_validation_vocab::mf_action().clone().into();
            let action: OxSubject = match self
                .store
                .triples_matching(&entry, &mf_action, &Any)
                .map_err(|e| Box::new(e.into()))?
                .map(Triple::into_object)
                .next()
                .unwrap()
            {
                OxTerm::NamedNode(named_node) => OxSubject::NamedNode(named_node),
                OxTerm::BlankNode(blank_node) => OxSubject::BlankNode(blank_node),
                _ => {
                    return Err(Box::new(TestSuiteError::Validation {
                        error: "Invalid action term in manifest".to_string(),
                    }));
                },
            };

            let mf_result: NamedNode = shacl_validation_vocab::mf_result().clone().into();
            let results = self
                .store
                .triples_matching(&entry, &mf_result, &Any)
                .map_err(|e| Box::new(e.into()))?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let report = ValidationReport::parse(&mut self.store, results)
                .map_err(|e| Box::new(TestSuiteError::Validation { error: e.to_string() }))?;

            let sht_data_graph: NamedNode = shacl_validation_vocab::sht_data_graph().clone().into();
            let data_graph_iri = self
                .store
                .triples_matching(&action, &sht_data_graph, &Any)
                .map_err(|e| Box::new(e.into()))?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let sht_shapes_graph: NamedNode = shacl_validation_vocab::sht_shapes_graph().clone().into();
            let shapes_graph_iri = self
                .store
                .triples_matching(&action, &sht_shapes_graph, &Any)
                .map_err(|e| Box::new(e.into()))?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let data_graph_path = Self::format_path(data_graph_iri.to_string());
            let shapes_graph_path = Self::format_path(shapes_graph_iri.to_string());

            let graph = Graph::from_path(
                Path::new(&data_graph_path),
                RDFFormat::Turtle,
                Some(&self.base),
                // &ReaderMode::default(),
            )
            .map_err(|e| TestSuiteError::Validation { error: e.to_string() })?;

            let data_graph = graph.store().clone();

            let shapes = Graph::from_path(
                Path::new(&shapes_graph_path),
                RDFFormat::Turtle,
                Some(&self.base),
                // &ReaderMode::default(),
            )
            .map_err(|e| TestSuiteError::Validation { error: e.to_string() })?;
            let shapes_graph = shapes.store().clone();
            let schema = ShaclParser::new(shapes_graph)
                .parse()
                .map_err(|e| Box::new(TestSuiteError::Validation { error: e.to_string() }))?;

            entries.push(ShaclTest::new(data_graph, schema, report));
        }

        Ok(entries)
    }

    fn format_path(term: String) -> String {
        let mut chars = term.chars();
        chars.next();
        chars.next_back();
        chars.as_str().to_string().replace("file:/", "")
    }
}
