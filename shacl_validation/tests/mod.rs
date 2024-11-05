use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::panic;
use std::path::Path;

use iri_s::IriS;
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
use sparql_service::RdfData;
use sparql_service::RdfDataError;
use srdf::RDFFormat;
use srdf::ReaderMode;
use srdf::SRDFBasic;
use srdf::SRDFGraph;
use srdf::SRDFGraphError;
use srdf::SRDF;
use thiserror::Error;

mod core;

struct ShaclTest {
    data: RdfData,
    shapes: Schema,
    report: ValidationReport,
}

impl ShaclTest {
    fn new(data: RdfData, shapes: Schema, report: ValidationReport) -> Self {
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
    fn new(path: &Path) -> Result<Self, TestSuite> {
        let base = match Path::new(path).canonicalize()?.to_str() {
            Some(path) => format!("file:/{}", path),
            None => panic!("Path not found!!"),
        };

        let term = RdfData::iri_s2term(&IriS::new_unchecked(&base));
        let subject = RdfData::term_as_subject(&term).unwrap();

        let graph = SRDFGraph::from_path(
            &Path::new(path),
            &RDFFormat::Turtle,
            Some(&base),
            &ReaderMode::Lax,
        )?;

        let store = RdfData::from_graph(graph)?;
        let entries = Manifest::parse_entries(&store, subject)?;

        let ans = Self {
            base,
            store,
            entries,
        };

        Ok(ans)
    }

    fn parse_entries(store: &RdfData, subject: OxSubject) -> Result<HashSet<OxTerm>, TestSuite> {
        let mut entry_terms = HashSet::new();

        let entry_subject = store
            .objects_for_subject_predicate(
                &subject,
                &RdfData::iri_s2iri(&shacl_validation_vocab::MF_ENTRIES),
            )?
            .into_iter()
            .next();

        if let Some(mut subject) = entry_subject {
            loop {
                let tmp = RdfData::term_as_subject(&subject).unwrap();

                match store
                    .objects_for_subject_predicate(&tmp, &RdfData::iri_s2iri(&srdf::RDF_FIRST))?
                    .into_iter()
                    .next()
                {
                    Some(terms) => entry_terms.insert(terms),
                    None => break,
                };

                subject = match store
                    .objects_for_subject_predicate(&tmp, &RdfData::iri_s2iri(&srdf::RDF_REST))?
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

    fn collect_tests(&self) -> Result<Vec<ShaclTest>, TestSuite> {
        let mut entries = Vec::new();
        for entry in &self.entries {
            let subject = RdfData::term_as_subject(entry).unwrap();

            let action = self
                .store
                .objects_for_subject_predicate(
                    &subject,
                    &RdfData::iri_s2iri(&shacl_validation_vocab::MF_ACTION),
                )?
                .into_iter()
                .next()
                .unwrap();

            let results = self
                .store
                .objects_for_subject_predicate(
                    &subject,
                    &RdfData::iri_s2iri(&shacl_validation_vocab::MF_RESULT),
                )?
                .into_iter()
                .next()
                .unwrap();

            let report = ValidationReport::parse(&self.store, results)?;

            let subject = RdfData::term_as_subject(&action).unwrap();

            let data_graph_iri = self
                .store
                .objects_for_subject_predicate(
                    &subject,
                    &RdfData::iri_s2iri(&shacl_validation_vocab::SHT_DATA_GRAPH),
                )?
                .into_iter()
                .next()
                .unwrap();

            let shapes_graph_iri = self
                .store
                .objects_for_subject_predicate(
                    &subject,
                    &RdfData::iri_s2iri(&shacl_validation_vocab::SHT_SHAPES_GRAPH),
                )?
                .into_iter()
                .next()
                .unwrap();

            let data_graph_path = Self::format_path(data_graph_iri.to_string());
            let shapes_graph_path = Self::format_path(shapes_graph_iri.to_string());

            let graph = SRDFGraph::from_path(
                data_graph_path,
                &RDFFormat::Turtle,
                Some(&self.base),
                &ReaderMode::default(),
            )?;
            let data_graph = RdfData::from_graph(graph)?;

            let rdf = SRDFGraph::from_reader(
                BufReader::new(File::open(shapes_graph_path)?),
                &RDFFormat::Turtle,
                Some(&self.base),
                &ReaderMode::default(),
            )?;
            let shapes_graph = ShaclParser::new(rdf).parse()?;

            entries.push(ShaclTest::new(data_graph, shapes_graph, report));
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

fn test(path: String, mode: ShaclValidationMode, slurp: bool) -> Result<(), TestSuite> {
    let manifest = Manifest::new(Path::new(&path))?;
    let tests = manifest.collect_tests()?;

    for test in tests {
        let validator = ShaclProcessor::new(test.data, mode, slurp);
        let report = validator.validate(&test.shapes.try_into()?)?;
        if report != test.report {
            return Err(TestSuite::NotEquals);
        }
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum TestSuite {
    #[error(transparent)]
    GraphCreation(#[from] SRDFGraphError),
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
