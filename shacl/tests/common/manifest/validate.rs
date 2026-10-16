use std::path::Path;
use oxrdf::NamedOrBlankNode;
use rudof_rdf::rdf_core::{RDFFormat, Rdf};
use rudof_rdf::rdf_core::vocabs::{ShaclTestVocab, TestManifestVocab};
use shacl::ast::ASTSchema;
use shacl::rdf::ShaclParser;
use shacl::validator::report::ValidationReport;
use shacl::validator::store::{Graph, Store};
use sparql_service::RdfData;
use crate::common::manifest::{Manifest, ManifestEntry, ManifestGraph};
use crate::common::TestSuiteError;

/// A manifest for `sht:Validate` tests
pub(crate) type ValidateManifest = Manifest<TestInstance<RdfData>>;

/// One `sht:Validate` test case
pub(crate) struct TestInstance<RDF: Rdf> {
    pub data: RDF,
    pub shapes: ASTSchema,
    pub report: ValidationReport,
}

impl<RDF: Rdf> TestInstance<RDF> {
    pub fn new(data: RDF, shapes: ASTSchema, report: ValidationReport) -> Self {
        Self { data, shapes, report }
    }
}

impl ManifestEntry for TestInstance<RdfData> {
    fn parse(manifest: &mut ManifestGraph, entry: NamedOrBlankNode) -> Result<Self, TestSuiteError> {
        let action: NamedOrBlankNode = manifest
            .required_value(&entry, &TestManifestVocab::mf_action().into())?
            .try_into()?;

        let results = manifest.required_value(&entry, &TestManifestVocab::mf_result().into())?;
        let report = ValidationReport::parse(manifest.store_mut(), results)
            .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

        let data_graph_iri = manifest.required_value(&action, &ShaclTestVocab::sht_data_graph().into())?;
        let shapes_graph_iri = manifest.required_value(&action, &ShaclTestVocab::sht_shapes_graph().into())?;

        let data_graph_path = graph_path(data_graph_iri.to_string());
        let shapes_graph_path = graph_path(shapes_graph_iri.to_string());
        let base = manifest.base();

        let data = Graph::from_path(Path::new(&data_graph_path), &RDFFormat::Turtle, Some(base))
            .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

        let shapes = Graph::from_path(Path::new(&shapes_graph_path), &RDFFormat::Turtle, Some(base))
            .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

        let schema = ShaclParser::new(shapes.store().clone())
            .parse()
            .map_err(|e| TestSuiteError::Validation(e.to_string()))?;

        Ok(Self::new(data.store().clone(), schema, report))

    }
}

/// Turns an IRI of a grpah into a path on disk
fn graph_path(term: String) -> String {
    let mut chars = term.chars();
    chars.next();
    chars.next_back();
    chars
        .as_str()
        .to_string()
        .replace("file:/", "")
}
