use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use iri_s::IriS;
use oxrdfio::RdfParser;
use prefixmap::PrefixMap;

use crate::model::{Iri, Quad, Subject, Triple};
use crate::{MutableRdf, Object, Predicate, Rdf, RdfFormat, Triples};

use super::error::{GraphError, MutableGraphError};

/// Reader mode when parsing RDF data files
#[derive(Debug, PartialEq, Clone, Default)]
pub enum ReaderMode {
    /// Stops when there is an error
    #[default]
    Strict,
    /// Emits a warning and continues processing
    Lax,
}

impl ReaderMode {
    pub fn is_strict(&self) -> bool {
        matches!(self, ReaderMode::Strict)
    }
}

#[derive(Debug, Default, Clone)]
pub struct GenericGraph<T: Triple> {
    focus: Option<T::Term>,
    graph: HashSet<T>, // TODO: is a BTree better for larger datasets?
    pm: PrefixMap,
    base: Option<T::Iri>,
}

impl<T: Triple> GenericGraph<T> {
    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    pub fn quads<G>(&self) -> impl Iterator<Item = dyn Quad<Triple = T, GraphName = G>> + '_ {
        // let graph_name = GraphName::DefaultGraph;
        self.graph.iter().map(move |t| todo!())
    }

    pub fn merge_from_reader<R: Read>(
        &mut self,
        read: R,
        format: &RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), GraphError> {
        let mut reader = RdfParser::from_format(format.into()).for_reader(read);

        // first we parse all the triples
        for triple in &reader {
            let triple = triple?;
            let subject = triple.subject;
            let predicate = triple.predicate;
            let object = triple.object;
            self.add_triple(subject, predicate, object)?;
        }

        // then, we parse the rest of the stuff
        if let RdfFormat::Turtle = format {
            let prefixes: HashMap<&str, &str> = reader.prefixes().collect();
            self.base = match (&self.base, base) {
                (None, None) => None,
                (Some(b), None) => Some(b.clone()),
                (_, Some(b)) => Some(IriS::new_unchecked(b)),
            };
            let pm = PrefixMap::from_hashmap(&prefixes)?;
            self.merge_prefixes(pm)?;
        }

        Ok(())
    }

    pub fn merge_prefixes(&mut self, prefixmap: PrefixMap) -> Result<(), GraphError> {
        self.pm.merge(prefixmap)?;
        Ok(())
    }

    pub fn from_reader<R: Read>(
        read: R,
        format: &RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<GenericGraph<T>, GraphError> {
        let mut graph = GenericGraph::default();
        graph.merge_from_reader(read, format, base, reader_mode)?;
        Ok(graph)
    }

    pub fn merge_from_path<P: AsRef<Path>>(
        &mut self,
        path: P,
        format: &RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), GraphError> {
        let path_name = path.as_ref().display();
        let file = File::open(path.as_ref()).map_err(|e| GraphError::ReadingPathError {
            path_name: path_name.to_string(),
            error: e,
        })?;
        let reader = BufReader::new(file);
        Self::merge_from_reader(self, reader, format, base, reader_mode)?;
        Ok(())
    }

    pub fn from_path<P: AsRef<Path>>(
        path: P,
        format: &RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<GenericGraph, GraphError> {
        let path_name = path.as_ref().display();
        let file = File::open(path.as_ref()).map_err(|e| GraphError::ReadingPathError {
            path_name: path_name.to_string(),
            error: e,
        })?;
        let reader = BufReader::new(file);
        Self::from_reader(reader, format, base, reader_mode)
    }

    pub fn resolve(&self, str: &str) -> Result<Iri, GraphError> {
        let r = self.pm.resolve(str)?;
        Ok(Self::cnv_iri(r))
    }

    pub fn from_str(
        data: &str,
        format: &RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<GenericGraph<T>, GraphError> {
        Self::from_reader(std::io::Cursor::new(&data), format, base, reader_mode)
    }

    pub fn parse_data(
        data: &String,
        format: &RdfFormat,
        base: &Path,
        reader_mode: &ReaderMode,
    ) -> Result<GenericGraph<T>, GraphError> {
        let mut attempt = PathBuf::from(base);
        attempt.push(data);
        let base = Some("base:://");
        let data_path = &attempt;
        let graph = Self::from_path(data_path, format, base, reader_mode)?;
        Ok(graph)
    }

    pub fn prefixmap(&self) -> &PrefixMap {
        &self.pm
    }
}

impl<T: Triple> Rdf for GenericGraph<T> {
    type Triple = T;
    type Error = GraphError;

    fn triples_matching<'a>(
        &'a self,
        subject: Option<&'a Subject<Self>>,
        predicate: Option<&'a Predicate<Self>>,
        object: Option<&'a Object<Self>>,
    ) -> Result<Triples<'a, Self>, Self::Error> {
        let triples = self
            .0
            .iter()
            .filter(move |triple| match (subject, predicate, object) {
                (None, None, None) => true,
                (None, None, Some(obj)) => triple.obj() == obj,
                (None, Some(pred), None) => triple.pred() == pred,
                (None, Some(pred), Some(obj)) => triple.pred() == pred && triple.obj() == obj,
                (Some(subj), None, None) => triple.subj() == subj,
                (Some(subj), None, Some(obj)) => triple.subj() == subj && triple.obj() == obj,
                (Some(subj), Some(pred), None) => triple.subj() == subj && triple.pred() == pred,
                (Some(subj), Some(pred), Some(obj)) => {
                    triple.subj() == subj && triple.pred() == pred && triple.obj() == obj
                }
            });

        Ok(Box::new(triples))
    }
}

