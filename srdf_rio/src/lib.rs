use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, Subject as OxSubject,
    Term as OxTerm, Triple as OxTriple,
};
use rio_api::model::{BlankNode, NamedNode, Subject, Term, Triple};
use rio_api::parser::*;
use rio_turtle::*;
use srdf::bnode::BNode;
use srdf::iri::IRI;

pub struct IRIRio<'a> {
    iri: NamedNode<'a>,
}

pub struct BNodeRio<'a> {
    bnode: BlankNode<'a>,
}
impl<'a> BNode<'a> for BNodeRio<'a> {
    fn label(&self) -> &'a str {
        self.bnode.id
    }
}

#[cfg(test)]
mod tests {
    use oxrdf::{Graph, SubjectRef};
    use rio_api::model::{Literal, Subject};

    use super::*;

    #[test]
    fn check_iri() {
        let rdf_type = IRIRio {
            iri: NamedNode {
                iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
            },
        };
        assert_eq!(
            rdf_type.iri.iri,
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type"
        );
    }

    #[test]
    fn parse_turtle() {
        fn cnv_subject(s: Subject) -> OxSubject {
            match s {
                Subject::NamedNode(n) => {
                    OxSubject::NamedNode(OxNamedNode::new_unchecked(n.iri.to_string()))
                }
                Subject::BlankNode(b) => OxSubject::BlankNode(OxBlankNode::new_unchecked(b.id)),
                Subject::Triple(_) => todo!(),
            }
        }

        fn cnv_named_node(s: NamedNode) -> OxNamedNode {
            OxNamedNode::new_unchecked(s.iri)
        }
        fn cnv_literal(l: Literal) -> OxLiteral {
            match l {
                Literal::Simple { value } => OxLiteral::new_simple_literal(value.to_string()),
                Literal::LanguageTaggedString { value, language } => {
                    OxLiteral::new_language_tagged_literal_unchecked(value, language)
                }
                Literal::Typed { value, datatype } => {
                    OxLiteral::new_typed_literal(value, cnv_named_node(datatype))
                }
            }
        }
        fn cnv_object(s: Term) -> OxTerm {
            match s {
                Term::NamedNode(n) => {
                    OxTerm::NamedNode(OxNamedNode::new_unchecked(n.iri.to_string()))
                }
                Term::BlankNode(b) => OxTerm::BlankNode(OxBlankNode::new_unchecked(b.id)),
                Term::Literal(l) => OxTerm::Literal(cnv_literal(l)),
                Term::Triple(_) => todo!(),
            }
        }

        fn cnv(t: Triple) -> OxTriple {
            OxTriple::new(
                cnv_subject(t.subject),
                cnv_named_node(t.predicate),
                cnv_object(t.object),
            )
        }

        let mut graph = Graph::default();
        let s = r#"PREFIX : <http://example.org/>
           :alice :name "Alice" ;
                  :knows [ :name "Bob" ], _:1 .
           _:1    :name "Carol" . 
         "#;
        let mut count = 0;
        let mut parser = TurtleParser::new(std::io::Cursor::new(&s), None);

        let res = parser.parse_all(&mut |triple| {
            count += 1;
            let t = cnv(triple);
            graph.insert(t.as_ref());
            Ok(()) as Result<(), TurtleError>
        });
        assert!(res.is_ok());
        assert_eq!(graph.len(), 5);
        let alice = OxNamedNode::new_unchecked("http://example.org/alice");
        assert_eq!(graph.triples_for_subject(alice.as_ref()).count(), 3);
        assert_eq!(count, 5)
    }
}
