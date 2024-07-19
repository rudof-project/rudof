use iri_s::IriS;
use oxrdf::{Subject, Term};
use srdf::{SRDFGraph, Triple, SRDF};

mod constraints;
mod shacl_validation_vocab;
mod validate;
mod validation_report;

pub fn get_triple_with_predicate(graph: &SRDFGraph, predicate: &IriS) -> Triple<SRDFGraph> {
    match graph.triples_with_predicate(predicate.as_named_node()) {
        Ok(triples) => match triples.into_iter().nth(0) {
            Some(triple) => triple,
            None => todo!(),
        },
        Err(_) => todo!(),
    }
}

pub fn get_triples_with_predicate(graph: &SRDFGraph, predicate: &IriS) -> Vec<Triple<SRDFGraph>> {
    match graph.triples_with_predicate(predicate.as_named_node()) {
        Ok(triples) => triples,
        Err(_) => todo!(),
    }
}

// TODO: use the subject
pub fn object(graph: &SRDFGraph, _subject: &Subject, predicate: &IriS) -> Option<Term> {
    match graph.triples_with_predicate(predicate.as_named_node()) {
        Ok(triples) => match triples.into_iter().nth(0) {
            Some(triple) => Some(triple.obj()),
            None => None,
        },
        Err(_) => None,
    }
}

