use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::Read;

use iri_s::IriS;
use oxrdf::Triple as OxTriple;
use oxrdfio::RdfParser as OxRdfParser;
use prefixmap::PrefixMap;
use tracing::debug;

use crate::model::focus_rdf::FocusRdf;
use crate::model::mutable_rdf::MutableRdf;
use crate::model::parse::RdfParse;
use crate::model::parse::ReaderMode;
use crate::model::rdf::Object;
use crate::model::rdf::Predicate;
use crate::model::rdf::Rdf;
use crate::model::rdf::Subject;
use crate::model::rdf_format::RdfFormat;
use crate::model::GraphName;
use crate::model::Iri;
use crate::model::Quad;
use crate::model::Triple;

use super::error::*;

pub type OxGraph = GenericGraph<OxTriple>;

#[derive(Debug, Clone)]
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

    pub fn quads<Q: Quad<Triple = T>>(&self) -> impl Iterator<Item = Q> + '_
    where
        T: Clone,
    {
        self.graph
            .iter()
            .map(move |t| Q::new(t.clone(), GraphName::Default))
    }

    pub fn merge_prefixes(&mut self, prefixmap: PrefixMap) -> Result<(), GraphError> {
        self.pm.merge(prefixmap)?;
        Ok(())
    }

    pub fn resolve(&self, str: &str) -> Result<IriS, GraphError> {
        let r = self.pm.resolve(str)?;
        Ok(r)
    }

    pub fn prefixmap(&self) -> &PrefixMap {
        &self.pm
    }
}

impl RdfParse for GenericGraph<OxTriple> {
    type ParseError = GraphParseError;

    fn merge_from_reader<R: Read>(
        &mut self,
        read: R,
        format: RdfFormat,
        base: Option<&str>,
        reader_mode: &ReaderMode,
    ) -> Result<(), Self::ParseError> {
        let reader = OxRdfParser::from_format(format.into()).for_reader(read);

        if let RdfFormat::Turtle = format {
            self.base = match (&self.base, base) {
                (None, None) => None,
                (Some(b), None) => Some(b.clone()),
                (_, Some(b)) => Some(Iri::new(b)),
            };
            let prefixes: HashMap<&str, &str> = reader.prefixes().collect();
            let pm = PrefixMap::from_hashmap(&prefixes)?;
            self.merge_prefixes(pm)?;
        }

        for triple in reader {
            let triple = match triple {
                Ok(triple) => triple,
                Err(error) => match reader_mode {
                    ReaderMode::Strict => return Err(error.into()),
                    ReaderMode::Lax => {
                        debug!("{}", format!("{error}, however we continue parsing."));
                        continue;
                    }
                },
            };
            let subject = triple.subject;
            let predicate = triple.predicate;
            let object = triple.object;
            self.add_triple(subject, predicate, object)?;
        }

        Ok(())
    }
}

impl<T: Triple> Rdf for GenericGraph<T> {
    type Triple = T;
    type Error = GraphError;

    fn triples_matching<'a>(
        &self,
        subject: Option<&'a Subject<Self>>,
        predicate: Option<&'a Predicate<Self>>,
        object: Option<&'a Object<Self>>,
    ) -> Result<impl Iterator<Item = &Self::Triple>, Self::Error> {
        let triples = self
            .graph
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

    fn prefixmap(&self) -> Option<PrefixMap> {
        Some(self.pm.clone())
    }
}

impl<T: Triple + Eq + Hash> MutableRdf for GenericGraph<T> {
    type MutableRdfError = MutableGraphError;

    fn add_triple(
        &mut self,
        subject: Subject<Self>,
        predicate: Predicate<Self>,
        object: Object<Self>,
    ) -> Result<(), Self::MutableRdfError> {
        self.graph.insert(T::new(subject, predicate, object));
        Ok(())
    }

    fn remove_triple(&mut self, triple: &T) -> Result<(), Self::MutableRdfError> {
        self.graph.remove(triple);
        Ok(())
    }

    fn add_base(&mut self, base: Predicate<Self>) -> Result<(), Self::MutableRdfError> {
        self.base = Some(base);
        Ok(())
    }

    fn add_prefix(
        &mut self,
        alias: &str,
        iri: Predicate<Self>,
    ) -> Result<(), Self::MutableRdfError> {
        self.pm.insert(alias, &iri.as_iri_s())?;
        Ok(())
    }
}

impl<T: Triple> FocusRdf for GenericGraph<T> {
    fn set_focus(&mut self, focus: Object<Self>) {
        self.focus = Some(focus);
    }

    fn get_focus(&self) -> Option<&Object<Self>> {
        match &self.focus {
            Some(focus) => Some(focus),
            None => None,
        }
    }
}

impl<T: Triple> Default for GenericGraph<T> {
    fn default() -> Self {
        Self {
            focus: Default::default(),
            graph: Default::default(),
            pm: Default::default(),
            base: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use oxrdf::NamedNode as OxNamedNode;
    use oxrdf::Subject as OxSubject;
    use oxrdf::Term as OxTerm;

    use crate::graph::graph::ReaderMode;
    use crate::model::mutable_rdf::MutableRdf;
    use crate::model::parse::RdfParse;
    use crate::model::rdf::Rdf;
    use crate::model::rdf_format::RdfFormat;
    use crate::model::Triple;

    use super::OxGraph;

    const DUMMY_GRAPH_1: &str = r#"
        prefix : <http://example.org/>
        prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        :x :p [ :p 1 ].
    "#;

    // const DUMMY_GRAPH_2: &str = r#"
    //     prefix : <http://example.org/>
    //     :x :p (1 2).
    // "#;

    fn graph_from_str(s: &str) -> OxGraph {
        OxGraph::from_str(s, RdfFormat::Turtle, None, &ReaderMode::Strict).unwrap()
    }

    #[test]
    fn test_outgoing_arcs() {
        let graph = graph_from_str(DUMMY_GRAPH_1);

        let x = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/x"));
        let p = OxNamedNode::new_unchecked("http://example.org/p");

        let subject = graph
            .triples_matching(Some(&x), Some(&p), None)
            .unwrap()
            .map(Triple::obj)
            .next()
            .unwrap()
            .to_owned()
            .try_into()
            .unwrap();

        let actual = graph.outgoing_arcs(&subject).unwrap();
        let expected = HashSet::from([OxTerm::Literal(1.into())]);
        assert_eq!(actual.get(&p), Some(&expected))
    }

    #[test]
    fn test_add_triple() {
        let mut graph = OxGraph::default();

        let alice = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/alice"));
        let knows = OxNamedNode::new_unchecked("http://example.org/knows");
        let bob = OxTerm::NamedNode(OxNamedNode::new_unchecked("http://example.org/bob"));

        graph.add_triple(alice, knows, bob).unwrap();

        assert_eq!(graph.len(), 1);
    }

    // #[test]
    // fn test_rdf_list() {
    //     let graph = graph_from_str(DUMMY_GRAPH_2);

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

    // #[test]
    // fn test_parser() {
    //     rdf_parser! {
    //         fn my_ok['a, A, RDF](value: &'a A)(RDF) -> A
    //         where [
    //             A: Clone
    //         ] { Ok(&value.clone()) }
    //     }

    //     let graph = graph_from_str("prefix : <http://example.org/>");
    //     let x = iri!("http://example.org/x");

    //     assert_eq!(my_ok(&3).parse(&x, graph).unwrap(), 3)
    // }

    // #[test]
    // fn test_parser_property_integers() {
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
}
