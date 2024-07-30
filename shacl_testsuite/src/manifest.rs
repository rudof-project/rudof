use std::collections::HashSet;
use std::path::Path;

use indoc::formatdoc;
use oxigraph::model::NamedNode;
use oxigraph::model::Term as OxTerm;
use oxigraph::store::Store;
use shacl_ast::Schema;
use shacl_validation::shacl_validation_vocab;
use shacl_validation::validation_report::report::ValidationReport;
use srdf::RDFNode;
use srdf::SRDFBasic;
use srdf::SRDF;

use crate::helper::sparql::{select, select_many};
use crate::helper::srdf::get_object_for;
use crate::helper::srdf::get_objects_for;
use crate::manifest_error::ManifestError;
use crate::ShaclTest;

pub trait Manifest<S, T> {
    fn new(base: String, store: S, includes: Vec<Self>, entries: HashSet<T>) -> Self
    where
        Self: Sized;

    fn collect_tests(&self) -> Result<Vec<ShaclTest<S, T>>, ManifestError>;

    fn load(file: &str) -> Result<Self, ManifestError>
    where
        Self: Sized;

    fn includes(&self) -> Vec<Self>
    where
        Self: Sized;

    fn flatten<'a>(manifest: &'a Self, manifests: &mut Vec<&'a Self>)
    where
        Self: Sized,
    {
        manifests.push(manifest);
        for manifest in &manifest.includes() {
            Self::flatten(manifest, manifests);
        }
    }

    fn format_path(term: String) -> String {
        let mut chars = term.chars();
        chars.next();
        chars.next_back();
        chars.as_str().to_string().replace("file:/", "")
    }
}

pub struct OxigraphManifest {
    base: String,
    store: Store,
    includes: Vec<OxigraphManifest>,
    entries: HashSet<OxTerm>,
}

impl Manifest<Store, OxTerm> for OxigraphManifest {
    fn new(
        base: String,
        store: Store,
        includes: Vec<OxigraphManifest>,
        entries: HashSet<OxTerm>,
    ) -> Self {
        OxigraphManifest {
            base,
            store,
            includes,
            entries,
        }
    }

    fn collect_tests(&self) -> Result<Vec<ShaclTest<Store, OxTerm>>, ManifestError> {
        let mut ans = Vec::new();
        for entry in &self.entries {
            let query = formatdoc! {
                "
                    SELECT ?action ?result ?label
                    WHERE {{
                        {} {} ?action .
                        {} {} ?result .
                        OPTIONAL {{ {} {} ?label }}
                    }}
                ",
                entry, shacl_validation_vocab::MF_ACTION.as_named_node(), // check it is blank node
                entry, shacl_validation_vocab::MF_RESULT.as_named_node(), // check it is not literal
                entry, srdf::RDFS_LABEL.as_named_node(),
            };

            let solution = match select(&self.store, query) {
                Ok(solution) => solution,
                Err(_) => break,
            };

            let label = solution.get("label").map(|label| label.to_string());

            let action = match solution.get("action") {
                Some(action) => action,
                None => todo!(),
            };

            let result = match solution.get("result") {
                Some(result) => result,
                None => todo!(),
            };

            let query = formatdoc! {
                "
                    SELECT DISTINCT ?data_graph ?shapes_graph
                    WHERE {{
                        {} {} ?data_graph .
                        {} {} ?shapes_graph .
                    }}
                ", 
                action, shacl_validation_vocab::SHT_DATA_GRAPH.as_named_node(),
                action, shacl_validation_vocab::SHT_SHAPES_GRAPH.as_named_node(),
            };

            let solution = select(&self.store, query)?;

            let data_graph_iri = match solution.get("data_graph") {
                Some(data_graph) => data_graph.to_owned(),
                None => todo!(),
            };

            let shapes_graph_iri = match solution.get("shapes_graph") {
                Some(shapes_graph) => shapes_graph.to_owned(),
                None => todo!(),
            };

            let term = Self::format_path(shapes_graph_iri.to_string());

            // let rdf = SRDFGraph::from_path(
            //     Path::new(&path),
            //     &RDFFormat::Turtle,
            //     Some(Iri::from_str(&self.base)?),
            // )?;

            // let schema = match ShaclParser::new(rdf).parse() {
            //     Ok(shapes_graph) => shapes_graph,
            //     Err(_) => return Err(ManifestError::ShaclParser),
            // };

            let term = Self::format_path(data_graph_iri.to_string());

            // let mut data_store = self.store.clone(); // explicit copy
            // if path != self.base {
            //     data_store = Store::new()?;
            //     data_store.bulk_loader().load_graph(
            //         BufReader::new(File::open(path)?),
            //         GraphFormat::Turtle,
            //         GraphNameRef::DefaultGraph,
            //         Some(&self.base),
            //     )?;
            // }

            ans.push(ShaclTest::new(
                entry.to_owned(),
                self.store.to_owned(), // TODO: can this be removed?
                data_store,
                schema,
                ValidationReport::parse(&self.store, result)?,
                label,
            ))
        }
        Ok(ans)
    }

