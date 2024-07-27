use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use indoc::formatdoc;
use oxigraph::io::GraphFormat;
use oxigraph::model::{GraphNameRef, NamedNode};
use oxigraph::{model::Term, store::Store};
use oxiri::Iri;
use shacl_ast::ShaclParser;
use shacl_validation::shacl_validation_vocab;
use shacl_validation::validation_report::report::ValidationReport;
use srdf::{RDFFormat, SRDFGraph};

use crate::helper::sparql::{select, select_many};
use crate::manifest_error::ManifestError;
use crate::ShaclTest;

pub struct Manifest {
    base: String,
    store: Store,
    includes: Vec<Manifest>,
    entries: HashSet<Term>,
}

impl Manifest {
    fn new(base: String, store: Store, includes: Vec<Manifest>, entries: HashSet<Term>) -> Self {
        Manifest {
            base,
            store,
            includes,
            entries,
        }
    }

    pub fn collect_tests(&self) -> Result<Vec<ShaclTest>, ManifestError> {
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

            // TODO: explain this
            let term = shapes_graph_iri.to_string().replace("file:/", "");
            let mut chars = term.chars();
            chars.next();
            chars.next_back();
            let path = chars.as_str().to_string();

            let rdf = SRDFGraph::from_path(
                Path::new(&path),
                &RDFFormat::Turtle,
                Some(Iri::from_str(&self.base)?),
            )?;

            let schema = match ShaclParser::new(rdf).parse() {
                Ok(shapes_graph) => shapes_graph,
                Err(_) => return Err(ManifestError::ShaclParser),
            };

            // TODO: explain this
            let term = data_graph_iri.to_string();
            let mut chars = term.chars();
            chars.next();
            chars.next_back();
            let path = chars.as_str().to_string();

            let mut data_store = self.store.clone(); // explicit copy
            if path != self.base {
                data_store = Store::new()?;
                data_store.bulk_loader().load_graph(
                    BufReader::new(File::open(path.replace("file:/", ""))?),
                    GraphFormat::Turtle,
                    GraphNameRef::DefaultGraph,
                    Some(&self.base),
                )?;
            }

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

    pub fn load(file: &str) -> Result<Manifest, ManifestError> {
        let path = Path::new(file);

        let base = match path.canonicalize()?.to_str() {
            Some(path) => format!("file:/{}", path),
            None => todo!(),
        };

        let store = Store::new()?;

        store.bulk_loader().load_graph(
            BufReader::new(File::open(path)?),
            GraphFormat::Turtle,
            GraphNameRef::DefaultGraph,
            Some(&base),
        )?;

        let subject = Term::NamedNode(NamedNode::new_unchecked(base.clone()));

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
            let file = manifest.to_string().replace("file:/", ""); // TODO: fn
            let mut chars = file.chars();
            chars.next();
            chars.next_back();
            if let Ok(child_manifest) = Self::load(chars.as_str()) {
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
                if let Term::NamedNode(_) = entry {
                    entries.insert(entry);
                }
            }
        }

        Ok(Manifest::new(base, store, includes, entries))
    }

    pub fn flatten<'a>(manifest: &'a Manifest, manifests: &mut Vec<&'a Manifest>) {
        manifests.push(manifest);
        for manifest in &manifest.includes {
            Self::flatten(manifest, manifests);
        }
    }
}
