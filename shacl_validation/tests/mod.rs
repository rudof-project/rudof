use std::collections::HashSet;
use std::io::Error;
use std::path::Path;

use oxrdf::NamedNode;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use oxrdf::TryFromTermError;
use shacl_ast::compiled::compiled_shacl_error::CompiledShaclError;
use shacl_ast::shacl_parser_error::ShaclParserError;
use shacl_ast::Schema;
use shacl_ast::ShaclParser;
use shacl_validation::engine::Engine;
use shacl_validation::shacl_processor::ShaclProcessor;
use shacl_validation::shacl_validation_vocab;
use shacl_validation::validate_error::ValidateError;
use shacl_validation::validation_report::report::ValidationReport;
use shacl_validation::validation_report::validation_report_error::ReportError;
use sparql_service::RdfData;
use srdf::matcher::Any;
use srdf::Query;
use srdf::RDFFormat;
use srdf::Rdf;
use srdf::ReaderMode;
use srdf::SRDFGraph;
use srdf::Triple;
use thiserror::Error;

mod core;

struct ShaclTest<R: Rdf> {
    data: R,
    shapes: Schema,
    report: ValidationReport,
}

impl<R: Rdf> ShaclTest<R> {
    fn new(data: R, shapes: Schema, report: ValidationReport) -> Self {
        ShaclTest {
            data,
            shapes,
            report,
        }
    }
}

pub struct Manifest {
    base: String,
    store: RdfData,
    entries: HashSet<OxTerm>,
}

impl Manifest {
    fn new(path: &Path) -> Result<Self, TestSuiteError> {
        let base = match Path::new(path).canonicalize()?.to_str() {
            Some(path) => format!("file:/{}", path),
            None => panic!("Path not found!!"),
        };

        let graph = SRDFGraph::from_path(
            Path::new(path),
            &RDFFormat::Turtle,
            Some(&base),
            &srdf::ReaderMode::Lax,
        )
        .map_err(|_| TestSuiteError::GraphCreation)?;

        let store = RdfData::from_graph(graph).map_err(|_| TestSuiteError::StoreCreation)?;
        let entries =
            Manifest::parse_entries(&store, NamedNode::new_unchecked(base.clone()).into())?;

        Ok(Self {
            base,
            store,
            entries,
        })
    }

    fn parse_entries(
        store: &RdfData,
        subject: OxSubject,
    ) -> Result<HashSet<OxTerm>, TestSuiteError> {
        let mut entry_terms = HashSet::new();

        let mf_entries: NamedNode = shacl_validation_vocab::MF_ENTRIES.clone().into();
        let entry_subject = store
            .triples_matching(subject, mf_entries, Any)
            .map_err(|_| TestSuiteError::Query)?
            .map(Triple::into_object)
            .next();

        if let Some(mut subject) = entry_subject {
            loop {
                let inner_subject: OxSubject = subject.clone().try_into().unwrap();
                let rdf_first: NamedNode = srdf::RDF_FIRST.clone().into();
                match store
                    .triples_matching(inner_subject.clone(), rdf_first, Any)
                    .map_err(|_| TestSuiteError::Query)?
                    .map(Triple::into_object)
                    .next()
                {
                    Some(terms) => entry_terms.insert(terms),
                    None => break,
                };

                let rdf_rest: NamedNode = srdf::RDF_REST.clone().into();
                subject = match store
                    .triples_matching(inner_subject, rdf_rest, Any)
                    .map_err(|_| TestSuiteError::Query)?
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

    fn collect_tests(&self) -> Result<Vec<ShaclTest<RdfData>>, TestSuiteError> {
        let mut entries = Vec::new();
        for entry in &self.entries {
            let entry: OxSubject = entry.clone().try_into()?;

            let mf_action: NamedNode = shacl_validation_vocab::MF_ACTION.clone().into();
            let action: OxSubject = self
                .store
                .triples_matching(entry.clone(), mf_action, Any)
                .map_err(|_| TestSuiteError::Query)?
                .map(Triple::into_object)
                .next()
                .unwrap()
                .try_into()?;

            let mf_result: NamedNode = shacl_validation_vocab::MF_RESULT.clone().into();
            let results = self
                .store
                .triples_matching(entry, mf_result, Any)
                .map_err(|_| TestSuiteError::Query)?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let report = ValidationReport::parse(&self.store, results)?;

            let sht_data_graph: NamedNode = shacl_validation_vocab::SHT_DATA_GRAPH.clone().into();
            let data_graph_iri = self
                .store
                .triples_matching(action.clone(), sht_data_graph, Any)
                .map_err(|_| TestSuiteError::Query)?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let sht_shapes_graph: NamedNode =
                shacl_validation_vocab::SHT_SHAPES_GRAPH.clone().into();
            let shapes_graph_iri = self
                .store
                .triples_matching(action, sht_shapes_graph, Any)
                .map_err(|_| TestSuiteError::Query)?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let data_graph_path = Self::format_path(data_graph_iri.to_string());
            let shapes_graph_path = Self::format_path(shapes_graph_iri.to_string());

            let graph = SRDFGraph::from_path(
                Path::new(&data_graph_path),
                &RDFFormat::Turtle,
                Some(&self.base),
                &ReaderMode::default(),
            )
            .map_err(|_| TestSuiteError::GraphCreation)?;
            let data_graph =
                RdfData::from_graph(graph).map_err(|_| TestSuiteError::StoreCreation)?;

            let shapes = SRDFGraph::from_path(
                Path::new(&shapes_graph_path),
                &RDFFormat::Turtle,
                Some(&self.base),
                &ReaderMode::default(),
            )
            .map_err(|_| TestSuiteError::GraphCreation)?;
            let schema = ShaclParser::new(shapes).parse()?;

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

fn test<E: Engine<RdfData>>(
    path: String,
    // subsetting: Subsetting,
) -> Result<(), TestSuiteError> {
    let manifest = Manifest::new(Path::new(&path))?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        let validator = ShaclProcessor::<RdfData, E>::new(test.data);
        let report = validator.validate(&test.shapes.try_into()?)?;
        if report != test.report {
            return Err(TestSuiteError::NotEquals);
        }
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum TestSuiteError {
    #[error(transparent)]
    ReportParsing(#[from] ReportError),

    #[error(transparent)]
    InputOutput(#[from] Error),

    #[error("Error when creating the RDF data graph")]
    GraphCreation,

    #[error("Error when creating the RDF data store")]
    StoreCreation,

    #[error("Error when querying the RDF data store")]
    Query,

    #[error(transparent)]
    CompilingShapes(#[from] CompiledShaclError),

    #[error(transparent)]
    Validation(#[from] ValidateError),

    #[error(transparent)]
    ParsingShape(#[from] ShaclParserError),

    #[error("The actual and expected ValidationReports are not equals")]
    NotEquals,

    #[error(transparent)]
    TryFromTerm(#[from] TryFromTermError),
}