    fn load(file: &str) -> Result<OxigraphManifest, ManifestError> {
        let path = Path::new(file);

        let base = match path.canonicalize()?.to_str() {
            Some(path) => format!("file:/{}", path),
            None => todo!(),
        };

        let store = Store::new()?;

        // store.bulk_loader().load_graph(
        //     BufReader::new(File::open(path)?),
        //     GraphFormat::Turtle,
        //     GraphNameRef::DefaultGraph,
        //     Some(&base),
        // )?;

        let subject = OxTerm::NamedNode(NamedNode::new_unchecked(base.clone()));

        let query = formatdoc! {
            "
                SELECT ?this
                WHERE {{
                    {} {} ?this
                }}
            ", 
            subject, shacl_validation_vocab::MF_INCLUDE.as_named_node(),
        };

        let mut includes = Vec::new();
        for manifest in select_many(&store, query)? {
            let path = Self::format_path(manifest.to_string());
            if let Ok(child_manifest) = Self::load(path.as_str()) {
                includes.push(child_manifest);
            }
        }

        let query = formatdoc! {
            "
                SELECT ?this
                WHERE {{
                    {} {} ?this
                }}
            ", 
            subject, shacl_validation_vocab::MF_ENTRIES.as_named_node(),
        };

        let mut entries = HashSet::new();
        if let Ok(query_result) = select(&store, query) {
            let query = formatdoc! {"
                PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
                SELECT ?this
                WHERE {{
                    {} rdf:rest*/rdf:first ?this
                }}
                ", query_result.get("this").unwrap()
            };

            for entry in select_many(&store, query)? {
                if let OxTerm::NamedNode(_) = entry {
                    entries.insert(entry);
                }
            }
        }

        Ok(Manifest::new(base, store, includes, entries))
    }

    fn includes(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        self.includes
    }
}

pub struct SRDFManifest<S: SRDF + SRDFBasic> {
    base: String,
    store: S,
    includes: Vec<SRDFManifest<S>>,
    entries: HashSet<RDFNode>,
}

impl<S: SRDF + SRDFBasic> Manifest<S, RDFNode> for SRDFManifest<S> {
    fn new(
        base: String,
        store: S,
        includes: Vec<SRDFManifest<S>>,
        entries: HashSet<RDFNode>,
    ) -> Self {
        SRDFManifest {
            base,
            store,
            includes,
            entries,
        }
    }

