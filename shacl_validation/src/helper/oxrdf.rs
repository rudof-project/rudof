use iri_s::IriS;
use oxrdf::{BlankNode, Subject, Term};
use srdf::RDFNode;

use super::helper_error::HelperError;

pub(crate) fn term_to_subject(term: Term) -> Result<Subject, HelperError> {
    match term {
        Term::NamedNode(node) => Ok(Subject::NamedNode(node)),
        Term::BlankNode(node) => Ok(Subject::BlankNode(node)),
        Term::Literal(_) => Err(HelperError::ParseLiteralToSubject),
    }
}

pub(crate) fn node_to_term(node: RDFNode) -> Term {
    match node {
        srdf::Object::Iri(iri_s) => Term::NamedNode(iri_s.as_named_node().to_owned()),
        srdf::Object::BlankNode(id) => Term::BlankNode(BlankNode::new_unchecked(id)),
        srdf::Object::Literal(literal) => Term::Literal(literal.into()),
    }
}

pub(crate) fn subject_to_node(subject: Subject) -> RDFNode {
    match subject {
        Subject::NamedNode(node) => RDFNode::iri(IriS::new_unchecked(node.as_str())),
        Subject::BlankNode(node) => RDFNode::bnode(node.to_string()),
    }
}

pub(crate) fn term_to_node(term: Term) -> RDFNode {
    match term {
        Term::NamedNode(node) => RDFNode::iri(IriS::new_unchecked(node.as_str())),
        Term::BlankNode(node) => RDFNode::bnode(node.to_string()),
        Term::Literal(literal) => RDFNode::literal(literal.into()),
    }
}

pub(crate) fn node_to_subject(node: RDFNode) -> Result<Subject, HelperError> {
    match node {
        RDFNode::Iri(iri_s) => Ok(Subject::NamedNode(iri_s.as_named_node().to_owned())),
        RDFNode::BlankNode(id) => Ok(Subject::BlankNode(BlankNode::new_unchecked(id))),
        RDFNode::Literal(_) => Err(HelperError::ParseLiteralToSubject),
    }
}
