use oxrdf::BlankNode as OxBlankNode;
use oxrdf::Literal as OxLiteral;
use oxrdf::NamedNode as OxIri;
use oxrdf::Subject as OxSubject;
use oxrdf::Term as OxTerm;
use oxrdf::Triple as OxTriple;

use crate::model::Object;
use crate::model::Predicate;
use crate::model::Subject;
use crate::model::Triple;

pub mod error;
pub mod graph;
pub mod parser;
pub mod serializer;

impl Triple for OxTriple {
    type Subject = OxSubject;
    type Iri = OxIri;
    type Term = OxTerm;

    fn new(subj: Self::Subject, pred: Self::Iri, obj: Self::Term) -> Self {
        OxTriple::new(subj, pred, obj)
    }

    fn subj(&self) -> &Self::Subject {
        &self.subject
    }

    fn pred(&self) -> &Self::Iri {
        &self.predicate
    }

    fn obj(&self) -> &Self::Term {
        &self.object
    }
}

impl Subject for OxSubject {
    type BlankNode = OxBlankNode;
    type Iri = OxIri;
    type Triple = OxTriple;

    fn is_blank_node(&self) -> bool {
        self.is_blank_node()
    }

    fn is_iri(&self) -> bool {
        self.is_named_node()
    }

    fn is_triple(&self) -> bool {
        self.is_triple()
    }

    fn as_blank_node(&self) -> Option<&OxBlankNode> {
        match self {
            OxSubject::NamedNode(_) => None,
            OxSubject::BlankNode(blank_node) => Some(blank_node),
            OxSubject::Triple(_) => None,
        }
    }

    fn as_iri(&self) -> Option<&Self::Iri> {
        match self {
            OxSubject::NamedNode(named_node) => Some(named_node),
            OxSubject::BlankNode(_) => None,
            OxSubject::Triple(_) => None,
        }
    }

    fn as_triple(&self) -> Option<&OxTriple> {
        match self {
            OxSubject::NamedNode(_) => None,
            OxSubject::BlankNode(_) => None,
            OxSubject::Triple(triple) => Some(&triple),
        }
    }
}

impl Predicate for OxIri {
    fn new(str: &str) -> Self {
        OxIri::new_unchecked(str)
    }
}

impl Object for OxTerm {
    type BlankNode = OxBlankNode;
    type Iri = OxIri;
    type Literal = OxLiteral;
    type Triple = OxTriple;

    fn is_blank_node(&self) -> bool {
        self.is_blank_node()
    }

    fn is_iri(&self) -> bool {
        self.is_named_node()
    }

    fn is_literal(&self) -> bool {
        self.is_literal()
    }

    fn is_triple(&self) -> bool {
        self.is_triple()
    }

    fn as_blank_node(&self) -> Option<&OxBlankNode> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(blank_node) => Some(blank_node),
            OxTerm::Triple(_) => None,
            OxTerm::Literal(_) => None,
        }
    }

    fn as_iri(&self) -> Option<&Self::Iri> {
        match self {
            OxTerm::NamedNode(named_node) => Some(named_node),
            OxTerm::BlankNode(_) => None,
            OxTerm::Triple(_) => None,
            OxTerm::Literal(_) => None,
        }
    }

    fn as_literal(&self) -> Option<&OxLiteral> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(_) => None,
            OxTerm::Triple(_) => None,
            OxTerm::Literal(literal) => Some(literal),
        }
    }

    fn as_triple(&self) -> Option<&OxTriple> {
        match self {
            OxTerm::NamedNode(_) => None,
            OxTerm::BlankNode(_) => None,
            OxTerm::Triple(triple) => Some(&triple),
            OxTerm::Literal(_) => None,
        }
    }
}
