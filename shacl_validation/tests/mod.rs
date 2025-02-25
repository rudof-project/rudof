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
use srdf::Query;
use srdf::RDFFormat;
use srdf::Rdf;
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

        let subject = OxSubject::NamedNode(NamedNode::new_unchecked(base.clone()));

        let graph = Graph::from_path(
            &Path::new(path),
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

        let entry_subject = store
            .objects_for_subject_predicate(
                &subject,
                &shacl_validation_vocab::MF_ENTRIES.clone().into(),
            )?
            .into_iter()
            .next();

        if let Some(mut subject) = entry_subject {
            loop {
                match store
                    .objects_for_subject_predicate(
                        &subject.clone().try_into().unwrap(),
                        &srdf::RDF_FIRST.clone().into(),
                    )?
                    .into_iter()
                    .next()
                {
                    Some(terms) => entry_terms.insert(terms),
                    None => break,
                };

                subject = match store
                    .objects_for_subject_predicate(
                        &subject.clone().try_into().unwrap(),
                        &srdf::RDF_REST.clone().into(),
                    )?
                    .into_iter()
                    .next()
                {
                    Some(subject) => subject,
                    None => break,
                };
            }
        }

        Ok(entry_terms)
    }

    fn collect_tests(&self) -> Result<Vec<ShaclTest<RdfData>>, TestSuiteError> {
        let mut entries = Vec::new();
        for entry in &self.entries {
            let entry = entry.clone().try_into()?;

            let action = self
                .store
                .objects_for_subject_predicate(
                    &entry,
                    &shacl_validation_vocab::MF_ACTION.clone().into(),
                )?
                .into_iter()
                .next()
                .unwrap();
            let action = action.try_into()?;

            let results = self
                .store
                .objects_for_subject_predicate(
                    &entry,
                    &shacl_validation_vocab::MF_RESULT.clone().into(),
                )?
                .into_iter()
                .next()
                .unwrap();

            let report = ValidationReport::parse(&self.store, results)?;

            let data_graph_iri = self
                .store
                .objects_for_subject_predicate(
                    &action,
                    &shacl_validation_vocab::SHT_DATA_GRAPH.clone().into(),
                )?
                .into_iter()
                .next()
                .unwrap();

            let shapes_graph_iri = self
                .store
                .objects_for_subject_predicate(
                    &action,
                    &shacl_validation_vocab::SHT_SHAPES_GRAPH.clone().into(),
                )?
                .into_iter()
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
