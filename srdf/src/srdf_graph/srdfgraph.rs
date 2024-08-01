use async_trait::async_trait;
use colored::*;
use iri_s::IriS;
// use log::debug;
use crate::async_srdf::AsyncSRDF;
use crate::literal::Literal;
use crate::numeric_literal::NumericLiteral;
use crate::{FocusRDF, RDFFormat, SRDFBasic, SRDFBuilder, Triple as STriple, RDF_TYPE_STR, SRDF};
use oxiri::Iri;
use oxrdfio::{RdfFormat, RdfSerializer};
use rust_decimal::Decimal;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::lang::Lang;
use crate::srdfgraph_error::SRDFGraphError;
use crate::Object;
use oxrdf::{
    BlankNode as OxBlankNode, Graph, Literal as OxLiteral, NamedNode as OxNamedNode,
    Subject as OxSubject, Term as OxTerm, Triple as OxTriple,
};
use oxsdatatypes::Decimal as OxDecimal;
use oxttl::TurtleParser;
use prefixmap::{prefixmap::*, IriRef, PrefixMapError};

#[derive(Debug, Default, Clone)]
pub struct SRDFGraph {
    focus: Option<OxTerm>,
    graph: Graph,
    pm: PrefixMap,
    base: Option<IriS>,
}

impl SRDFGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    pub fn from_reader<R: BufRead>(
        read: R,
        _format: &RDFFormat,
        base: Option<Iri<String>>,
    ) -> Result<SRDFGraph, SRDFGraphError> {
        // TODO: check format and use oxrdfio...
        let turtle_parser = match base {
            None => TurtleParser::new(),
            Some(ref iri) => TurtleParser::new().with_base_iri(iri.as_str())?,
        };
        let mut graph = Graph::default();
        let mut reader = turtle_parser.parse_read(read);
        for triple_result in reader.by_ref() {
            graph.insert(triple_result?.as_ref());
        }
        let prefixes: HashMap<&str, &str> = reader.prefixes().collect();
        let base = base.map(|iri| IriS::new_unchecked(iri.as_str()));
        let pm = PrefixMap::from_hashmap(&prefixes)?;
        Ok(SRDFGraph {
            focus: None,
            graph,
            pm,
            base,
        })
    }

    pub fn resolve(&self, str: &str) -> Result<OxNamedNode, SRDFGraphError> {
        let r = self.pm.resolve(str)?;
        Ok(Self::cnv_iri(r))
    }

    pub fn show_blanknode(&self, bn: &OxBlankNode) -> String {
        let str: String = format!("{}", bn);
        format!("{}", str.green())
    }

    pub fn show_literal(&self, lit: &OxLiteral) -> String {
        let str: String = format!("{}", lit);
        format!("{}", str.red())
    }

    pub fn from_str(
        data: &str,
        format: &RDFFormat,
        base: Option<Iri<String>>,
    ) -> Result<SRDFGraph, SRDFGraphError> {
        Self::from_reader(std::io::Cursor::new(&data), format, base)
    }

    fn cnv_iri(iri: IriS) -> OxNamedNode {
        OxNamedNode::new_unchecked(iri.as_str())
    }

    pub fn from_path(
        path: &Path,
        format: &RDFFormat,
        base: Option<Iri<String>>,
    ) -> Result<SRDFGraph, SRDFGraphError> {
        let file = File::open(path).map_err(|e| SRDFGraphError::ReadingPathError {
            path_name: path.display().to_string(),
            error: e,
        })?;
        let reader = BufReader::new(file);
        Self::from_reader(reader, format, base)
    }

    pub fn parse_data(
        data: &String,
        format: &RDFFormat,
        base: &Path,
    ) -> Result<SRDFGraph, SRDFGraphError> {
        let mut attempt = PathBuf::from(base);
        attempt.push(data);
        let base = Some(Iri::parse("base:://".to_owned()).unwrap());
        let data_path = &attempt;
        let graph = Self::from_path(data_path, format, base)?;
        Ok(graph)
    }

    pub fn prefixmap(&self) -> PrefixMap {
        self.pm.clone()
    }
}

impl SRDFBasic for SRDFGraph {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = SRDFGraphError;

