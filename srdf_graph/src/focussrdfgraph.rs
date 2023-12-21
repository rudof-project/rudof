use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use iri_s::IriS;
use oxrdf::{BlankNode, Literal, NamedNode, Subject, Term};
use prefixmap::{IriRef, PrefixMapError};
use srdf::srdf_parser::FocusRDF;
use srdf::{SRDFComparisons, SRDF};

use crate::{SRDFGraph, SRDFGraphError};

pub struct FocusSRDFGraph {
    focus: Term,
    graph: SRDFGraph,
}

impl FocusSRDFGraph {
    pub fn new(focus: Term, graph: SRDFGraph) -> FocusSRDFGraph {
        FocusSRDFGraph { focus, graph }
    }
}

impl SRDF for FocusSRDFGraph {
    fn get_predicates_for_subject(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashSet<Self::IRI>, Self::Err> {
        todo!()
    }

    fn get_objects_for_subject_predicate(
        &self,
        subject: &Self::Subject,
        pred: &Self::IRI,
    ) -> Result<HashSet<Self::Term>, Self::Err> {
        todo!()
    }

    fn subjects_with_predicate_object(
        &self,
        pred: &Self::IRI,
        object: &Self::Term,
    ) -> Result<HashSet<Self::Subject>, Self::Err> {
        todo!()
    }

    fn outgoing_arcs(
        &self,
        subject: &Self::Subject,
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Term>>, Self::Err> {
        todo!()
    }

    fn incoming_arcs(
        &self,
        object: &Self::Term,
    ) -> Result<HashMap<Self::IRI, HashSet<Self::Subject>>, Self::Err> {
        todo!()
    }

    fn outgoing_arcs_from_list(
        &self,
        subject: &Self::Subject,
        preds: Vec<Self::IRI>,
    ) -> Result<(HashMap<Self::IRI, HashSet<Self::Term>>, Vec<Self::IRI>), Self::Err> {
        todo!()
    }
}

impl SRDFComparisons for FocusSRDFGraph {
    type IRI = NamedNode;
    type BNode = BlankNode;
    type Literal = Literal;
    type Subject = Subject;
    type Term = Term;
    type Err = SRDFGraphError;

    fn subject_as_iri(subject: &Subject) -> Option<NamedNode> {
        match subject {
            Subject::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }
    fn subject_as_bnode(subject: &Subject) -> Option<BlankNode> {
        match subject {
            Subject::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }
    fn subject_is_iri(subject: &Subject) -> bool {
        match subject {
            Subject::NamedNode(_) => true,
            _ => false,
        }
    }
    fn subject_is_bnode(subject: &Subject) -> bool {
        match subject {
            Subject::BlankNode(_) => true,
            _ => false,
        }
    }

    fn object_as_iri(object: &Term) -> Option<NamedNode> {
        match object {
            Term::NamedNode(n) => Some(n.clone()),
            _ => None,
        }
    }
    fn object_as_bnode(object: &Term) -> Option<BlankNode> {
        match object {
            Term::BlankNode(b) => Some(b.clone()),
            _ => None,
        }
    }

    fn object_as_literal(object: &Term) -> Option<Literal> {
        match object {
            Term::Literal(l) => Some(l.clone()),
            _ => None,
        }
    }

    fn object_is_iri(object: &Term) -> bool {
        match object {
            Term::NamedNode(_) => true,
            _ => false,
        }
    }
    fn object_is_bnode(object: &Term) -> bool {
        match object {
            Term::BlankNode(_) => true,
            _ => false,
        }
    }

    fn object_is_literal(object: &Term) -> bool {
        match object {
            Term::Literal(_) => true,
            _ => false,
        }
    }

    fn subject_as_term(subject: &Self::Subject) -> Self::Term {
        match subject {
            Subject::NamedNode(n) => Term::NamedNode(n.clone()),
            Subject::BlankNode(b) => Term::BlankNode(b.clone()),
        }
    }

    fn term_as_subject(object: &Self::Term) -> Option<Self::Subject> {
        match object {
            Term::NamedNode(n) => Some(Subject::NamedNode(n.clone())),
            Term::BlankNode(b) => Some(Subject::BlankNode(b.clone())),
            _ => None,
        }
    }

    fn lexical_form(&self, literal: &Literal) -> String {
        literal.to_string()
    }

    fn lang(&self, literal: &Literal) -> Option<String> {
        literal.language().map(|s| s.to_string())
    }

    fn datatype(&self, literal: &Literal) -> NamedNode {
        literal.datatype().into_owned()
    }

    fn iri_s2iri(iri_s: &IriS) -> NamedNode {
        iri_s.as_named_node().clone()
    }

    fn iri_as_term(iri: NamedNode) -> Term {
        Term::NamedNode(iri)
    }

    fn iri_as_subject(iri: NamedNode) -> Subject {
        Subject::NamedNode(iri)
    }

    fn iri2iri_s(iri: &NamedNode) -> IriS {
        IriS::from_named_node(iri)
    }

    fn term_as_object(term: &Term) -> srdf::Object {
        match term {
            Term::BlankNode(bn) => srdf::Object::BlankNode(bn.to_string()),
            Term::Literal(lit) => {
                let lit = lit.to_owned();
                match lit.destruct() {
                    (s, None, None) => {
                        srdf::Object::Literal(srdf::literal::Literal::StringLiteral {
                            lexical_form: s,
                            lang: None,
                        })
                    }
                    (s, None, Some(lang)) => {
                        srdf::Object::Literal(srdf::literal::Literal::StringLiteral {
                            lexical_form: s,
                            lang: Some(srdf::lang::Lang::new(lang.as_str())),
                        })
                    }
                    (s, Some(datatype), _) => {
                        let iri_s = Self::iri2iri_s(&datatype);
                        srdf::Object::Literal(srdf::literal::Literal::DatatypeLiteral {
                            lexical_form: s,
                            datatype: IriRef::Iri(iri_s),
                        })
                    }
                }
            }
            Term::NamedNode(iri) => srdf::Object::Iri {
                iri: Self::iri2iri_s(iri),
            },
        }
    }

    fn resolve_prefix_local(&self, prefix: &str, local: &str) -> Result<IriS, PrefixMapError> {
        let iri = self.graph.prefixmap().resolve_prefix_local(prefix, local)?;
        Ok(iri.clone())
    }

    fn qualify_iri(&self, node: &NamedNode) -> String {
        let iri = IriS::from_str(node.as_str()).unwrap();
        self.graph.prefixmap().qualify(&iri)
    }

    fn qualify_subject(&self, subj: &Subject) -> String {
        match subj {
            Subject::BlankNode(bn) => self.graph.show_blanknode(bn),
            Subject::NamedNode(n) => self.qualify_iri(n),
        }
    }

    fn qualify_term(&self, term: &Term) -> String {
        match term {
            Term::BlankNode(bn) => self.graph.show_blanknode(bn),
            Term::Literal(lit) => self.graph.show_literal(&lit),
            Term::NamedNode(n) => self.qualify_iri(n),
        }
    }
}

impl FocusRDF for FocusSRDFGraph {
    fn set_focus(&mut self, focus: Self::Term) {
        self.focus = focus;
    }

    fn get_focus(&self) -> &Self::Term {
        &self.focus
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use iri_s::IriS;
    use srdf::srdf_parser::RDFParser;
    use srdf::SRDFComparisons;

    use super::*;

    #[test]
    fn test_rdfparser1() {
        let str = r#"prefix : <http://example.org/>

        :x :p :y .
        "#;
        let rdf = SRDFGraph::from_str(str, None).unwrap();
        let x = <SRDFGraph as SRDFComparisons>::iri_s2term(&IriS::new_unchecked(
            "http://example.org/x",
        ));

        let focus_rdf_graph = FocusSRDFGraph::new(x, rdf);
        let parser: RDFParser<FocusSRDFGraph> = RDFParser::new(focus_rdf_graph);
        let p =
            <SRDFGraph as SRDFComparisons>::iri_s2iri(&IriS::new_unchecked("http://example.org/p"));
        let y = <SRDFGraph as SRDFComparisons>::iri_s2term(&IriS::new_unchecked(
            "http://example.org/y",
        ));
        // let values: HashSet<_> = parser.predicate_values(&p).unwrap();
        // let expected: HashSet<_> = [y].into_iter().collect();
        // assert_eq!(values, expected)
    }
}