impl<T: Triple> MutableRdf for GenericGraph<T> {
    type MutableError = MutableGraphError;

    fn add_triple(
        &mut self,
        subject: Subject<Self>,
        predicate: Predicate<Self>,
        object: Object<Self>,
    ) -> Result<(), Self::MutableError> {
        self.0.insert(T::new(subject, predicate, object));
        Ok(())
    }

    fn remove_triple(&mut self, triple: &T) -> Result<(), Self::MutableError> {
        self.0.remove(triple);
        Ok(())
    }

    fn add_base(&mut self, base: &Predicate<Self>) -> Result<(), Self::Error> {
        todo!()
    }

    fn add_prefix(&mut self, alias: &str, iri: &Predicate<Self>) -> Result<(), Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // #[tokio::test]
    // async fn parse_get_predicates() {
    //     use crate::graph::graph::AsyncSRDF;

    //     let s = r#"PREFIX : <http://example.org/>
    //         PREFIX schema: <https://schema.org/>

    //         :alice schema:name "Alice" ;
    //                schema:knows :bob, :carol ;
    //                :age  23 .
    //      "#;
    //     let parsed_graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::Strict).unwrap();
    //     let alice = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/alice"));
    //     let knows = OxNamedNode::new_unchecked("https://schema.org/knows");
    //     let bag_preds = parsed_graph.get_predicates_subject(&alice).await.unwrap();
    //     assert!(bag_preds.contains(&knows));
    //     let bob = OxTerm::NamedNode(OxNamedNode::new_unchecked("http://example.org/bob"));
    //     let alice_knows =
    //         AsyncSRDF::get_objects_for_subject_predicate(&parsed_graph, &alice, &knows)
    //             .await
    //             .unwrap();
    //     assert!(alice_knows.contains(&bob));
    // }

    // #[test]
    // fn test_outgoing_arcs() {
    //     let s = r#"prefix : <http://example.org/>
    //     prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

