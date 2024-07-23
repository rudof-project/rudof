use iri_s::IriS;
use oxiri::Iri;
use oxrdf::{NamedNode, Subject, Term};
use shacl_ast::{Schema, ShaclParser};
use shacl_validation::{
    shacl_validation_vocab::{
        MF_ACTION, MF_ENTRIES, MF_INCLUDE, MF_RESULT, SHT_DATA_GRAPH, SHT_SHAPES_GRAPH,
    },
    validation_report::report::ValidationReport,
};
use srdf::{RDFFormat, SRDFGraph, RDFS_LABEL, RDF_FIRST, RDF_REST, SRDF};
use std::{collections::HashSet, path::Path, str::FromStr};

use crate::manifest_error::ManifestError;
use crate::ShaclTest;

pub struct Manifest {
    base: String,
    graph: SRDFGraph,
    includes: Vec<Manifest>,
    entries: Vec<NamedNode>,
}

impl Manifest {
    fn new(
        base: String,
        graph: SRDFGraph,
        includes: Vec<Manifest>,
        entries: Vec<NamedNode>,
    ) -> Self {
        Manifest {
            base,
            graph,
            includes,
            entries,
        }
    }

    fn get_objects_for(
        graph: &SRDFGraph,
        subject: &Subject,
        predicate: &IriS,
    ) -> Result<HashSet<Term>, ManifestError> {
        match graph.objects_for_subject_predicate(subject, predicate.as_named_node()) {
            Ok(triples) => Ok(triples),
            Err(_) => todo!(),
        }
    }

    fn get_object_for(
        graph: &SRDFGraph,
        subject: &Subject,
        predicate: &IriS,
    ) -> Result<Option<Term>, ManifestError> {
        let objects = Self::get_objects_for(graph, subject, predicate)?;
        match objects.into_iter().nth(0) {
            Some(object) => Ok(Some(object)),
            None => Ok(None),
        }
    }

    pub fn collect_tests(&self) -> Result<Vec<ShaclTest>, ManifestError> {
        let mut ans = Vec::new();

        let base = match Iri::from_str(&self.base) {
            Ok(base) => base,
            Err(_) => todo!(),
        };

        for entry in &self.entries {
            let label = match Self::get_object_for(
                &self.graph,
                &Subject::NamedNode(entry.to_owned()),
                &RDFS_LABEL,
            ) {
                Ok(label) => label,
                Err(_) => todo!(),
            };

            let action = match Self::get_object_for(
                &self.graph,
                &Subject::NamedNode(entry.to_owned()),
                &MF_ACTION,
            ) {
                Ok(Some(Term::BlankNode(action))) => action,
                Ok(Some(Term::NamedNode(_))) => todo!(),
                Ok(Some(Term::Literal(_))) => todo!(),
                _ => todo!(),
            };

            let result = match Self::get_object_for(
                &self.graph,
                &Subject::NamedNode(entry.to_owned()),
                &MF_RESULT,
            ) {
                Ok(result) => ValidationReport::parse(
                    self.graph.to_owned(),
                    match result {
                        Some(Term::NamedNode(named_node)) => Subject::NamedNode(named_node),
                        Some(Term::BlankNode(blank_node)) => Subject::BlankNode(blank_node),
                        Some(Term::Literal(_)) => todo!(),
                        None => todo!(),
                    },
                )?,
                _ => todo!(),
            };

            let graph_term = match Self::get_object_for(
                &self.graph,
                &Subject::BlankNode(action.to_owned()),
                &SHT_DATA_GRAPH,
            ) {
                Ok(Some(graph_term)) => graph_term,
                _ => todo!(),
            };

            let shapes_term = match Self::get_object_for(
                &self.graph,
                &Subject::BlankNode(action.to_owned()),
                &SHT_SHAPES_GRAPH,
            ) {
                Ok(Some(shapes_term)) => shapes_term,
                _ => todo!(),
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
                Term::NamedNode(entry.clone()),
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

        Ok(ans)
    }

    fn get_subject_for(
        graph: &SRDFGraph,
        subject: &Subject,
        predicate: &IriS,
    ) -> Result<Option<Subject>, ManifestError> {
        match Self::get_object_for(graph, subject, predicate) {
            Ok(entry) => match entry {
                Some(entry) => match entry {
                    Term::NamedNode(named_node) => Ok(Some(Subject::NamedNode(named_node))),
                    Term::BlankNode(blank_node) => Ok(Some(Subject::BlankNode(blank_node))),
                    Term::Literal(_) => todo!(),
                },
                None => Ok(None),
            },
            Err(_) => todo!(),
        }
    }

    pub fn load(file: &str) -> Result<Option<Manifest>, ManifestError> {
        let path = Path::new(file);

        let base = match path.canonicalize() {
            Ok(path) => match path.to_str() {
                Some(path) => format!("file:://{}", path),
                None => todo!(),
            },
            Err(_) => todo!(),
        };

        let base_subject = Subject::NamedNode(NamedNode::new_unchecked(base.to_owned()));
        let base_iri = Iri::from_str(&base)?;
        let graph = SRDFGraph::from_path(path, &RDFFormat::Turtle, Some(base_iri))?;
        let includes = Self::get_objects_for(&graph, &base_subject, &MF_INCLUDE)?;

        let mut include_manifests = Vec::new();
        for include in includes {
            let object = Self::clear_object(include, &base);
            let file = Self::clear_file(file, object);
            let child_manifest = Self::load(&file);
            if let Ok(Some(child_manifest)) = child_manifest {
                include_manifests.push(child_manifest);
            }
        }
        let mut entry_terms = Vec::new();
        let entry_subject = Self::get_subject_for(&graph, &base_subject, &MF_ENTRIES)?;

        if let Some(mut subject) = entry_subject {
            loop {
                entry_terms.push(match Self::get_object_for(&graph, &subject, &RDF_FIRST)? {
                    Some(Term::NamedNode(named_node)) => named_node,
                    Some(Term::BlankNode(_)) => todo!(),
                    Some(Term::Literal(_)) => todo!(),
                    None => break,
                });
                subject = match Self::get_subject_for(&graph, &subject, &RDF_REST)? {
                    Some(subject) => subject,
                    None => break,
                };
            }
        }

        Ok(Some(Manifest::new(
            base,
            graph,
            include_manifests,
            entry_terms,
        )))
    }

    pub fn flatten<'a>(manifest: &'a Manifest, manifests: &mut Vec<&'a Manifest>) {
        manifests.push(manifest);
        for manifest in &manifest.includes {
            Self::flatten(manifest, manifests);
        }
    }

    fn clear_object(object: Term, base: &str) -> String {
        let base = base.replace("/manifest.ttl", "");
        let object = object.to_string();
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