    fn subject_as_iri(subject: &OxSubject) -> Option<OxNamedNode> {
        match subject {
            OxSubject::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }
    fn subject_as_bnode(subject: &OxSubject) -> Option<OxBlankNode> {
        match subject {
            OxSubject::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }
    fn subject_is_iri(subject: &OxSubject) -> bool {
        matches!(subject, OxSubject::NamedNode(_))
    }
    fn subject_is_bnode(subject: &OxSubject) -> bool {
        matches!(subject, OxSubject::BlankNode(_))
    }

    fn term_as_iri(object: &OxTerm) -> Option<OxNamedNode> {
        match object {
            OxTerm::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }
    fn term_as_bnode(object: &OxTerm) -> Option<OxBlankNode> {
        match object {
            OxTerm::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }

    fn term_as_literal(object: &OxTerm) -> Option<OxLiteral> {
        match object {
            OxTerm::Literal(l) => Some(l.clone()),
            _ => None,
        }
    }

    fn term_is_iri(object: &OxTerm) -> bool {
        matches!(object, OxTerm::NamedNode(_))
    }
    fn term_is_bnode(object: &OxTerm) -> bool {
        matches!(object, OxTerm::BlankNode(_))
    }

    fn term_is_literal(object: &OxTerm) -> bool {
        matches!(object, OxTerm::Literal(_))
    }

    fn subject_as_term(subject: &Self::Subject) -> Self::Term {
        match subject {
            OxSubject::NamedNode(n) => OxTerm::NamedNode(n.clone()),
            OxSubject::BlankNode(b) => OxTerm::BlankNode(b.clone()),
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => unimplemented!(),
        }
    }

    fn term_as_subject(object: &Self::Term) -> Option<Self::Subject> {
        match object {
            OxTerm::NamedNode(n) => Some(OxSubject::NamedNode(n.clone())),
            OxTerm::BlankNode(b) => Some(OxSubject::BlankNode(b.clone())),
            _ => None,
        }
    }

    fn lexical_form(literal: &OxLiteral) -> &str {
        literal.value()
    }

    fn lang(literal: &OxLiteral) -> Option<String> {
        literal.language().map(|s| s.to_string())
    }

    fn datatype(literal: &OxLiteral) -> OxNamedNode {
        literal.datatype().into_owned()
    }

    fn iri_s2iri(iri_s: &IriS) -> OxNamedNode {
        iri_s.as_named_node().clone()
    }

    fn iri_as_term(iri: OxNamedNode) -> OxTerm {
        OxTerm::NamedNode(iri)
    }

    fn iri_as_subject(iri: OxNamedNode) -> OxSubject {
        OxSubject::NamedNode(iri)
    }

    fn iri2iri_s(iri: &OxNamedNode) -> IriS {
        IriS::from_named_node(iri)
    }

    fn term_s2term(term: &OxTerm) -> Self::Term {
        term.clone()
    }

    fn term_as_object(term: &OxTerm) -> Object {
        match term {
            OxTerm::BlankNode(bn) => Object::BlankNode(bn.as_str().to_string()),
            OxTerm::Literal(lit) => {
                let lit = lit.to_owned();
                match lit.destruct() {
                    (s, None, None) => Object::Literal(Literal::StringLiteral {
                        lexical_form: s,
                        lang: None,
                    }),
                    (s, None, Some(lang)) => Object::Literal(Literal::StringLiteral {
                        lexical_form: s,
                        lang: Some(Lang::new(lang.as_str())),
                    }),
                    (s, Some(datatype), _) => {
                        let iri_s = Self::iri2iri_s(&datatype);
                        Object::Literal(Literal::DatatypeLiteral {
                            lexical_form: s,
                            datatype: IriRef::Iri(iri_s),
                        })
                    }
                }
            }
            OxTerm::NamedNode(iri) => Object::Iri(Self::iri2iri_s(iri)),
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        let iri = self.pm.resolve_prefix_local(prefix, local)?;
        Ok(iri.clone())
    }

    fn qualify_iri(&self, node: &OxNamedNode) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap();
        self.pm.qualify(&iri)
    }

    fn qualify_subject(&self, subj: &OxSubject) -> String {
        match subj {
            OxSubject::BlankNode(bn) => self.show_blanknode(bn),
            OxSubject::NamedNode(n) => self.qualify_iri(n),
            #[cfg(feature = "rdf-star")]
            OxSubject::Triple(_) => unimplemented!(),
        }
    }

    fn qualify_term(&self, term: &OxTerm) -> String {
        match term {
            OxTerm::BlankNode(bn) => self.show_blanknode(bn),
            OxTerm::Literal(lit) => self.show_literal(lit),
            OxTerm::NamedNode(n) => self.qualify_iri(n),
            #[cfg(feature = "rdf-star")]
            OxTerm::Triple(_) => unimplemented!(),
        }
    }

    fn prefixmap(&self) -> Option<prefixmap::PrefixMap> {
        Some(self.pm.clone())
    }

    fn bnode_id2bnode(id: &str) -> Self::BNode {
        OxBlankNode::new_unchecked(id)
    }

    fn bnode_as_term(bnode: Self::BNode) -> Self::Term {
        OxTerm::BlankNode(bnode)
    }

    fn object_as_term(obj: &Object) -> Self::Term {
        match obj {
            Object::Iri(iri) => Self::iri_s2term(iri),
            Object::BlankNode(bn) => Self::bnode_id2term(bn),
            Object::Literal(lit) => {
                let literal: OxLiteral = match lit {
                    Literal::StringLiteral { lexical_form, lang } => match lang {
                        Some(lang) => OxLiteral::new_language_tagged_literal_unchecked(
                            lexical_form,
                            lang.to_string(),
                        ),
                        None => OxLiteral::new_simple_literal(lexical_form),
                    },
                    Literal::DatatypeLiteral {
                        lexical_form,
                        datatype,
                    } => OxLiteral::new_typed_literal(lexical_form, cnv_iri_ref(datatype)),
                    Literal::NumericLiteral(n) => match n {
                        NumericLiteral::Integer(n) => {
                            let n: i128 = *n as i128;
                            OxLiteral::from(n)
                        }
                        NumericLiteral::Decimal(d) => {
                            let decimal = cnv_decimal(d);
                            OxLiteral::from(decimal)
                        }
                        NumericLiteral::Double(d) => OxLiteral::from(*d),
                    },
                    Literal::BooleanLiteral(b) => OxLiteral::from(*b),
                };
                OxTerm::Literal(literal)
            }
        }
    }

    fn bnode_as_subject(bnode: Self::BNode) -> Self::Subject {
        OxSubject::BlankNode(bnode)
    }
}

fn cnv_iri_ref(iri_ref: &IriRef) -> OxNamedNode {
    OxNamedNode::new_unchecked(iri_ref.to_string())
}

fn cnv_decimal(_d: &Decimal) -> OxDecimal {
    todo!()
}

impl SRDF for SRDFGraph {
    fn predicates_for_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashSet<Self::IRI>, Self::Err> {
        let mut ps = HashSet::new();
        for triple in self.graph.triples_for_subject(subject) {
            let pred = triple.predicate.into_owned();
            ps.insert(pred);
        }
        Ok(ps)
    }

    fn objects_for_subject_predicate(
        &self,
        subject: &Self::Subject,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, Self::Err> {
        let predicate = pred.as_ref();
        let mut result = HashSet::new();
        for o in self.graph.objects_for_subject_predicate(subject, predicate) {
            result.insert(o.into_owned());
        }
        Ok(result)
    }

    fn subjects_with_predicate_object(
        &self,
        pred: &Self::IRI,
        object: &Self::Term,
    ) -> Result<HashSet<Self::Subject>, Self::Err> {
        let mut result = HashSet::new();
        for subj in self
            .graph
            .subjects_for_predicate_object(pred.as_ref(), object.as_ref())
        {
            result.insert(subj.into_owned());
        }
        Ok(result)
    }

    fn outgoing_arcs(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Term>>, Self::Err> {
        let mut results: HashMap<Self::IRI, HashSet<Self::Term>> = HashMap::new();
        for triple in self.graph.triples_for_subject(subject) {
            let pred = triple.predicate.into_owned();
            let term = triple.object.into_owned();
            match results.entry(pred) {
                Entry::Occupied(mut vs) => {
                    vs.get_mut().insert(term.clone());
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(HashSet::from([term.clone()]));
                }
            }
        }
        Ok(results)
    }

    fn incoming_arcs(
        &self,
        object: &Self::Term,
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Subject>>, Self::Err> {
        let mut results: HashMap<Self::IRI, HashSet<Self::Subject>> = HashMap::new();
        for triple in self.graph.triples_for_object(object) {
            let pred = triple.predicate.into_owned();
            let subj = triple.subject.into_owned();
            match results.entry(pred) {
                Entry::Occupied(mut vs) => {
                    vs.get_mut().insert(subj.clone());
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(HashSet::from([subj.clone()]));
                }
            }
        }
        Ok(results)
    }

    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: Vec<Self::IRI>,
    ) -> Result<(HashMap<Self::IRI, HashSet<Self::Term>>, Vec<Self::IRI>), Self::Err> {
        let mut results: HashMap<Self::IRI, HashSet<Self::Term>> = HashMap::new();
        let mut remainder = Vec::new();
        for triple in self.graph.triples_for_subject(subject) {
            let pred = triple.predicate.into_owned();
            let term = triple.object.into_owned();
            if preds.contains(&pred) {
                match results.entry(pred) {
                    Entry::Occupied(mut vs) => {
                        vs.get_mut().insert(term.clone());
                    }
                    Entry::Vacant(vacant) => {
                        vacant.insert(HashSet::from([term.clone()]));
                    }
                }
            } else {
                remainder.push(pred)
            }
        }
        Ok((results, remainder))
    }

    fn triples_with_predicate(
        &self,
        pred: &Self::IRI,
    ) -> Result<Vec<crate::Triple<Self>>, Self::Err> {
        let mut result = Vec::new();
        for triple in self.graph.triples_for_predicate(pred) {
            let subj = triple.subject.into_owned();
            let pred = triple.predicate.into_owned();
            let obj = triple.object.into_owned();
            result.push(STriple::new(subj, pred, obj))
        }
        Ok(result)
    }
}

#[async_trait]
impl AsyncSRDF for SRDFGraph {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;
    type Err = SRDFGraphError;

    async fn get_predicates_subject(
        &self,
        subject: &OxSubject,
    ) -> Result<HashSet<OxNamedNode>, SRDFGraphError> {
        let mut results = HashSet::new();
        for triple in self.graph.triples_for_subject(subject) {
            let predicate: OxNamedNode = triple.predicate.to_owned().into();
            results.insert(predicate);
        }
        Ok(results)
    }

    async fn get_objects_for_subject_predicate(
        &self,
        subject: &OxSubject,
        pred: &OxNamedNode,
    ) -> Result<HashSet<OxTerm>, SRDFGraphError> {
        let mut results = HashSet::new();
        for triple in self.graph.triples_for_subject(subject) {
            let predicate: OxNamedNode = triple.predicate.to_owned().into();
            if predicate.eq(pred) {
                let object: OxTerm = triple.object.to_owned().into();
                results.insert(object);
            }
        }
        Ok(results)
    }

    async fn get_subjects_for_object_predicate(
        &self,
        object: &OxTerm,
        pred: &OxNamedNode,
    ) -> Result<HashSet<OxSubject>, SRDFGraphError> {
        let mut results = HashSet::new();
        for triple in self.graph.triples_for_object(object) {
            let predicate: OxNamedNode = triple.predicate.to_owned().into();
            if predicate.eq(pred) {
                let subject: OxSubject = triple.subject.to_owned().into();
                results.insert(subject);
            }
        }
        Ok(results)
    }
}

impl FocusRDF for SRDFGraph {
    fn set_focus(&mut self, focus: &Self::Term) {
        self.focus = Some(focus.clone())
    }

    fn get_focus(&self) -> &Option<Self::Term> {
        &self.focus
    }
}

impl SRDFBuilder for SRDFGraph {
    fn add_base(&mut self, base: &Option<IriS>) -> Result<(), Self::Err> {
        self.base.clone_from(base);
        Ok(())
    }

    fn add_prefix(&mut self, alias: &str, iri: &IriS) -> Result<(), Self::Err> {
        self.pm.insert(alias, iri);
        Ok(())
    }

    fn add_prefix_map(&mut self, prefix_map: PrefixMap) -> Result<(), Self::Err> {
        self.pm = prefix_map.clone();
        Ok(())
    }

    fn add_triple(
        &mut self,
        subj: &Self::Subject,
        pred: &Self::IRI,
        obj: &Self::Term,
    ) -> Result<(), Self::Err> {
        let triple = OxTriple::new(subj.clone(), pred.clone(), obj.clone());
        self.graph.insert(&triple);
        Ok(())
    }

    fn remove_triple(
        &mut self,
        subj: &Self::Subject,
        pred: &Self::IRI,
        obj: &Self::Term,
    ) -> Result<(), Self::Err> {
        let triple = OxTriple::new(subj.clone(), pred.clone(), obj.clone());
        self.graph.remove(&triple);
        Ok(())
    }

    fn add_type(&mut self, node: &crate::RDFNode, r#type: Self::Term) -> Result<(), Self::Err> {
        match Self::object_as_subject(node) {
            Some(subj) => {
                let triple = OxTriple::new(subj, rdf_type(), r#type.clone());
                self.graph.insert(&triple);
                Ok(())
            }
            None => {
                panic!("Error adding type to {node} because it can't be converted to a subject")
            }
        }
    }

    fn empty() -> Self {
        SRDFGraph {
            focus: None,
            graph: Graph::new(),
            pm: PrefixMap::new(),
            base: None,
        }
    }

    fn serialize<W: Write>(&self, format: RDFFormat, write: W) -> Result<(), Self::Err> {
        let mut serializer = RdfSerializer::from_format(cnv_rdf_format(format));

        for (prefix, iri) in &self.pm.map {
            serializer = serializer.with_prefix(prefix, iri.as_str()).unwrap();
        }

        let mut writer = serializer.serialize_to_write(write);
        for triple in self.graph.iter() {
            writer.write_triple(triple)?;
        }
        writer.finish()?;
        Ok(())
    }
}

fn cnv_rdf_format(rdf_format: RDFFormat) -> RdfFormat {
    match rdf_format {
        RDFFormat::NTriples => RdfFormat::NTriples,
        RDFFormat::Turtle => RdfFormat::Turtle,
        RDFFormat::RDFXML => RdfFormat::RdfXml,
        RDFFormat::TriG => RdfFormat::TriG,
        RDFFormat::N3 => RdfFormat::N3,
        RDFFormat::NQuads => RdfFormat::NQuads,
    }
}

fn rdf_type() -> OxNamedNode {
    OxNamedNode::new_unchecked(RDF_TYPE_STR)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{int, srdf, SRDFGraph, SRDF};
    use iri_s::iri;

    #[tokio::test]
    async fn parse_get_predicates() {
        use crate::srdfgraph::AsyncSRDF;

        let s = r#"PREFIX : <http://example.org/>
            PREFIX schema: <https://schema.org/>

            :alice schema:name "Alice" ;
                   schema:knows :bob, :carol ;
                   :age  23 .
         "#;
        let parsed_graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let alice = OxSubject::NamedNode(OxNamedNode::new_unchecked("http://example.org/alice"));
        let knows = OxNamedNode::new_unchecked("https://schema.org/knows");
        let bag_preds = parsed_graph.get_predicates_subject(&alice).await.unwrap();
        assert!(bag_preds.contains(&knows));
        let bob = OxTerm::NamedNode(OxNamedNode::new_unchecked("http://example.org/bob"));
        let alice_knows =
            AsyncSRDF::get_objects_for_subject_predicate(&parsed_graph, &alice, &knows)
                .await
                .unwrap();
        assert!(alice_knows.contains(&bob));
    }

    #[test]
    fn test_outgoing_arcs() {
        let s = r#"prefix : <http://example.org/>
        prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

        :x :p [ :p 1 ].
        "#;

        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = <SRDFGraph as SRDFBasic>::iri_s2subject(&iri!("http://example.org/x"));
        let p = <SRDFGraph as SRDFBasic>::iri_s2iri(&iri!("http://example.org/p"));
        let terms = srdf::SRDF::objects_for_subject_predicate(&graph, &x, &p).unwrap();
        let term = terms.iter().next().unwrap().clone();
        let subject = <SRDFGraph as SRDFBasic>::term_as_subject(&term).unwrap();
        let outgoing = graph.outgoing_arcs(&subject).unwrap();
        let one = <SRDFGraph as SRDFBasic>::object_as_term(&Object::Literal(int!(1)));
        assert_eq!(outgoing.get(&p), Some(&HashSet::from([one])))
    }

    #[test]
    fn test_outgoing_arcs_bnode() {
        let s = r#"prefix : <http://example.org/>
        prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

        :x :p [ :p 1 ].
        "#;

        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = <SRDFGraph as SRDFBasic>::iri_s2subject(&iri!("http://example.org/x"));
        let p = <SRDFGraph as SRDFBasic>::iri_s2iri(&iri!("http://example.org/p"));
        let terms = srdf::SRDF::objects_for_subject_predicate(&graph, &x, &p).unwrap();
        let term = terms.iter().next().unwrap().clone();
        let bnode = <SRDFGraph as SRDFBasic>::term_as_bnode(&term).unwrap();
        let subject = <SRDFGraph as SRDFBasic>::bnode_id2subject(bnode.as_str());
        let outgoing = graph.outgoing_arcs(&subject).unwrap();
        let one = <SRDFGraph as SRDFBasic>::object_as_term(&Object::Literal(int!(1)));
        assert_eq!(outgoing.get(&p), Some(&HashSet::from([one])))
    }

    #[test]
    fn test_parser() {
        use crate::{ok, rdf_parser, RDFNodeParse};
        rdf_parser! {
            fn my_ok['a, A, RDF](value: &'a A)(RDF) -> A
            where [
                A: Clone
            ] { ok(&value.clone()) }
        }
        let s = r#"prefix : <http://example.org/>"#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        assert_eq!(my_ok(&3).parse(&x, graph).unwrap(), 3)
    }

    #[test]
    fn test_parser_property_integers() {
        use crate::{property_integers, RDFNodeParse};
        let s = r#"prefix : <http://example.org/>
          :x :p 1, 2, 3, 2 .
        "#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let mut parser = property_integers(&p);
        assert_eq!(parser.parse(&x, graph).unwrap(), HashSet::from([1, 2, 3]))
    }

    #[test]
    fn test_parser_then_mut() {
        use crate::{ok, property_integers, RDFNodeParse};
        let s = r#"prefix : <http://example.org/>
          :x :p 1, 2, 3 .
        "#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let mut parser = property_integers(&p).then_mut(move |ns| {
            ns.extend(vec![4, 5]);
            ok(ns)
        });
        assert_eq!(
            parser.parse(&x, graph).unwrap(),
            HashSet::from([1, 2, 3, 4, 5])
        )
    }

    #[test]
    fn test_parser_or() {
        use crate::{property_bool, RDFNodeParse};
        let s = r#"prefix : <http://example.org/>
          :x :p 1, 2 ;
             :q true .
        "#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let q = iri!("http://example.org/q");
        let mut parser = property_bool(&p).or(property_bool(&q));
        assert!(parser.parse(&x, graph).unwrap())
    }

    #[test]
    fn test_parser_or_enum_1() {
        #[derive(Debug, PartialEq)]
        enum A {
            Int(isize),
            Bool(bool),
        }
        use crate::{property_bool, property_integer, RDFNodeParse};
        let s = r#"prefix : <http://example.org/>
          :x :p 1 .
        "#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let parser_a_bool = property_bool(&p).map(A::Bool);
        let parser_a_int = property_integer(&p).map(A::Int);
        let mut parser = parser_a_int.or(parser_a_bool);
        assert_eq!(parser.parse(&x, graph).unwrap(), A::Int(1))
    }

    #[test]
    fn test_parser_or_enum_2() {
        #[derive(Debug, PartialEq)]
        enum A {
            Int(isize),
            Bool(bool),
        }
        use crate::{property_bool, property_integer, RDFNodeParse};
        let s = r#"prefix : <http://example.org/>
          :x :p true .
        "#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let parser_a_bool = property_bool(&p).map(A::Bool);
        let parser_a_int = property_integer(&p).map(A::Int);
        let mut parser = parser_a_int.or(parser_a_bool);
        assert_eq!(parser.parse(&x, graph).unwrap(), A::Bool(true))
    }

    #[test]
    fn test_parser_and() {
        use crate::{property_bool, property_integer, RDFNodeParse};
        let s = r#"prefix : <http://example.org/>
          :x :p true ;
             :q 1    .
        "#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let q = iri!("http://example.org/q");
        let mut parser = property_bool(&p).and(property_integer(&q));
        assert_eq!(parser.parse(&x, graph).unwrap(), (true, 1))
    }

    #[test]
    fn test_parser_map() {
        use crate::{property_integer, RDFNodeParse};
        let s = r#"prefix : <http://example.org/>
          :x :p 1 .
        "#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        let mut parser = property_integer(&p).map(|n| n + 1);
        assert_eq!(parser.parse(&x, graph).unwrap(), 2)
    }

    #[test]
    fn test_parser_and_then() {
        use crate::{property_string, RDFNodeParse, RDFParseError};
        let s = r#"prefix : <http://example.org/>
          :x :p "1" .
        "#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");
        struct IntConversionError(String);
        fn cnv_int(s: String) -> Result<isize, IntConversionError> {
            s.parse().map_err(|_| IntConversionError(s))
        }

        impl From<IntConversionError> for RDFParseError {
            fn from(error: IntConversionError) -> RDFParseError {
                RDFParseError::Custom {
                    msg: format!("Int conversion error: {}", error.0),
                }
            }
        }

        let mut parser = property_string(&p).and_then(cnv_int);
        assert_eq!(parser.parse(&x, graph).unwrap(), 1)
    }

    #[test]
    fn test_parser_flat_map() {
        use crate::{property_string, PResult, RDFNodeParse, RDFParseError};
        let s = r#"prefix : <http://example.org/>
          :x :p "1" .
        "#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        let p = iri!("http://example.org/p");

        fn cnv_int(s: String) -> PResult<isize> {
            s.parse().map_err(|_| RDFParseError::Custom {
                msg: format!("Error converting {s}"),
            })
        }

        let mut parser = property_string(&p).flat_map(cnv_int);
        assert_eq!(parser.parse(&x, graph).unwrap(), 1)
    }

    #[test]
    fn test_rdf_parser_macro() {
        use crate::SRDFGraph;
        use crate::{rdf_parser, satisfy, RDFNodeParse, SRDFBasic};
        use iri_s::iri;

        rdf_parser! {
              fn is_term['a, RDF](term: &'a RDF::Term)(RDF) -> ()
              where [
              ] {
               let name = format!("is_{term}");
               satisfy(|t| { t == *term }, name.as_str())
              }
        }

        let s = r#"prefix : <http://example.org/>
                   :x :p 1.
        "#;
        let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
        let x = iri!("http://example.org/x");
        let term = <SRDFGraph as SRDFBasic>::iri_s2term(&x);
        let mut parser = is_term(&term);
        let result = parser.parse(&x, graph);
        assert!(result.is_ok())
    }
}

#[test]
fn test_rdf_list() {
    use crate::SRDFGraph;
    use crate::{property_value, rdf_list, set_focus, RDFNodeParse};
    use iri_s::iri;

    let s = r#"prefix : <http://example.org/>
               :x :p (1 2).
    "#;
    let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
    let x = iri!("http://example.org/x");
    let p = iri!("http://example.org/p");
    let mut parser = property_value(&p).then(move |obj| set_focus(&obj).with(rdf_list()));
    let result: Vec<OxTerm> = parser.parse(&x, graph).unwrap();
    assert_eq!(
        result,
        vec![
            OxTerm::from(OxLiteral::from(1)),
            OxTerm::from(OxLiteral::from(2))
        ]
    )
}

#[test]
fn test_not() {
    use crate::SRDFGraph;
    use crate::{not, property_value, RDFNodeParse};
    use iri_s::iri;

    let s = r#"prefix : <http://example.org/>
               :x :p 1 .
    "#;
    let graph = SRDFGraph::from_str(s, &RDFFormat::Turtle, None).unwrap();
    let x = iri!("http://example.org/x");
    let q = iri!("http://example.org/q");
    assert!(not(property_value(&q)).parse(&x, graph).is_ok())
}

#[test]
fn test_iri() {
    use crate::SRDFGraph;
    use crate::{iri, RDFNodeParse};
    use iri_s::iri;

    let graph = SRDFGraph::new();
    let x = iri!("http://example.org/x");
    assert_eq!(iri().parse(&x, graph).unwrap(), x)
}