    //     :x :p [ :p 1 ].
    //     "#;

    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::Strict).unwrap();
    //     let x = <GenericGraph as SRDFBasic>::iri_s2subject(&iri!("http://example.org/x"));
    //     let p = <GenericGraph as SRDFBasic>::iri_s2iri(&iri!("http://example.org/p"));
    //     let terms = rdf::Rdf::objects_for_subject_predicate(&graph, &x, &p).unwrap();
    //     let term = terms.iter().next().unwrap().clone();
    //     let subject = <GenericGraph as SRDFBasic>::term_as_subject(&term).unwrap();
    //     let outgoing = graph.outgoing_arcs(&subject).unwrap();
    //     let one = <GenericGraph as SRDFBasic>::object_as_term(&Object::Literal(int!(1)));
    //     assert_eq!(outgoing.get(&p), Some(&HashSet::from([one])))
    // }

    // #[test]
    // fn test_outgoing_arcs_bnode() {
    //     let s = r#"prefix : <http://example.org/>
    //     prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

    //     :x :p [ :p 1 ].
    //     "#;

    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::Strict).unwrap();
    //     let x = <GenericGraph as SRDFBasic>::iri_s2subject(&iri!("http://example.org/x"));
    //     let p = <GenericGraph as SRDFBasic>::iri_s2iri(&iri!("http://example.org/p"));
    //     let terms = rdf::Rdf::objects_for_subject_predicate(&graph, &x, &p).unwrap();
    //     let term = terms.iter().next().unwrap().clone();
    //     let bnode = <GenericGraph as SRDFBasic>::term_as_bnode(&term).unwrap();
    //     let subject = <GenericGraph as SRDFBasic>::bnode_id2subject(bnode.as_str());
    //     let outgoing = graph.outgoing_arcs(&subject).unwrap();
    //     let one = <GenericGraph as SRDFBasic>::object_as_term(&Object::Literal(int!(1)));
    //     assert_eq!(outgoing.get(&p), Some(&HashSet::from([one])))
    // }

    // #[test]
    // fn test_parser() {
    //     use crate::{ok, rdf_parser, RDFNodeParse};
    //     rdf_parser! {
    //         fn my_ok['a, A, RDF](value: &'a A)(RDF) -> A
    //         where [
    //             A: Clone
    //         ] { ok(&value.clone()) }
    //     }
    //     let s = r#"prefix : <http://example.org/>"#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::Strict).unwrap();
    //     let x = iri!("http://example.org/x");
    //     assert_eq!(my_ok(&3).parse(&x, graph).unwrap(), 3)
    // }

    // #[test]
    // fn test_parser_property_integers() {
    //     use crate::{property_integers, RDFNodeParse};
    //     let s = r#"prefix : <http://example.org/>
    //       :x :p 1, 2, 3, 2 .
    //     "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::Strict).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let p = iri!("http://example.org/p");
    //     let mut parser = property_integers(&p);
    //     assert_eq!(parser.parse(&x, graph).unwrap(), HashSet::from([1, 2, 3]))
    // }

    // #[test]
    // fn test_parser_then_mut() {
    //     use crate::{ok, property_integers, RDFNodeParse};
    //     let s = r#"prefix : <http://example.org/>
    //       :x :p 1, 2, 3 .
    //     "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::Strict).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let p = iri!("http://example.org/p");
    //     let mut parser = property_integers(&p).then_mut(move |ns| {
    //         ns.extend(vec![4, 5]);
    //         ok(ns)
    //     });
    //     assert_eq!(
    //         parser.parse(&x, graph).unwrap(),
    //         HashSet::from([1, 2, 3, 4, 5])
    //     )
    // }

    // #[test]
    // fn test_parser_or() {
    //     use crate::{property_bool, RDFNodeParse};
    //     let s = r#"prefix : <http://example.org/>
    //       :x :p 1, 2 ;
    //          :q true .
    //     "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::Strict).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let p = iri!("http://example.org/p");
    //     let q = iri!("http://example.org/q");
    //     let mut parser = property_bool(&p).or(property_bool(&q));
    //     assert!(parser.parse(&x, graph).unwrap())
    // }

    // #[test]
    // fn test_parser_or_enum_1() {
    //     #[derive(Debug, PartialEq)]
    //     enum A {
    //         Int(isize),
    //         Bool(bool),
    //     }
    //     use crate::{property_bool, property_integer, RDFNodeParse};
    //     let s = r#"prefix : <http://example.org/>
    //       :x :p 1 .
    //     "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::Strict).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let p = iri!("http://example.org/p");
    //     let parser_a_bool = property_bool(&p).map(A::Bool);
    //     let parser_a_int = property_integer(&p).map(A::Int);
    //     let mut parser = parser_a_int.or(parser_a_bool);
    //     assert_eq!(parser.parse(&x, graph).unwrap(), A::Int(1))
    // }

    // #[test]
    // fn test_parser_or_enum_2() {
    //     #[derive(Debug, PartialEq)]
    //     enum A {
    //         Int(isize),
    //         Bool(bool),
    //     }
    //     use crate::{property_bool, property_integer, RDFNodeParse};
    //     let s = r#"prefix : <http://example.org/>
    //       :x :p true .
    //     "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let p = iri!("http://example.org/p");
    //     let parser_a_bool = property_bool(&p).map(A::Bool);
    //     let parser_a_int = property_integer(&p).map(A::Int);
    //     let mut parser = parser_a_int.or(parser_a_bool);
    //     assert_eq!(parser.parse(&x, graph).unwrap(), A::Bool(true))
    // }

    // #[test]
    // fn test_parser_and() {
    //     use crate::{property_bool, property_integer, RDFNodeParse};
    //     let s = r#"prefix : <http://example.org/>
    //       :x :p true ;
    //          :q 1    .
    //     "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let p = iri!("http://example.org/p");
    //     let q = iri!("http://example.org/q");
    //     let mut parser = property_bool(&p).and(property_integer(&q));
    //     assert_eq!(parser.parse(&x, graph).unwrap(), (true, 1))
    // }

    // #[test]
    // fn test_parser_map() {
    //     use crate::{property_integer, RDFNodeParse};
    //     let s = r#"prefix : <http://example.org/>
    //       :x :p 1 .
    //     "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let p = iri!("http://example.org/p");
    //     let mut parser = property_integer(&p).map(|n| n + 1);
    //     assert_eq!(parser.parse(&x, graph).unwrap(), 2)
    // }

    // #[test]
    // fn test_parser_and_then() {
    //     use crate::{property_string, RDFNodeParse, RDFParseError};
    //     let s = r#"prefix : <http://example.org/>
    //       :x :p "1" .
    //     "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let p = iri!("http://example.org/p");
    //     struct IntConversionError(String);
    //     fn cnv_int(s: String) -> Result<isize, IntConversionError> {
    //         s.parse().map_err(|_| IntConversionError(s))
    //     }

    //     impl From<IntConversionError> for RDFParseError {
    //         fn from(error: IntConversionError) -> RDFParseError {
    //             RDFParseError::Custom {
    //                 msg: format!("Int conversion error: {}", error.0),
    //             }
    //         }
    //     }

    //     let mut parser = property_string(&p).and_then(cnv_int);
    //     assert_eq!(parser.parse(&x, graph).unwrap(), 1)
    // }

    // #[test]
    // fn test_parser_flat_map() {
    //     use crate::{property_string, PResult, RDFNodeParse, RDFParseError};
    //     let s = r#"prefix : <http://example.org/>
    //       :x :p "1" .
    //     "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let p = iri!("http://example.org/p");

    //     fn cnv_int(s: String) -> PResult<isize> {
    //         s.parse().map_err(|_| RDFParseError::Custom {
    //             msg: format!("Error converting {s}"),
    //         })
    //     }

    //     let mut parser = property_string(&p).flat_map(cnv_int);
    //     assert_eq!(parser.parse(&x, graph).unwrap(), 1)
    // }

    // #[test]
    // fn test_rdf_parser_macro() {
    //     use crate::GenericGraph;
    //     use crate::{rdf_parser, satisfy, RDFNodeParse, SRDFBasic};
    //     use iri_s::iri;

    //     rdf_parser! {
    //           fn is_term['a, RDF](term: &'a RDF::Term)(RDF) -> ()
    //           where [
    //           ] {
    //            let name = format!("is_{term}");
    //            satisfy(|t| { t == *term }, name.as_str())
    //           }
    //     }

    //     let s = r#"prefix : <http://example.org/>
    //                :x :p 1.
    //     "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let term = <GenericGraph as SRDFBasic>::iri_s2term(&x);
    //     let mut parser = is_term(&term);
    //     let result = parser.parse(&x, graph);
    //     assert!(result.is_ok())
    // }

    // #[test]
    // fn test_not() {
    //     use crate::GenericGraph;
    //     use crate::{not, property_value, RDFNodeParse};
    //     use iri_s::iri;

    //     let s = r#"prefix : <http://example.org/>
    //            :x :p 1 .
    // "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let q = iri!("http://example.org/q");
    //     assert!(not(property_value(&q)).parse(&x, graph).is_ok())
    // }

    // #[test]
    // fn test_iri() {
    //     use crate::GenericGraph;
    //     use crate::{iri, RDFNodeParse};
    //     use iri_s::iri;

    //     let graph = GenericGraph::new();
    //     let x = iri!("http://example.org/x");
    //     assert_eq!(iri().parse(&x, graph).unwrap(), x)
    // }

    // #[test]
    // fn test_add_triple() {
    //     use crate::GenericGraph;
    //     use iri_s::iri;

    //     let mut graph = GenericGraph::new();
    //     let alice = <GenericGraph as SRDFBasic>::iri_s2subject(&iri!("http://example.org/alice"));
    //     let knows = <GenericGraph as SRDFBasic>::iri_s2iri(&iri!("http://example.org/knows"));
    //     let bob = <GenericGraph as SRDFBasic>::iri_s2term(&iri!("http://example.org/bob"));

    //     graph.add_triple(&alice, &knows, &bob).unwrap();

    //     assert_eq!(graph.len(), 1);
    // }

    // #[test]
    // fn test_rdf_list() {
    //     use crate::GenericGraph;
    //     use crate::{property_value, rdf_list, set_focus, RDFNodeParse};
    //     use iri_s::iri;

    //     let s = r#"prefix : <http://example.org/>
    //            :x :p (1 2).
    // "#;
    //     let graph =
    //         GenericGraph::from_str(s, &RdfFormat::Turtle, None, &ReaderMode::default()).unwrap();
    //     let x = iri!("http://example.org/x");
    //     let p = iri!("http://example.org/p");
    //     let mut parser = property_value(&p).then(move |obj| set_focus(&obj).with(rdf_list()));
    //     let result: Vec<OxTerm> = parser.parse(&x, graph).unwrap();
    //     assert_eq!(
    //         result,
    //         vec![
    //             OxTerm::from(OxLiteral::from(1)),
    //             OxTerm::from(OxLiteral::from(2))
    //         ]
    //     )
    // }
}
