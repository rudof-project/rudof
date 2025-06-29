use std::collections::HashSet;
use std::io::Error;
use std::path::Path;

use oxrdf::NamedNode;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use oxrdf::TryFromTermError;
use shacl_ast::Schema;
use shacl_ir::compiled::compiled_shacl_error::CompiledShaclError;
use shacl_rdf::shacl_parser_error::ShaclParserError;
use shacl_rdf::ShaclParser;
use shacl_validation::shacl_processor::RdfDataValidation;
use shacl_validation::shacl_processor::ShaclProcessor;
use shacl_validation::shacl_processor::ShaclValidationMode;
use shacl_validation::shacl_validation_vocab;
use shacl_validation::store::graph::Graph;
use shacl_validation::store::Store;
use shacl_validation::validate_error::ValidateError;
use shacl_validation::validation_report::report::ValidationReport;
use shacl_validation::validation_report::validation_report_error::ReportError;
use sparql_service::RdfData;
use sparql_service::RdfDataError;
use srdf::matcher::Any;
use srdf::NeighsRDF;
use srdf::RDFFormat;
use srdf::Rdf;
use srdf::Triple;
use thiserror::Error;

mod core;

struct ShaclTest<R: Rdf> {
    data: R,
    shapes: Schema<R>,
    report: ValidationReport,
}

impl<R: Rdf> ShaclTest<R> {
    fn new(data: R, shapes: Schema<R>, report: ValidationReport) -> Self {
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

        let subject = OxSubject::NamedNode(NamedNode::new_unchecked(base.clone()));

        let graph = Graph::from_path(
            Path::new(path),
            RDFFormat::Turtle,
            Some(&base),
            // &ReaderMode::Lax,
        )?;

        let store = graph.store().clone();

        let entries = Manifest::parse_entries(&store, subject)?;

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
            .triples_matching(subject, mf_entries, Any)?
            .map(Triple::into_object)
            .next();

        if let Some(mut subject) = entry_subject {
            loop {
                let inner_subject: OxSubject = subject.clone().try_into().unwrap();
                let rdf_first: NamedNode = srdf::rdf_first().clone().into();
                match store
                    .triples_matching(inner_subject.clone(), rdf_first, Any)?
                    .map(Triple::into_object)
                    .next()
                {
                    Some(terms) => entry_terms.insert(terms),
                    None => break,
                };

                let rdf_rest: NamedNode = srdf::rdf_rest().clone().into();
                subject = match store
                    .triples_matching(inner_subject, rdf_rest, Any)?
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
                .triples_matching(entry.clone(), mf_action, Any)?
                .map(Triple::into_object)
                .next()
                .unwrap()
                .try_into()?;

            let mf_result: NamedNode = shacl_validation_vocab::MF_RESULT.clone().into();
            let results = self
                .store
                .triples_matching(entry, mf_result, Any)?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let report = ValidationReport::parse(&self.store, results)?;

            let sht_data_graph: NamedNode = shacl_validation_vocab::SHT_DATA_GRAPH.clone().into();
            let data_graph_iri = self
                .store
                .triples_matching(action.clone(), sht_data_graph, Any)?
                .map(Triple::into_object)
                .next()
                .unwrap();

            let sht_shapes_graph: NamedNode =
                shacl_validation_vocab::SHT_SHAPES_GRAPH.clone().into();
            let shapes_graph_iri = self
                .store
                .triples_matching(action, sht_shapes_graph, Any)?
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
            )?;
            let data_graph = graph.store().clone();

            let shapes = Graph::from_path(
                Path::new(&shapes_graph_path),
                RDFFormat::Turtle,
                Some(&self.base),
                // &ReaderMode::default(),
            )?;
            let shapes_graph = shapes.store().clone();
            let schema = ShaclParser::new(shapes_graph).parse()?;

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

fn test(
    path: String,
    mode: ShaclValidationMode,
    // subsetting: Subsetting,
) -> Result<(), TestSuiteError> {
    let manifest = Manifest::new(Path::new(&path))?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        let validator = RdfDataValidation::from_rdf_data(test.data, mode);
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

    #[error(transparent)]
    RdfData(#[from] RdfDataError),

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
