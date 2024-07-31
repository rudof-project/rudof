use std::collections::HashSet;
use std::path::Path;

use iri_s::IriS;
use oxrdf::Term;
use shacl_validation::runner::GraphValidatorRunner;
use shacl_validation::runner::ValidatorRunner;
use shacl_validation::shacl_validation_vocab;
use shacl_validation::validation_report::report::ValidationReport;
use srdf::RDFFormat;
use srdf::SRDFBasic;
use srdf::SRDFGraph;
use srdf::SRDF;

use crate::helper::srdf::get_object_for;
use crate::helper::srdf::get_objects_for;
use crate::manifest_error::ManifestError;
use crate::ShaclTest;

pub trait Manifest<S: SRDF + SRDFBasic> {
    fn new(base: String, store: S, includes: Vec<Self>, entries: HashSet<S::Term>) -> Self
    where
        Self: Sized;

    fn load_data_graph(path: &Path, base: &str) -> S;

    fn base(&self) -> String;

    fn store(&self) -> &S;

    fn includes(&self) -> Vec<Self>
    where
        Self: Sized;

    fn entries(&self) -> HashSet<S::Term>;

    fn collect_tests(&self) -> Result<Vec<ShaclTest<S>>, ManifestError> {
        let mut ans = Vec::new();

        for entry in &self.entries() {
            let label = get_object_for(self.store(), entry, &S::iri_s2iri(&srdf::RDFS_LABEL))?;

            let action = match get_object_for(
                self.store(),
                entry,
                &S::iri_s2iri(&shacl_validation_vocab::MF_ACTION),
            )? {
                Some(action) => match S::term_as_bnode(&action) {
                    Some(action) => S::bnode_as_term(action),
                    None => todo!(),
                },
                None => todo!(),
            };

            let result = match get_object_for(
                self.store(),
                entry,
                &S::iri_s2iri(&shacl_validation_vocab::MF_RESULT),
            )? {
                Some(result) => ValidationReport::parse(self.store(), result)?,
                None => todo!(),
            };

            let data_graph_iri = get_object_for(
                self.store(),
                &action,
                &S::iri_s2iri(&shacl_validation_vocab::SHT_DATA_GRAPH),
            )?
            .unwrap();

            let shapes_graph_iri = get_object_for(
                self.store(),
                &action,
                &S::iri_s2iri(&shacl_validation_vocab::SHT_SHAPES_GRAPH),
            )?
            .unwrap();

            let shapes = Self::format_path(shapes_graph_iri.to_string());
            let data = Self::format_path(data_graph_iri.to_string());

            ans.push(ShaclTest::new(
                data,
                shapes,
                Some(self.base()),
                result,
                match label {
                    Some(label) => Some(label.to_string()),
                    None => todo!(),
                },
            ))
        }

        Ok(ans)
    }

    fn load(path: &Path) -> Result<Self, ManifestError>
    where
        Self: Sized,
    {
        let base = match Path::new(path).canonicalize()?.to_str() {
            Some(path) => format!("file:/{}", path),
            None => todo!(),
        };

        let subject = S::iri_s2term(&IriS::new_unchecked(&base));
        let graph = Self::load_data_graph(path, &base);

        let mut includes = Vec::new();
        for manifest in get_objects_for(
            &graph,
            &subject,
            &S::iri_s2iri(&shacl_validation_vocab::MF_INCLUDE),
        )? {
            let format_path = Self::format_path(manifest.to_string());
            let path = Path::new(&format_path);
            if let Ok(child_manifest) = Self::load(path) {
                includes.push(child_manifest);
            }
        }

        let mut entry_terms = HashSet::new();

        let entry_subject = get_object_for(
            &graph,
            &subject,
            &S::iri_s2iri(&shacl_validation_vocab::MF_ENTRIES),
        )?;

        if let Some(mut subject) = entry_subject {
            loop {
                entry_terms.insert(
                    match get_object_for(&graph, &subject, &S::iri_s2iri(&srdf::RDF_FIRST))? {
                        Some(term) => term,
                        None => break,
                    },
                );

                subject = match get_object_for(&graph, &subject, &S::iri_s2iri(&srdf::RDF_REST))? {
                    Some(subject) => subject,
                    None => break,
                };
            }
        }

        Ok(Manifest::new(base, graph, includes, entry_terms))
    }

    fn flatten(manifest: Self, manifests: &mut Vec<Self>)
    where
        Self: Sized + Clone,
    {
        manifests.push(manifest.to_owned());
        for manifest in manifest.includes() {
            Self::flatten(manifest.to_owned(), manifests);
        }
    }

    fn format_path(term: String) -> String {
        let mut chars = term.chars();
        chars.next();
        chars.next_back();
        chars.as_str().to_string().replace("file:/", "")
    }
}

#[derive(Clone)]
pub struct GraphManifest {
    base: String,
    store: SRDFGraph,
    includes: Vec<GraphManifest>,
    entries: HashSet<Term>,
}

impl Manifest<SRDFGraph> for GraphManifest {
    fn new(
        base: String,
        store: SRDFGraph,
        includes: Vec<GraphManifest>,
        entries: HashSet<Term>,
    ) -> Self {
        GraphManifest {
            base,
            store,
            includes,
            entries,
        }
    }

    fn load_data_graph(path: &Path, base: &str) -> SRDFGraph {
        let path = Path::new(path);
        match GraphValidatorRunner::new(path, RDFFormat::Turtle, Some(base)) {
            Ok(validator) => validator,
            Err(_) => todo!(),
        }
        .store()
        .to_owned()
    }

    fn base(&self) -> String {
        self.base.to_owned()
    }

    fn store(&self) -> &SRDFGraph {
        &self.store
    }

    fn includes(&self) -> Vec<GraphManifest> {
        self.includes.to_owned()
    }

    fn entries(&self) -> HashSet<Term> {
        self.entries.to_owned()
    }
}