// TODO: use the subject
pub fn objects(graph: &SRDFGraph, _subject: &Subject, predicate: &IriS) -> Vec<Term> {
    match graph.triples_with_predicate(predicate.as_named_node()) {
        Ok(triples) => triples.iter().map(|triple| triple.obj()).collect(),
        Err(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use oxiri::Iri;
    use oxrdf::{BlankNode, Subject, Term};
    use shacl_ast::{Schema, ShaclParser};
    use srdf::{srdf_graph::SRDFGraph, RDFFormat, RDFS_LABEL};
    use std::{path::Path, str::FromStr};

    use crate::{
        shacl_validation_vocab::{
            MF_ACTION, MF_ENTRIES, MF_INCLUDE, MF_RESULT, SHT_DATA_GRAPH, SHT_SHAPES_GRAPH,
        },
        validate::validate,
        validation_report::ValidationReport,
    };

    use super::{get_triples_with_predicate, object};

    struct Manifest {
        base: String,
        graph: SRDFGraph,
        includes: Vec<Manifest>,
        entries: Vec<BlankNode>,
    }

    impl Manifest {
        fn new(
            base: String,
            graph: SRDFGraph,
            includes: Vec<Manifest>,
            entries: Vec<BlankNode>,
        ) -> Self {
            Manifest {
                base,
                graph,
                includes,
                entries,
            }
        }

        fn collect_tests(&self) -> Vec<ShaclTest> {
            let mut ans = Vec::new();

            let base = match Iri::from_str(&self.base) {
                Ok(base) => base,
                Err(_) => todo!(),
            };

            for entry in &self.entries {
                let label = object(
                    &self.graph,
                    &Subject::BlankNode(entry.to_owned()),
                    &RDFS_LABEL,
                );

                let action = match object(
                    &self.graph,
                    &Subject::BlankNode(entry.to_owned()),
                    &MF_ACTION,
                ) {
                    Some(Term::NamedNode(_)) => todo!(),
                    Some(Term::BlankNode(action)) => action,
                    Some(Term::Literal(_)) => todo!(),
                    None => todo!(),
                };

                let result = match object(
                    &self.graph,
                    &Subject::BlankNode(entry.to_owned()),
                    &MF_RESULT,
                ) {
                    Some(result) => ValidationReport::parse(
                        self.graph.to_owned(),
                        match result {
                            Term::NamedNode(named_node) => Subject::NamedNode(named_node),
                            Term::BlankNode(blank_node) => Subject::BlankNode(blank_node),
                            Term::Literal(_) => todo!(),
                        },
                    ),
                    None => todo!(),
                };

                let graph_term = match object(
                    &self.graph,
                    &Subject::BlankNode(action.to_owned()),
                    &SHT_DATA_GRAPH,
                ) {
                    Some(graph_term) => graph_term,
                    None => todo!(),
                };

                let shapes_term = match object(
                    &self.graph,
                    &Subject::BlankNode(action.to_owned()),
                    &SHT_SHAPES_GRAPH,
                ) {
                    Some(shapes_term) => shapes_term,
                    None => todo!(),
                };

                let mut shapes_graph = Schema::default();
                if shapes_term != graph_term {
                    let term = shapes_term.to_string().replace("file:://", "");
                    let mut chars = term.chars();
                    chars.next();
                    chars.next_back();

                    let rdf = match SRDFGraph::from_path(
                        Path::new(&chars.as_str().to_string()),
                        &RDFFormat::Turtle,
                        Some(base.clone()),
                    ) {
                        Ok(rdf) => rdf,
                        Err(_) => todo!(),
                    };

                    shapes_graph = match ShaclParser::new(rdf).parse() {
                        Ok(shapes_graph) => shapes_graph,
                        Err(_) => todo!(),
                    };
                }

                let term = graph_term.to_string();
                let mut chars = term.chars();
                chars.next();
                chars.next_back();
                let graph_chars = chars.as_str().to_string();

                let mut data_graph = self.graph.to_owned();
                if graph_chars != self.base {
                    let file_name = graph_chars.replace("file:://", "");
                    data_graph = match SRDFGraph::from_path(
                        Path::new(&file_name),
                        &RDFFormat::Turtle,
                        Some(base.clone()),
                    ) {
                        Ok(rdf) => rdf,
                        Err(_) => todo!(),
                    };
                }

                ans.push(ShaclTest::new(
                    Term::BlankNode(entry.clone()),
                    self.graph.to_owned(),
                    shapes_graph,
                    result,
                    data_graph,
                    match label {
                        Some(label) => Some(label.to_string()),
                        None => None,
                    },
                ))
            }

            ans
        }

        fn load(file: &str) -> Option<Manifest> {
            let path = Path::new(file);
            let base = match path.canonicalize() {
                Ok(path) => match path.to_str() {
                    Some(path) => format!("file:://{}", path),
                    None => todo!(),
                },
                Err(_) => todo!(),
            };

            let base_iri = match Iri::from_str(&base) {
                Ok(iri) => iri,
                Err(_) => todo!(),
            };

            let graph = match SRDFGraph::from_path(path, &RDFFormat::Turtle, Some(base_iri)) {
                Ok(graph) => graph,
                Err(_) => todo!(),
            };

            let includes = get_triples_with_predicate(&graph, &MF_INCLUDE);

            let mut include_manifests = Vec::new();
            for include in includes {
                let object = Self::clear_object(include, &base);
                let file = Self::clear_file(file, object);
                let child_manifest = Self::load(&file);
                if let Some(child_manifest) = child_manifest {
                    include_manifests.push(child_manifest);
                }
            }

            let entries = get_triples_with_predicate(&graph, &MF_ENTRIES);
            let mut entry_terms = Vec::new();
            for entry in entries {
                entry_terms.push(match entry.obj() {
                    Term::NamedNode(_) => todo!(),
                    Term::BlankNode(blank_node) => blank_node,
                    Term::Literal(_) => todo!(),
                })
            }

            Some(Manifest::new(base, graph, include_manifests, entry_terms))
        }

        fn flatten<'a>(manifest: &'a Manifest, manifests: &mut Vec<&'a Manifest>) {
            manifests.push(manifest);
            for manifest in &manifest.includes {
                Self::flatten(manifest, manifests);
            }
        }

        fn clear_object(triple: srdf::Triple<SRDFGraph>, base: &str) -> String {
            let base = base.replace("/manifest.ttl", "");
            let object = triple.obj().to_string();
            let split = object.split(&base).collect::<Vec<&str>>();
            let object = split[1];
            let mut chars = object.chars();
            chars.next();
            chars.next_back();
            chars.as_str().to_string()
        }

        fn clear_file(prev: &str, next: String) -> String {
            let prev = prev.replace("/manifest.ttl", "");
            format!("{}/{}", prev, next)
        }
    }

    struct ShaclTest {
        node: Term,
        graph: SRDFGraph,
        schema: Schema,
        result: ValidationReport,
        data_graph: SRDFGraph,
        label: Option<String>,
    }

    impl ShaclTest {
        fn new(
            node: Term,
            graph: SRDFGraph,
            schema: Schema,
            result: ValidationReport,
            data_graph: SRDFGraph,
            label: Option<String>,
        ) -> Self {
            ShaclTest {
                node,
                graph,
                schema,
                result,
                data_graph,
                label,
            }
        }
    }

    #[test]
    fn test_all() {
        let manifest = match Manifest::load("resources/tests/core/manifest.ttl") {
            Some(manifest) => manifest,
            None => todo!(),
        };

        let mut manifests = Vec::new();
        Manifest::flatten(&manifest, &mut manifests);

        let mut tests = Vec::new();
        for manifest in manifests {
            tests.extend(manifest.collect_tests());
        }

        let total = tests.len();
        let mut count = 0;
        for test in tests {
            match validate(&test.data_graph, test.schema) {
                Ok(actual) => {
                    if actual == test.result {
                        count += 1;
                    }
                }
                Err(_) => todo!(),
            };
        }

        println!("{}/{}", count, total);
    }
}
