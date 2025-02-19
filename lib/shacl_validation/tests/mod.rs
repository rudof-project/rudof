use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

use oxrdf::NamedNode;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use shacl_ast::compiled::compiled_shacl_error::CompiledShaclError;
use shacl_ast::shacl_parser_error::ShaclParserError;
use shacl_ast::Schema;
use shacl_ast::ShaclParser;
use shacl_validation::shacl_processor::ShaclProcessor;
use shacl_validation::shacl_processor::ShaclValidationMode;
use shacl_validation::shacl_validation_vocab;
use shacl_validation::validate_error::ValidateError;
use shacl_validation::validation_report::report::ValidationReport;
use shacl_validation::validation_report::validation_report_error::ReportError;
use shacl_validation::Subsetting;
use sparql_service::RdfData;
use sparql_service::RdfDataError;
use srdf::model::rdf::Rdf;
use srdf::model::reader::RdfReader;
use srdf::model::reader::ReaderMode;
use srdf::model::RdfFormat;
use srdf::oxgraph::OxGraph;
use srdf::oxgraph_error::GraphParseError;
use thiserror::Error;

mod core;

struct ShaclTest<R: Rdf> {
    data: R,
    shapes: Schema<R>,
    report: ValidationReport<R>,
}

impl<R: Rdf> ShaclTest<R> {
    fn new(data: R, shapes: Schema<R>, report: ValidationReport<R>) -> Self {
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

        let graph = OxGraph::from_path(
            &Path::new(path),
            RdfFormat::Turtle,
            Some(&base),
            &ReaderMode::Lax,
        )?;

        let store = RdfData::from_graph(graph);

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
            .triples_matching(Some(&subject), &shacl_validation_vocab::MF_ENTRIES, None)?
            .into_iter()
            .next();

        if let Some(mut subject) = entry_subject {
            loop {
                match store
                    .triples_matching(&subject, &srdf::RDF_FIRST, None)?
                    .into_iter()
                    .next()
                {
                    Some(terms) => entry_terms.insert(terms),
                    None => break,
                };

                subject = match store
                    .triples_matching(&subject, &srdf::RDF_REST, None)?
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

    fn collect_tests(&self) -> Result<Vec<ShaclTest>, TestSuiteError> {
        let mut entries = Vec::new();
        for entry in &self.entries {
            let entry = entry.try_into()?;

            let action = self
                .store
                .triples_matching(
                    &entry,
                    &RdfData::iri_s2iri(&shacl_validation_vocab::MF_ACTION),
                    None,
                )?
                .into_iter()
                .next()
                .unwrap();
            let action = action.try_into()?;

            let results = self
                .store
                .triples_matching(&entry, &shacl_validation_vocab::MF_RESULT, None)?
                .into_iter()
                .next()
                .unwrap();

            let report = ValidationReport::parse(&self.store, results)?;

            let data_graph_iri = self
                .store
                .triples_matching(&action, &shacl_validation_vocab::SHT_DATA_GRAPH, None)?
                .into_iter()
                .next()
                .unwrap();

            let shapes_graph_iri = self
                .store
                .triples_matching(&action, &shacl_validation_vocab::SHT_SHAPES_GRAPH, None)?
                .into_iter()
                .next()
                .unwrap();

            let data_graph_path = Self::format_path(data_graph_iri.to_string());
            let shapes_graph_path = Self::format_path(shapes_graph_iri.to_string());

            let graph = OxGraph::from_path(
                data_graph_path,
                RdfFormat::Turtle,
                Some(&self.base),
                &ReaderMode::default(),
            )?;
            let data_graph = RdfData::from_graph(graph);

            let shapes = OxGraph::from_reader(
                BufReader::new(File::open(shapes_graph_path)?),
                RdfFormat::Turtle,
                Some(&self.base),
                &ReaderMode::default(),
            )?;
            let shapes_graph = RdfData::from_graph(shapes);
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
    subsetting: Subsetting,
) -> Result<(), TestSuiteError> {
    let manifest = Manifest::new(Path::new(&path))?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        let validator = ShaclProcessor::new(test.data, mode, subsetting.clone());
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
    GraphParse(#[from] GraphParseError),

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
}
