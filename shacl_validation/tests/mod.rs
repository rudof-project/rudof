use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::panic;
use std::path::Path;

use iri_s::IriS;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use shacl_ast::Schema;
use shacl_ast::ShaclParser;
use shacl_validation::shacl_processor::ShaclProcessor;
use shacl_validation::shacl_processor::ShaclValidationMode;
use shacl_validation::shacl_validation_vocab;
use shacl_validation::validation_report::report::ValidationReport;
use sparql_service::RdfData;
use srdf::RDFFormat;
use srdf::ReaderMode;
use srdf::SRDFBasic;
use srdf::SRDFGraph;
use srdf::SRDF;

mod core;

struct ShaclTest {
    data: RdfData,
    shapes: Schema,
    base: Option<String>,
    report: ValidationReport,
    label: Option<String>,
}

impl ShaclTest {
    fn new(
        data: RdfData,
        shapes: Schema,
        base: Option<String>,
        report: ValidationReport,
        label: Option<String>,
    ) -> Self {
        ShaclTest {
            data,
            shapes,
            base,
            report,
            label,
        }
    }
}

pub struct Manifest {
    base: String,
    store: RdfData,
    entries: HashSet<OxTerm>,
}

impl Manifest {
    fn new(path: &Path) -> Self {
        let base = match Path::new(path).canonicalize().unwrap().to_str() {
            Some(path) => format!("file:/{}", path),
            None => panic!("Path not found!!"),
        };

        let term = RdfData::iri_s2term(&IriS::new_unchecked(&base));

        let subject = match RdfData::term_as_subject(&term) {
            Some(subject) => subject,
            None => todo!(),
        };

        let graph = SRDFGraph::from_path(
            &Path::new(path),
            &RDFFormat::Turtle,
            Some(&base),
            &ReaderMode::Lax,
        )
        .unwrap();

        let store = RdfData::from_graph(graph).unwrap();
        let entries = Manifest::parse_entries(&store, subject);

        Self {
            base,
            store,
            entries,
        }
    }

    fn parse_entries(store: &RdfData, subject: OxSubject) -> HashSet<OxTerm> {
        let mut entry_terms = HashSet::new();

        let entry_subject = store
            .objects_for_subject_predicate(
                &subject,
                &RdfData::iri_s2iri(&shacl_validation_vocab::MF_ENTRIES),
            )
            .unwrap()
            .into_iter()
            .next();

        if let Some(mut subject) = entry_subject {
            loop {
                let tmp = RdfData::term_as_subject(&subject).unwrap();

                match store
                    .objects_for_subject_predicate(
                        &tmp,
                        &RdfData::iri_s2iri(&shacl_validation_vocab::MF_ENTRIES),
                    )
                    .unwrap()
                    .into_iter()
                    .next()
                {
                    Some(terms) => entry_terms.insert(terms),
                    None => break,
                };

                subject = match store
                    .objects_for_subject_predicate(&tmp, &RdfData::iri_s2iri(&srdf::RDF_REST))
                    .unwrap()
                    .into_iter()
                    .next()
                {
                    Some(subject) => subject,
                    None => break,
                };
            }
        }

        entry_terms
    }

    fn collect_tests(&self) -> Vec<ShaclTest> {
        self.entries
            .iter()
            .map(|entry| {
                let subject = RdfData::term_as_subject(entry).unwrap();

                let label = self
                    .store
                    .objects_for_subject_predicate(&subject, &RdfData::iri_s2iri(&srdf::RDFS_LABEL))
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap();

                let action = self
                    .store
                    .objects_for_subject_predicate(
                        &subject,
                        &RdfData::iri_s2iri(&shacl_validation_vocab::MF_ACTION),
                    )
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap();

                let results = self
                    .store
                    .objects_for_subject_predicate(
                        &subject,
                        &RdfData::iri_s2iri(&shacl_validation_vocab::MF_RESULT),
                    )
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap();

                let report = ValidationReport::parse(&self.store, results).unwrap();

                let subject = RdfData::term_as_subject(&action).unwrap();

                let data_graph_iri = self
                    .store
                    .objects_for_subject_predicate(
                        &subject,
                        &RdfData::iri_s2iri(&shacl_validation_vocab::SHT_DATA_GRAPH),
                    )
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap();

                let shapes_graph_iri = self
                    .store
                    .objects_for_subject_predicate(
                        &subject,
                        &RdfData::iri_s2iri(&shacl_validation_vocab::SHT_SHAPES_GRAPH),
                    )
                    .unwrap()
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
                )
                .unwrap();
                let data_graph = RdfData::from_graph(graph).unwrap();

                let rdf = SRDFGraph::from_reader(
                    BufReader::new(File::open(shapes_graph_path).unwrap()),
                    &RDFFormat::Turtle,
                    Some(&self.base),
                    &ReaderMode::default(),
                )
                .unwrap();
                let shapes_graph = ShaclParser::new(rdf).parse().unwrap();

                ShaclTest::new(
                    data_graph,
                    shapes_graph,
                    Some(self.base.clone()),
                    report,
                    Some(label.to_string()),
                )
            })
            .collect()
    }

    fn format_path(term: String) -> String {
        let mut chars = term.chars();
        chars.next();
        chars.next_back();
        chars.as_str().to_string().replace("file:/", "")
    }
}

fn test(path: String, mode: ShaclValidationMode, slurp: bool) {
    let manifest = Manifest::new(Path::new(&path));
    let tests = manifest.collect_tests();

    for test in tests {
        let validator = ShaclProcessor::new(test.data, mode, slurp);
        let report = validator.validate(&test.shapes.try_into().unwrap());
        assert_eq!(report.unwrap(), test.report)
    }
}