    fn collect_tests(&self) -> Result<Vec<ShaclTest<S, RDFNode>>, ManifestError> {
        let mut ans = Vec::new();

        for entry in &self.entries {
            let label = get_object_for(
                &self.store,
                &Subject::NamedNode(entry.to_owned()),
                &srdf::RDFS_LABEL,
            )?;

            let action = match get_object_for(
                &self.store,
                &Subject::NamedNode(entry.to_owned()),
                &shacl_validation_vocab::MF_ACTION,
            ) {
                Ok(Some(Term::BlankNode(action))) => action,
                Ok(Some(Term::NamedNode(_))) => todo!(),
                Ok(Some(Term::Literal(_))) => todo!(),
                _ => todo!(),
            };

            let result = match get_object_for(
                &self.store,
                &Subject::NamedNode(entry.to_owned()),
                &shacl_validation_vocab::MF_RESULT,
            ) {
                Ok(result) => ValidationReport::parse(
                    self.store,
                    match result {
                        Some(Term::NamedNode(named_node)) => Subject::NamedNode(named_node),
                        Some(Term::BlankNode(blank_node)) => Subject::BlankNode(blank_node),
                        Some(Term::Literal(_)) => todo!(),
                        None => todo!(),
                    },
                )?,
                _ => todo!(),
            };

            let data_graph_iri = get_object_for(
                &self.store,
                &Subject::BlankNode(action.to_owned()),
                &shacl_validation_vocab::SHT_DATA_GRAPH,
            )?;

            let shapes_graph_iri = get_object_for(
                &self.store,
                &Subject::BlankNode(action.to_owned()),
                &shacl_validation_vocab::SHT_SHAPES_GRAPH,
            )?;

            let term = Self::format_path(shapes_graph_iri.to_string());

            let mut shapes_graph = Schema::default();

            // let rdf = match SRDFGraph::from_path(
            //     Path::new(&chars.as_str().to_string()),
            //     &RDFFormat::Turtle,
            //     Some(base.clone()),
            // ) {
            //     Ok(rdf) => rdf,
            //     Err(_) => todo!(),
            // };

            // shapes_graph = match ShaclParser::new(rdf).parse() {
            //     Ok(shapes_graph) => shapes_graph,
            //     Err(_) => todo!(),
            // };

            let term = Self::format_path(data_graph_iri.to_string());

            let mut data_graph = self.graph.to_owned();
            if term != self.base {
                // data_graph = match SRDFGraph::from_path(
                //     Path::new(&file_name),
                //     &RDFFormat::Turtle,
                //     Some(base.clone()),
                // ) {
                //     Ok(rdf) => rdf,
                //     Err(_) => todo!(),
                // };
            }

            ans.push(ShaclTest::new(
                RDFNode::NamedNode(entry.clone()),
                self.store.to_owned(),
                shapes_graph,
                result,
                data_graph,
                match label {
                    Some(label) => Some(label.to_string()),
                    None => None,
                },
            ))
        }

        Ok(ans)
    }

    fn load(file: &str) -> Result<SRDFManifest<S>, ManifestError> {
        let path = Path::new(file);

        let base = match path.canonicalize()?.to_str() {
            Some(path) => format!("file:/{}", path),
            None => todo!(),
        };

        let base_subject = Subject::NamedNode(NamedNode::new_unchecked(base.to_owned()));
        let base_iri = Iri::from_str(&base)?;
        let graph = SRDFGraph::from_path(path, &RDFFormat::Turtle, Some(base_iri))?;

        let mut includes = Vec::new();
        for manifest in get_objects_for(&graph, &base_subject, &shacl_validation_vocab::MF_INCLUDE)?
        {
            let path = Self::format_path(manifest.to_string());
            if let Ok(child_manifest) = Self::load(path.as_str()) {
                includes.push(child_manifest);
            }
        }

        let mut entry_terms = Vec::new();
        let entry_subject = get_subject_for(&graph, &base_subject, &MF_ENTRIES)?;

        if let Some(mut subject) = entry_subject {
            loop {
                entry_terms.push(match get_object_for(&graph, &subject, &RDF_FIRST)? {
                    Some(Term::NamedNode(named_node)) => named_node,
                    Some(Term::BlankNode(_)) => todo!(),
                    Some(Term::Literal(_)) => todo!(),
                    None => break,
                });
                subject = match get_subject_for(&graph, &subject, &RDF_REST)? {
                    Some(subject) => subject,
                    None => break,
                };
            }
        }

        Ok(Manifest::new(base, graph, includes, entry_terms))
    }

    fn includes(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        self.includes
    }
}
