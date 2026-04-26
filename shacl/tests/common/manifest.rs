use crate::common::error::TestSuiteError;
use crate::common::test_instance::TestInstance;
use oxrdf::{NamedNode, NamedOrBlankNode, Term};
use rudof_rdf::rdf_core::term::Triple;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, ShaclTestVocab, TestManifestVocab};
use rudof_rdf::rdf_core::{Any, NeighsRDF, RDFFormat};
use shacl::rdf::ShaclParser;
use shacl::validator::report::ValidationReport;
use shacl::validator::store::{Graph, Store};
use sparql_service::RdfData;
use std::collections::HashSet;
use std::path::Path;

pub(crate) struct Manifest {
    base: String,
    store: RdfData,
    entries: HashSet<Term>,
}

impl Manifest {
    pub fn new(path: &Path) -> Result<Self, TestSuiteError> {
        println!("...Loading manifest from path: {path:?}");
        let base = path
            .canonicalize()
            .map_err(|e| TestSuiteError::Validation(e.to_string()))?;
        println!("Base path canonicalized successfully: {base:?}");

        let base = match base.to_str() {
            None => panic!("Path not found!!"),
            Some(path) => format!("file:/{path}"),
        };

        let subject = NamedOrBlankNode::NamedNode(NamedNode::new_unchecked(base.clone()));

        println!("Base IRI for manifest: {base}");
        let graph = Graph::from_path(path, &RDFFormat::Turtle, Some(&base))
            .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

        println!("Graph loaded successfully");

        let mut store = graph.store().clone();

        let entries = Manifest::parse_entries(&mut store, subject)?;
        println!("Entries parsed successfully: {entries:#?}");
        Ok(Self { base, store, entries })
    }

    fn format_path(term: String) -> String {
        let mut chars = term.chars();
        chars.next();
        chars.next_back();
        chars.as_str().to_string().replace("file:/", "")
    }

    fn parse_entries(store: &mut RdfData, subject: NamedOrBlankNode) -> Result<HashSet<Term>, TestSuiteError> {
        let mut entry_terms = HashSet::new();

        let mf_entries: NamedNode = TestManifestVocab::mf_entries().into();
        let entry_subject = store
            .triples_matching(&subject, &mf_entries, &Any)?
            .map(Triple::into_object)
            .next();

        if let Some(mut subject) = entry_subject {
            loop {
                let inner_subject: NamedOrBlankNode = subject.clone().try_into()?;
                let rdf_first: NamedNode = RdfVocab::rdf_first().into();
                match store
                    .triples_matching(&inner_subject, &rdf_first, &Any)?
                    .map(Triple::into_object)
                    .next()
                {
                    None => break,
                    Some(terms) => entry_terms.insert(terms),
                };

                let rdf_rest: NamedNode = RdfVocab::rdf_rest().into();
                subject = match store
                    .triples_matching(&inner_subject, &rdf_rest, &Any)?
                    .map(Triple::into_object)
                    .next()
                {
                    None => break,
                    Some(term) => term,
                };
            }
        }

        Ok(entry_terms)
    }

    pub fn collect_tests(&mut self) -> Result<Vec<TestInstance<RdfData>>, TestSuiteError> {
        let mut entries = Vec::new();

        for entry in &self.entries {
            let entry: NamedOrBlankNode = match entry.clone() {
                Term::NamedNode(nn) => NamedOrBlankNode::NamedNode(nn),
                Term::BlankNode(bn) => NamedOrBlankNode::BlankNode(bn),
                _ => return Err(TestSuiteError::Validation("Invalid entry term in manifest".to_string())),
            };

            let mf_action: NamedNode = TestManifestVocab::mf_action().into();
            let action: NamedOrBlankNode = match self
                .store
                .triples_matching(&entry, &mf_action, &Any)
                .map_err(<sparql_service::RdfDataError as Into<TestSuiteError>>::into)?
                .map(Triple::into_object)
                .next()
                .unwrap()
            {
                Term::NamedNode(nn) => NamedOrBlankNode::NamedNode(nn),
                Term::BlankNode(bn) => NamedOrBlankNode::BlankNode(bn),
                _ => {
                    return Err(TestSuiteError::Validation(
                        "Invalid action term in manifest".to_string(),
                    ));
                },
            };

            let mf_result: NamedNode = TestManifestVocab::mf_result().into();
            let results = self
                .store
                .triples_with_subject_predicate(&entry, &mf_result)
                .map_err(<sparql_service::RdfDataError as Into<TestSuiteError>>::into)?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let report = ValidationReport::parse(&mut self.store, results)
                .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

            let sht_data_graph: NamedNode = ShaclTestVocab::sht_data_graph().into();
            let data_graph_iri = self
                .store
                .triples_with_subject_predicate(&action, &sht_data_graph)
                .map_err(<sparql_service::RdfDataError as Into<TestSuiteError>>::into)?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let sht_shapes_graph: NamedNode = ShaclTestVocab::sht_shapes_graph().into();
            let shapes_graph_iri = self
                .store
                .triples_with_subject_predicate(&action, &sht_shapes_graph)
                .map_err(<sparql_service::RdfDataError as Into<TestSuiteError>>::into)?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let data_graph_path = Self::format_path(data_graph_iri.to_string());
            let shapes_graph_path = Self::format_path(shapes_graph_iri.to_string());

            let graph = Graph::from_path(Path::new(&data_graph_path), &RDFFormat::Turtle, Some(&self.base))
                .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

            let shapes = Graph::from_path(Path::new(&shapes_graph_path), &RDFFormat::Turtle, Some(&self.base))
                .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

            let schema = ShaclParser::new(shapes.store().clone())
                .parse()
                .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

            entries.push(TestInstance::new(graph.store().clone(), schema, report));
        }

        Ok(entries)
    }
}
